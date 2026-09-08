//! Lifecycle soak — the tripwire that makes `performance-standards.md` §0
//! enforceable (p2 **E6d/e**).
//!
//! Navigates every gallery route (the real `NAV` table) ×3 rounds, then asserts
//! every shared-registry census returns to the after-round-1 baseline. Round 1
//! warms the true statics (theme, fonts, root signals); a leak that grows across
//! rounds 2→3 fails here with its counter's name. Also exercises an overlay
//! open/close per round (E6e) and asserts the animation driver settles to idle.

use pebbles::prelude::*;

use crate::app;
use crate::state::{LANDING, NAV, navigate};

const WINDOW: Size = Size::new(1180.0, 820.0);

/// One shell-style frame: advance animations + scroll springs, reconcile, relayout.
fn frame(ui: &mut Ui, env: &mut pebbles::render::TextEnv, now: &mut f64) {
    *now += 0.016;
    pebbles::core::animation::tick(*now);
    ui.tick_scrolls(0.016);
    ui.make_current();
    ui.rebuild_if_dirty();
    ui.layout(env, WINDOW);
}

/// Pump frames until the animation driver goes idle (bounded, so a legitimately
/// looping screen — spinner, shimmer — doesn't hang the test).
fn settle(ui: &mut Ui, env: &mut pebbles::render::TextEnv, now: &mut f64) {
    for _ in 0..30 {
        frame(ui, env, now);
        if !pebbles::core::animation::active() {
            break;
        }
    }
}

/// Hop to `route` and let it settle (the previous screen unmounts here).
fn hop(route: &str, ui: &mut Ui, env: &mut pebbles::render::TextEnv, now: &mut f64) {
    navigate(route);
    settle(ui, env, now);
}

/// The full shared-registry census (`performance-standards.md` E6c).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Census {
    elements: usize,
    render_nodes: usize,
    signals: usize,
    memos: usize,
    subscriptions: usize,
    cleanups: usize,
    pending: usize,
    focus: usize,
    loops: usize,
    timeouts: usize,
    scroll_handlers: usize,
    scroll_metrics: usize,
    text_edit: usize,
    overlays: usize,
    passive: usize,
}

fn census(ui: &Ui) -> Census {
    Census {
        elements: ui.element_count(),
        render_nodes: ui.render_node_count(),
        signals: pebbles::core::census_signals(),
        memos: pebbles::core::census_memos(),
        subscriptions: pebbles::core::census_subscriptions(),
        cleanups: pebbles::core::census_cleanups(),
        pending: pebbles::core::census_pending(),
        focus: pebbles::core::census_registrations(),
        loops: pebbles::core::census_loops(),
        timeouts: pebbles::core::census_timeouts(),
        scroll_handlers: pebbles::core::census_handlers(),
        scroll_metrics: pebbles::render::scroll_metrics::len(),
        text_edit: pebbles::render::text_edit::len(),
        overlays: pebbles::widgets::overlay::census_overlays(),
        passive: pebbles::widgets::overlay::census_passive(),
    }
}

/// E6e: open + close one menu overlay and one passive layer, so the service
/// registries are exercised (not just pages).
fn exercise_overlays(ui: &mut Ui, env: &mut pebbles::render::TextEnv, now: &mut f64) {
    ui.make_current();
    show_overlay(text("menu").into_widget(), 10.0, 10.0, 200.0, 120.0);
    frame(ui, env, now);
    hide_overlay();
    show_passive(text("tip").into_widget(), 10.0, 10.0);
    frame(ui, env, now);
    hide_passive();
    frame(ui, env, now);
}

#[test]
fn navigation_soak_returns_to_baseline() {
    Theme::light().make_current();
    crate::state::init();
    pebbles::widgets::overlay::init();
    pebbles::core::focus::init();
    dialog::init();
    pebbles::core::animation::reset();

    let mut ui = Ui::new();
    let mut env = pebbles::render::TextEnv::new();
    ui.make_current();
    ui.mount_root(View::new(theme().colors.background, component(app::app)).into_widget());
    ui.layout(&mut env, WINDOW);

    let routes: Vec<&str> = NAV.iter().flat_map(|g| g.routes.iter().map(|(r, _, _)| *r)).collect();
    let mut now = 0.0_f64;

    // Round 1 warms the true statics; the baseline is taken AFTER it.
    for r in &routes {
        hop(r, &mut ui, &mut env, &mut now);
    }
    hop("overview", &mut ui, &mut env, &mut now);
    exercise_overlays(&mut ui, &mut env, &mut now);
    let baseline = census(&ui);

    for round in 2..=3 {
        for r in &routes {
            hop(r, &mut ui, &mut env, &mut now);
        }
        hop("overview", &mut ui, &mut env, &mut now);
        exercise_overlays(&mut ui, &mut env, &mut now);
        let after = census(&ui);
        assert_eq!(after, baseline, "round {round}: a shared registry grew vs the round-1 baseline");
    }

    // No screen may leave the frame driver running once back on Overview.
    assert!(!pebbles::core::animation::active(), "animation driver never settled after the soak");
}

/// Hovering the hero nav links must NOT churn the landing page's reactive scope.
/// Regression guard for the bug where per-link hover state lived in a plain helper
/// (not a component), so every hover rebuilt the whole landing page and froze
/// input/scroll. Each hover must settle within the bounded frame budget and leave
/// every shared registry at its pre-hover baseline.
#[test]
fn hero_nav_hover_is_stable() {
    Theme::light().make_current();
    crate::state::init();
    pebbles::widgets::overlay::init();
    pebbles::core::focus::init();
    dialog::init();
    pebbles::core::animation::reset();
    // Wide viewport so the hero renders the full link bar, not the compact menu.
    pebbles::widgets::overlay::set_window_size(1180.0, 820.0);

    let mut ui = Ui::new();
    let mut env = pebbles::render::TextEnv::new();
    ui.make_current();
    ui.mount_root(View::new(theme().colors.background, component(app::app)).into_widget());
    navigate(LANDING);
    let mut now = 0.0_f64;
    settle(&mut ui, &mut env, &mut now);

    // The links sit in the top strip, right of center. Confirm at least one is
    // hit-testable so the sweep below is actually exercising the wide bar.
    let link_xs = [545.0, 615.0, 712.0, 828.0, 922.0];
    let hit = link_xs.iter().any(|&x| ui.tap_target_at(Offset::new(x, 56.0)).is_some());
    assert!(hit, "no hero nav link was hit-testable — bar layout / coordinates drifted");

    let baseline = census(&ui);

    // Sweep hover across each link and back off, many cycles. An unbounded rebuild
    // loop would hang `rebuild_if_dirty` inside `settle` and never return.
    for _ in 0..8 {
        for &x in &link_xs {
            let _ = ui.dispatch_hover(Offset::new(x, 56.0));
            settle(&mut ui, &mut env, &mut now);
        }
        let _ = ui.dispatch_hover(Offset::new(1170.0, 780.0)); // off every link
        settle(&mut ui, &mut env, &mut now);
    }

    let after = census(&ui);
    assert_eq!(after, baseline, "hovering the hero nav links churned a shared registry (leak / rebuild storm)");
    assert!(!pebbles::core::animation::active(), "hover left the frame driver running");
}
