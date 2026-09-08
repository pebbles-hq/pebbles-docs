//! The **docs hub** — a documentation site with a left sidenav of learning sections
//! (introduction → setup → core concepts → building UIs) that ends in the **Widgets**
//! and **Components** catalog, plus a scrolling content pane for the active section.
//!
//! State: a `section` signal (which doc page is open) and a `query` signal (the
//! catalog search). The hub reads `section`; the filtered catalog grid is a separate
//! child component ([`render_grid`]) that reads `query`, so a keystroke re-renders
//! only the grid and the search field keeps focus.

use pebbles::prelude::*;

use crate::state::{NAV, navigate};
use crate::ui::{brand, gap_h, gap_w};

// ---------------------------------------------------------------------------
// Section registry
// ---------------------------------------------------------------------------

/// A labelled group of doc/lesson sections in the sidenav (shared by the Learn hub).
pub(crate) struct DocGroup {
    pub label: &'static str,
    pub items: &'static [(&'static str, &'static str)], // (section id, label)
}

/// The docs sidenav — standard "learn the framework" sections, with the Widgets and
/// Components catalog as the last two entries.
const DOCS: &[DocGroup] = &[
    DocGroup {
        label: "GET STARTED",
        items: &[
            ("introduction", "Introduction"),
            ("installation", "Installation"),
            ("quick-start", "Quick Start"),
            ("project-layout", "Project Layout"),
        ],
    },
    DocGroup {
        label: "CORE CONCEPTS",
        items: &[
            ("components", "Components"),
            ("state", "State Management"),
            ("effects", "Effects & Memos"),
            ("events", "Events"),
            ("context", "Context"),
        ],
    },
    DocGroup {
        label: "BUILDING UIS",
        items: &[
            ("layout", "Layout"),
            ("styling", "Styling"),
            ("theming", "Theming"),
            ("routing", "Routing"),
            ("motion", "Motion & Animation"),
            ("async", "Async & Data"),
            ("platforms", "Platforms"),
        ],
    },
    DocGroup {
        label: "CATALOG",
        items: &[("widgets", "Widgets"), ("catalog", "Components")],
    },
];

// ---------------------------------------------------------------------------
// Sidenav
// ---------------------------------------------------------------------------

pub(crate) fn nav_row(id: &'static str, label: &'static str, active: bool, section: Signal<String>) -> impl IntoWidget {
    let c = theme().colors;
    let (fg, deco) = if active {
        (
            brand::BROWN,
            BoxDecoration::new().color(c.secondary).radius(BorderRadius::all(8.0)),
        )
    } else {
        (c.muted_foreground, BoxDecoration::new())
    };
    pressable(
        container()
            .decoration(deco)
            .padding(EdgeInsets::symmetric(10.0, 7.0))
            .child(text(label.to_string()).size(13.5).weight(if active { 600.0 } else { 500.0 }).color(fg)),
    )
    .radius(8.0)
    .on_tap(move || section.set(id.to_string()))
}

pub(crate) fn sidenav(groups: &'static [DocGroup], active: &str, section: Signal<String>) -> impl IntoWidget {
    let c = theme().colors;
    let mut items: Vec<AnyWidget> = Vec::new();
    for (gi, group) in groups.iter().enumerate() {
        if gi > 0 {
            items.push(gap_h(16.0).into_widget());
        }
        items.push(
            padding(
                EdgeInsets::symmetric(10.0, 4.0),
                text(group.label.to_string()).size(11.0).weight(700.0).color(c.muted_foreground),
            )
            .into_widget(),
        );
        for (id, label) in group.items {
            items.push(nav_row(id, label, active == *id, section).into_widget());
            items.push(gap_h(2.0).into_widget());
        }
    }
    container()
        .width(248.0)
        .decoration(BoxDecoration::new().color(c.background).border(Border::new(c.border, 1.0)))
        .child(scroll_view(
            container().padding(EdgeInsets::all(16.0)).child(
                column(items).cross_axis_alignment(CrossAxisAlignment::Stretch).main_axis_size(MainAxisSize::Min),
            ),
        ))
}

