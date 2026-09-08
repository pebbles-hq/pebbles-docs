//! The site's single top navigation bar — identical on the landing page, docs, learn,
//! showcase, and every widget screen. Responsive: below [`crate::ui::COMPACT_W`] the
//! links collapse behind a menu button (left of the logo) that opens the mobile drawer
//! ([`mobile_menu`]); each page stacks that drawer over its content.

use pebbles::prelude::*;

use crate::state::{
    close_menu, menu_open, to_components, to_docs, to_landing, to_learn, to_showcase, toggle_menu,
};
use crate::ui::{brand, gap_w, is_compact, logo_mark};

/// The nav destinations, shared by the wide bar and the compact drawer so they never
/// drift apart. `(label, action)`.
fn targets() -> [(&'static str, fn()); 5] {
    [
        ("Learn", to_learn),
        ("Docs", to_docs),
        ("Components", to_components),
        ("Showcase", to_showcase),
        ("GitHub", open_github),
    ]
}

fn open_github() {
    eprintln!("open https://github.com/pebbles-hq/pebbles");
}

/// A horizontal text link (wide bar). Muted at rest; on hover the label brightens
/// to the full foreground and a warm brand-tinted pill fades in behind it, so the
/// pointer target reads clearly instead of sitting flat.
fn nav_link(label: &'static str, on: fn(), px: f32) -> impl IntoWidget {
    let c = theme().colors;
    let hovered = create_signal(false);
    let color = if hovered.get() { c.foreground } else { c.muted_foreground };
    pressable(
        container()
            .padding(EdgeInsets::symmetric(f64::from(px) * 0.85, f64::from(px) * 0.5))
            .child(text(label.to_string()).size(px).weight(500.0).color(color)),
    )
    .radius(9.0)
    .hover_tint(brand::BROWN)
    .on_hover(move |h| hovered.set(h))
    .on_tap(on)
}

/// The brand mark (mascot logo + wordmark), sized for the bar — clicking it returns to
/// the landing page.
fn brand_mark(logo_px: f64, text_px: f32) -> impl IntoWidget {
    let c = theme().colors;
    pressable(
        row(children![
            logo_mark(logo_px),
            gap_w(if logo_px >= 40.0 { 12.0 } else { 9.0 }),
            text("Pebbles").size(text_px).bold().color(c.foreground),
        ])
        .main_axis_size(MainAxisSize::Min)
        .cross_axis_alignment(CrossAxisAlignment::Center),
    )
    .radius(9.0)
    .on_tap(to_landing)
}

fn theme_toggle() -> impl IntoWidget {
    icon_button(if theme().dark { lucide::SUN } else { lucide::MOON }).on_pressed(toggle_theme)
}

/// The standard top bar (docs / learn / showcase): background from the page, a hairline
/// rule beneath it.
pub fn top_nav() -> AnyWidget {
    nav(false)
}

/// The landing hero bar: taller, a bigger logo, **transparent** (no background fill, no
/// hairline) so it blends into the hero and scrolls with the content.
pub fn hero_nav() -> AnyWidget {
    nav(true)
}

fn nav(hero: bool) -> AnyWidget {
    let c = theme().colors;
    let compact = is_compact();
    // The hero bar is bigger and chrome-less; the standard bar is compact with a rule.
    // `link_px` scales the links/button up with the hero logo so the right side keeps
    // pace with the 68px mark instead of reading as tiny.
    let (logo_px, text_px, vpad, link_px) =
        if hero { (68.0, 30.0, 22.0, 15.5) } else { (34.0, 18.0, 13.0, 13.5) };

    let bar_inner: AnyWidget = if compact {
        // Menu button (left of the logo), brand, then just the theme toggle. The links
        // live in the drawer the button opens.
        row(children![
            icon_button(lucide::MENU).on_pressed(toggle_menu),
            gap_w(4.0),
            brand_mark(logo_px, text_px),
            spacer(),
            theme_toggle(),
        ])
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .into_widget()
    } else {
        let mut links: Vec<AnyWidget> = Vec::new();
        for (label, on) in targets() {
            links.push(nav_link(label, on, link_px).into_widget());
            links.push(gap_w(if hero { 4.0 } else { 2.0 }).into_widget());
        }
        links.push(gap_w(if hero { 14.0 } else { 10.0 }).into_widget());
        links.push(theme_toggle().into_widget());
        links.push(gap_w(if hero { 10.0 } else { 6.0 }).into_widget());
        links.push(
            button("Get started")
                .size(if hero { ButtonSize::Md } else { ButtonSize::Sm })
                .trailing(lucide::ARROW_RIGHT)
                .on_pressed(to_learn)
                .into_widget(),
        );
        row(children![
            brand_mark(logo_px, text_px),
            spacer(),
            row(links).main_axis_size(MainAxisSize::Min).cross_axis_alignment(CrossAxisAlignment::Center),
        ])
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .into_widget()
    };

    let bar = container().padding(EdgeInsets::symmetric(if compact { 16.0 } else { 26.0 }, vpad)).child(bar_inner);

    if hero {
        // Transparent, no rule — the caller places it on the hero background.
        bar.into_widget()
    } else {
        column(children![bar, container().height(1.0).decoration(BoxDecoration::new().color(c.border))])
            .cross_axis_alignment(CrossAxisAlignment::Stretch)
            .main_axis_size(MainAxisSize::Min)
            .into_widget()
    }
}

/// A tappable row in the mobile drawer.
fn drawer_row(label: &'static str, on: fn()) -> impl IntoWidget {
    let c = theme().colors;
    pressable(
        container().padding(EdgeInsets::symmetric(14.0, 11.0)).child(
            text(label.to_string()).size(15.0).weight(500.0).color(c.foreground),
        ),
    )
    .radius(10.0)
    .on_tap(move || {
        close_menu();
        on();
    })
}

/// The compact navigation drawer, overlaid over the page when the viewport is compact
/// and the menu is open — otherwise nothing. `extra` is page-specific content shown
/// below the links (e.g. the docs/learn section list). Returns a full-bleed `Stack`
/// (scrim + left panel) the caller layers over its content.
pub fn mobile_menu(extra: Vec<AnyWidget>) -> AnyWidget {
    if !(is_compact() && menu_open().get()) {
        return container().into_widget();
    }
    let c = theme().colors;
    let h = media_query().size.height;

    let scrim = Positioned::fill(
        GestureDetector::new(container().color(Color::new([0.0, 0.0, 0.0, 0.45]))).on_tap(close_menu),
    );

    let mut items: Vec<AnyWidget> = Vec::new();
    for (label, on) in targets() {
        items.push(drawer_row(label, on).into_widget());
    }
    items.push(gap_h(10.0).into_widget());
    items.push(
        button("Get started").size(ButtonSize::Md).full_width().trailing(lucide::ARROW_RIGHT).on_pressed(move || {
            close_menu();
            to_learn();
        }).into_widget(),
    );
    if !extra.is_empty() {
        items.push(gap_h(14.0).into_widget());
        items.push(container().height(1.0).decoration(BoxDecoration::new().color(c.border)).into_widget());
        items.push(gap_h(14.0).into_widget());
        items.extend(extra);
    }

    let panel = Positioned::new(
        container()
            .width(288.0)
            .height(h)
            .decoration(BoxDecoration::new().color(c.background).border(Border::new(c.border, 1.0)))
            .child(scroll_view(
                container().padding(EdgeInsets::all(16.0)).child(
                    column(items).cross_axis_alignment(CrossAxisAlignment::Stretch).main_axis_size(MainAxisSize::Min),
                ),
            )),
    )
    .left(0.0)
    .top(0.0);

    stack(children![scrim, panel]).fit(StackFit::Expand).alignment(Alignment::TOP_LEFT).into_widget()
}