// ---------------------------------------------------------------------------
// Catalog (Widgets / Components) — the searchable grid
// ---------------------------------------------------------------------------

/// Which NAV categories belong under "Widgets" (layout/primitive/foundational) vs
/// "Components" (the shadcn-style UI set). Everything non-GET-STARTED is one or the other.
fn is_widget_group(label: &str) -> bool {
    matches!(
        label,
        "LAYOUT"
            | "LAYOUT (LONG-TAIL)"
            | "MOTION & TRANSITIONS"
            | "DRAG & DROP / POINTER"
            | "PAINTING & EFFECTS"
            | "FOUNDATIONS"
    )
}

/// Props for the reactive grid child: the search query + which half of the catalog.
struct GridProps {
    query: Signal<String>,
    widgets: bool,
}

/// One clickable component card (icon + label) that opens that widget's screen.
fn widget_card(route: &'static str, ic: IconData, label: &'static str) -> impl IntoWidget {
    let c = theme().colors;
    pressable(
        container()
            .width(188.0)
            .decoration(
                BoxDecoration::new()
                    .color(c.card)
                    .radius(BorderRadius::all(12.0))
                    .border(Border::new(c.border, 1.0)),
            )
            .padding(EdgeInsets::all(13.0))
            .child(
                row(children![
                    container()
                        .decoration(BoxDecoration::new().color(c.secondary).radius(BorderRadius::all(9.0)))
                        .padding(EdgeInsets::all(8.0))
                        .child(icon(ic).size(16.0).color(brand::BROWN)),
                    gap_w(11.0),
                    expanded(text(label.to_string()).size(13.5).weight(600.0).color(c.foreground)),
                ])
                .cross_axis_alignment(CrossAxisAlignment::Center),
            ),
    )
    .radius(12.0)
    .label(label)
    .on_tap(move || navigate(route))
}

/// The reactive, filtered grid — re-renders on every keystroke (it reads the query),
/// while the search field in the parent stays put.
fn render_grid(p: &GridProps) -> Column {
    let c = theme().colors;
    let raw = p.query.get().to_lowercase();
    let q = raw.trim();

    let mut items: Vec<AnyWidget> = Vec::new();
    let mut total = 0usize;

    for group in NAV {
        if group.label == "GET STARTED" || is_widget_group(group.label) != p.widgets {
            continue;
        }
        let cards: Vec<AnyWidget> = group
            .routes
            .iter()
            .filter(|(_, _, label)| {
                q.is_empty() || label.to_lowercase().contains(q) || group.label.to_lowercase().contains(q)
            })
            .map(|(r, ic, label)| widget_card(r, *ic, label).into_widget())
            .collect();
        if cards.is_empty() {
            continue;
        }
        total += cards.len();

        items.push(
            row(children![
                text(group.label.to_string()).size(12.0).weight(700.0).color(c.muted_foreground),
                gap_w(8.0),
                container()
                    .decoration(BoxDecoration::new().color(c.muted).radius(BorderRadius::all(999.0)))
                    .padding(EdgeInsets::symmetric(7.0, 1.0))
                    .child(text(cards.len().to_string()).size(10.5).weight(600.0).color(c.muted_foreground)),
            ])
            .main_axis_size(MainAxisSize::Min)
            .cross_axis_alignment(CrossAxisAlignment::Center)
            .into_widget(),
        );
        items.push(gap_h(14.0).into_widget());
        items.push(wrap(cards).spacing(12.0).run_spacing(12.0).into_widget());
        items.push(gap_h(30.0).into_widget());
    }

    if total == 0 {
        items.push(
            container()
                .decoration(
                    BoxDecoration::new()
                        .color(c.card)
                        .radius(BorderRadius::all(14.0))
                        .border(Border::new(c.border, 1.0)),
                )
                .padding(EdgeInsets::all(40.0))
                .child(
                    column(children![
                        icon(lucide::SEARCH_X).size(26.0).color(c.muted_foreground),
                        gap_h(12.0),
                        text(format!("No matches for “{}”", p.query.get()))
                            .size(15.0)
                            .weight(600.0)
                            .color(c.foreground),
                    ])
                    .cross_axis_alignment(CrossAxisAlignment::Start)
                    .main_axis_size(MainAxisSize::Min),
                )
                .into_widget(),
        );
    }

    column(items).cross_axis_alignment(CrossAxisAlignment::Start).main_axis_size(MainAxisSize::Min)
}

fn catalog(title: &str, blurb: &str, query: Signal<String>, widgets: bool) -> AnyWidget {
    let c = theme().colors;
    let search =
        text_field().leading(lucide::SEARCH).placeholder("Search…").width(560.0).bind(query);
    column(children![
        text(title.to_string()).size(30.0).bold().color(c.foreground),
        gap_h(6.0),
        text(blurb.to_string()).size(15.0).line_height(1.5).color(c.muted_foreground),
        gap_h(20.0),
        search,
        gap_h(30.0),
        component_props(render_grid, GridProps { query, widgets }),
    ])
    .cross_axis_alignment(CrossAxisAlignment::Start)
    .main_axis_size(MainAxisSize::Min)
    .into_widget()
}

// ---------------------------------------------------------------------------
// Prose content — building blocks
// ---------------------------------------------------------------------------

pub(crate) fn h2(s: &str) -> AnyWidget {
    let c = theme().colors;
    column(children![gap_h(10.0), text(s.to_string()).size(20.0).bold().color(c.foreground), gap_h(8.0)])
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .main_axis_size(MainAxisSize::Min)
        .into_widget()
}

pub(crate) fn p(s: &str) -> AnyWidget {
    let c = theme().colors;
    column(children![text(s.to_string()).size(15.0).line_height(1.65).color(c.muted_foreground), gap_h(14.0)])
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .main_axis_size(MainAxisSize::Min)
        .into_widget()
}

pub(crate) fn code(src: &str) -> AnyWidget {
    let ink = Color::from_rgba8(0x0D, 0x11, 0x1B, 0xFF);
    let fg = Color::from_rgba8(0xE6, 0xE9, 0xF2, 0xFF);
    let lines: Vec<AnyWidget> = src
        .lines()
        .map(|l| text(l.to_string()).font_family("JetBrains Mono").size(13.0).line_height(1.7).color(fg).into_widget())
        .collect();
    column(children![
        container()
            .decoration(BoxDecoration::new().color(ink).radius(BorderRadius::all(12.0)))
            .padding(EdgeInsets::all(18.0))
            .child(
                column(lines).cross_axis_alignment(CrossAxisAlignment::Start).main_axis_size(MainAxisSize::Min),
            ),
        gap_h(18.0),
    ])
    .cross_axis_alignment(CrossAxisAlignment::Stretch)
    .main_axis_size(MainAxisSize::Min)
    .into_widget()
}

pub(crate) fn prose(title: &str, subtitle: &str, blocks: Vec<AnyWidget>) -> AnyWidget {
    let c = theme().colors;
    let mut items: Vec<AnyWidget> = vec![
        text(title.to_string()).size(32.0).bold().color(c.foreground).into_widget(),
        gap_h(8.0).into_widget(),
        text(subtitle.to_string()).size(16.5).line_height(1.55).color(c.muted_foreground).into_widget(),
        gap_h(24.0).into_widget(),
    ];
    items.extend(blocks);
    column(items).cross_axis_alignment(CrossAxisAlignment::Start).main_axis_size(MainAxisSize::Min).into_widget()
}

// ---------------------------------------------------------------------------
// Prose content — the pages
// ---------------------------------------------------------------------------

fn page(id: &str) -> AnyWidget {
    match id {
        "introduction" => prose(
            "Introduction",
            "Pebbles is a Flutter-style, GPU-accelerated GUI framework for Rust — declarative widgets, fine-grained reactivity, and one codebase for desktop, web, and mobile.",
            vec![
                p("You describe your UI as a tree of widgets returned from plain functions. State lives in signals; when a signal changes, only the parts of the tree that read it re-render. Every frame is rasterized on the GPU through Vello, so text, gradients, blurs, and clips stay crisp and smooth at any size."),
                h2("Why Pebbles"),
                p("If you know Flutter or SolidJS, you already know Pebbles: components are functions, state is a signal, and events are closures. There is no virtual DOM, no macro DSL to learn, and no garbage collector — just Rust and `cargo run`."),
                h2("The loop"),
                p("Build a widget tree, hand the root to `App`, and run it. The rest of these docs walk through the pieces: installing the toolchain, the reactivity model, and the widget + component catalog."),
            ],
        ),
        "installation" => prose(
            "Installation",
            "Add Pebbles to a Cargo project and install the small toolchain the web build needs.",
            vec![
                h2("Add the dependency"),
                p("Pebbles tracks its main branch by git. Add it to your Cargo.toml:"),
                code("[dependencies]\npebbles = { git = \"https://github.com/pebbles-hq/pebbles\" }"),
                h2("Optional features"),
                p("Opt into extras as you need them — for example image loading and native file dialogs:"),
                code("pebbles = { git = \"…\", features = [\"image-view\", \"file-dialogs\"] }"),
                h2("The CLI (optional)"),
                p("The pebbles CLI scaffolds projects and runs the web build. Install it once:"),
                code("cargo install --git https://github.com/pebbles-hq/pebbles pebbles-cli\ncargo install --locked trunk   # for the web target"),
            ],
        ),
        "quick-start" => prose(
            "Quick Start",
            "A complete counter app — function component, a signal, and a closure event.",
            vec![
                code("use pebbles::prelude::*;\n\nfn counter() -> impl IntoWidget {\n    let count = create_signal(0);\n    center(column(children![\n        text(count.get().to_string()).size(48.0),\n        button(\"+\").on_pressed(move || count.update(|n| *n += 1)),\n    ]))\n}\n\nfn main() -> Result<(), Box<dyn std::error::Error>> {\n    App::new(component(counter)).title(\"Counter\").run()\n}"),
                p("`cargo run` opens a native window. To run it in a WebGPU browser instead:"),
                code("pebbles run -d web"),
            ],
        ),
        "project-layout" => prose(
            "Project Layout",
            "How a typical Pebbles app is organized.",
            vec![
                p("A small app is a single main.rs. As it grows, the common split is one module per screen, a state module for global signals, and a styles/ui module for shared helpers — exactly how this gallery is laid out."),
                code("src/\n  main.rs      // App::new(...).run()\n  app.rs       // the root component + routing\n  state.rs     // global signals (created at app scope)\n  screens/     // one component per screen\n  ui.rs        // shared helpers (cards, sections)"),
                p("`pebbles create <name>` scaffolds this for you, wired to the framework by git."),
            ],
        ),
        "components" => prose(
            "Components",
            "A component is a plain function that returns a widget tree. No structs, no traits, no macros.",
            vec![
                code("fn greeting(name: &str) -> impl IntoWidget {\n    text(format!(\"Hello, {name}\"))\n}"),
                p("Mount a function as a reactive component with `component(func)`; it re-runs only when a signal it reads changes. Signals created in the body persist across those re-renders, so local state is safe."),
                code("fn panel() -> impl IntoWidget {\n    let open = create_signal(false);\n    column(children![\n        button(\"Toggle\").on_pressed(move || open.update(|o| *o = !*o)),\n        if open.get() { text(\"Now you see me\").into_widget() } else { gap_h(0.0).into_widget() },\n    ])\n}"),
                h2("Props"),
                p("For reusable, parameterized widgets, pass data with `component_props(func, props)` where `func` is `fn(&Props) -> impl IntoWidget`."),
            ],
        ),
        "state" => prose(
            "State Management",
            "One primitive — `create_signal` — powers both local and global state.",
            vec![
                p("A signal holds a value. `.get()` reads it (and subscribes the calling component), `.set(v)` replaces it, and `.update(|v| …)` mutates it in place. Any component that read the signal re-renders when it changes — nothing else does."),
                code("let count = create_signal(0);\ncount.get();               // read + subscribe\ncount.set(10);             // replace\ncount.update(|n| *n += 1); // mutate"),
                h2("Global state"),
                p("Global state is the *same* primitive, created once at app scope (before any component runs) so it is shared rather than owned by one component. Read and write it from anywhere — no prop-drilling. For larger shapes, `create_store` gives fine-grained field-level updates."),
            ],
        ),
        "effects" => prose(
            "Effects & Memos",
            "Run side effects when signals change, and derive cached values from them.",
            vec![
                h2("create_effect"),
                p("An effect re-runs whenever any signal it reads changes — for logging, persistence, or syncing to the outside world."),
                code("create_effect(move || {\n    println!(\"count is now {}\", count.get());\n});"),
                h2("create_memo"),
                p("A memo is a derived signal: it recomputes only when its inputs change and caches the result, so expensive derivations run once per change instead of once per read."),
                code("let doubled = create_memo(move || count.get() * 2);\ntext(doubled.get().to_string());"),
            ],
        ),
        "events" => prose(
            "Events",
            "Events are plain closures — no synthetic event system.",
            vec![
                p("Interactive widgets take callbacks directly. A button's `on_pressed` is just a `Fn`, usually a `move` closure that writes a signal."),
                code("button(\"Save\").on_pressed(move || status.set(\"saved\".into()));"),
                h2("Gestures"),
                p("Wrap anything in a `gesture_detector` (or `pressable` for a tappable region with hover/press feedback) to handle taps, double-taps, long-press, drag, and pointer events."),
                code("pressable(my_card).on_tap(move || selected.set(id));"),
            ],
        ),
        "context" => prose(
            "Context",
            "Pass values down the tree without threading them through every component.",
            vec![
                p("Provide a value with `provide_context(value)` and read the nearest one with `consume_context::<T>()`. It's the idiomatic way to share things like the current user, a theme override, or a service handle."),
                code("provide_context(Session { user: \"ada\".into() });\n// deeper in the tree:\nlet session = consume_context::<Session>();"),
            ],
        ),
        "layout" => prose(
            "Layout",
            "Compose UIs from flex and box widgets — the same model as Flutter.",
            vec![
                p("`row` and `column` lay children along an axis; `expanded` and `spacer` distribute leftover space; `stack` + `positioned` overlay children; `container` adds padding, sizing, and decoration. Alignment is controlled with `main_axis_alignment` / `cross_axis_alignment`."),
                code("row(children![\n    icon(lucide::USER),\n    gap_w(8.0),\n    expanded(text(\"Name\")),\n    button(\"Edit\"),\n])\n.cross_axis_alignment(CrossAxisAlignment::Center)"),
                p("See the Widgets catalog for the full set — wraps, grids, scroll views, resizable and split panes, and more."),
            ],
        ),
        "styling" => prose(
            "Styling",
            "An RN/CSS-like style system you can apply anywhere.",
            vec![
                p("`style()` builds a reusable `Style` — background, padding, radius, border, shadow, text properties — that you apply with `.styled(..)` on any widget or `text(..).style(..)`. Keep styles in a module and compose them with `.merge(..)`."),
                code("let card = style()\n    .background(theme().colors.card)\n    .padding_all(16.0)\n    .radius_all(12.0)\n    .border(Border::new(theme().colors.border, 1.0));\n\nmy_content.styled(card)"),
            ],
        ),
        "theming" => prose(
            "Theming",
            "A shadcn-flavored theme with first-class light and dark modes.",
            vec![
                p("`theme()` returns the active theme; `theme().colors` gives semantic roles (background, foreground, card, primary, muted, border…). Set the theme once at startup and flip it live at any time — every component that read `theme()` re-renders."),
                code("Theme::light().make_current();     // at startup\n\nicon_button(lucide::MOON).on_pressed(toggle_theme);  // flip live"),
            ],
        ),
        "routing" => prose(
            "Routing",
            "Built-in, signal-driven routing — no router crate required.",
            vec![
                p("Hold the current route in a global signal and map routes to components with `route_view`. Navigation is just a signal write, so it composes with everything else."),
                code("route_view(current.clone())\n    .route(\"home\", || component(home))\n    .route(\"settings\", || component(settings))\n    .fallback(|| component(not_found))"),
            ],
        ),
        "motion" => prose(
            "Motion & Animation",
            "Implicit and explicit animation, with springs and curves.",
            vec![
                p("Implicit widgets like `animated_container` tween automatically when their properties change. For explicit control, drive a value with `animate_to` / `animate_spring`, or use the transition widgets (fade, scale, slide, size) and hero flights."),
                code("animate_to(offset, 120.0, 0.3);   // to 120 over 0.3s\nanimated_container().width(w.get()).height(64.0)"),
            ],
        ),
        "async" => prose(
            "Async & Data",
            "Load data off the UI thread and stream it back into signals.",
            vec![
                p("`spawn` runs a future and delivers its result back on the UI thread. `create_resource` wraps an async source as a signal with loading/error states, and `stream_builder` renders a widget per value from a channel."),
                code("let user = create_resource(move || fetch_user(id.get()));\nmatch user.get() {\n    Some(u) => text(u.name),\n    None => text(\"Loading…\"),\n}"),
            ],
        ),
        "platforms" => prose(
            "Platforms",
            "One widget tree, three targets: desktop, web, and mobile.",
            vec![
                p("The same code runs natively (winit + wgpu), on the web (WebGPU via wasm), and on mobile shells. Build for the web with the CLI; branch on the target with the compile-time platform API when you need to."),
                code("pebbles run -d web        // serve the wasm build\n\nif platform::is_web() { /* web-only path */ }"),
                p("Pebbles is WebGPU-only on the web — it needs Chrome/Edge, Safari 26+, or Firefox with WebGPU enabled."),
            ],
        ),
        _ => prose("Not found", "That section doesn't exist yet.", vec![]),
    }
}

// ---------------------------------------------------------------------------
// The hub
// ---------------------------------------------------------------------------

/// The docs page: shared top nav, a learning sidenav, and a content pane whose last
/// two sidenav entries (Widgets, Components) show the searchable catalog.
pub fn docs() -> Element {
    // Global so the top nav can open the hub at a specific section (Docs → Introduction,
    // Components → the catalog).
    let section = crate::state::doc_section();
    let query = create_signal(String::new());
    let active = section.get();

    let content: AnyWidget = match active.as_str() {
        "catalog" => catalog(
            "Components",
            "The shadcn-flavored UI set — inputs, feedback, navigation, overlays. Pick one to open it live.",
            query,
            false,
        ),
        "widgets" => catalog(
            "Widgets",
            "The layout, motion, painting, and foundational widgets Pebbles is built from.",
            query,
            true,
        ),
        id => page(id),
    };

    let content_pane = expanded(scroll_view(
        container()
            .padding(EdgeInsets::symmetric(40.0, 34.0))
            .child(container().constraints(BoxConstraints::loose(Size::new(820.0, f64::INFINITY))).child(content)),
    ));

    column(children![
        crate::site_nav::top_nav(),
        expanded(
            row(children![sidenav(DOCS, &active, section), content_pane])
                .cross_axis_alignment(CrossAxisAlignment::Stretch),
        ),
    ])
    .cross_axis_alignment(CrossAxisAlignment::Stretch)
    .main_axis_size(MainAxisSize::Max)
    .into_widget()
}
