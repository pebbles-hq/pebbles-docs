//! The site's single top navigation bar — identical on the landing page, docs, learn,
//! showcase, and every widget screen. Responsive: below [`crate::ui::COMPACT_W`] the
//! links collapse behind a menu button (left of the logo) that opens the mobile drawer
//! ([`mobile_menu`]); each page stacks that drawer over its content.

use pebbles::prelude::*;

use crate::state::{
    close_menu, menu_open, to_components, to_docs, to_landing, to_learn, to_showcase, toggle_menu,
};
use crate::ui::{gap_w, is_compact, logo_mark};

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

/// A muted horizontal text link (wide bar).
fn nav_link(label: &'static str, on: fn()) -> impl IntoWidget {
    let c = theme().colors;
    pressable(
        container()
            .padding(EdgeInsets::symmetric(10.0, 6.0))
            .child(text(label.to_string()).size(13.5).weight(500.0).color(c.muted_foreground)),
    )
    .radius(8.0)
    .on_tap(on)
}

/// The brand mark (mascot logo + wordmark) — clicking it returns to the landing page.
fn brand_mark() -> impl IntoWidget {
    let c = theme().colors;
    pressable(
        row(children![
            logo_mark(34.0),
            gap_w(9.0),
            text("Pebbles").size(18.0).bold().color(c.foreground),
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

/// The shared top navigation bar with a hairline rule beneath it.
pub fn top_nav() -> impl IntoWidget {
    let c = theme().colors;
    let compact = is_compact();

    let bar_inner: AnyWidget = if compact {
        // Menu button (left of the logo), brand, then just the theme toggle. The links
        // live in the drawer the button opens.
        row(children![
            icon_button(lucide::MENU).on_pressed(toggle_menu),
            gap_w(4.0),
            brand_mark(),
            spacer(),
            theme_toggle(),
        ])
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .into_widget()
    } else {
        let mut links: Vec<AnyWidget> = Vec::new();
        for (label, on) in targets() {
            links.push(nav_link(label, on).into_widget());
            links.push(gap_w(2.0).into_widget());
        }
        links.push(gap_w(10.0).into_widget());
        links.push(theme_toggle().into_widget());
        links.push(gap_w(6.0).into_widget());
        links.push(
            button("Get started").size(ButtonSize::Sm).trailing(lucide::ARROW_RIGHT).on_pressed(to_learn).into_widget(),
        );
        row(children![
            brand_mark(),
            spacer(),
            row(links).main_axis_size(MainAxisSize::Min).cross_axis_alignment(CrossAxisAlignment::Center),
        ])
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .into_widget()
    };

    let bar = container().padding(EdgeInsets::symmetric(if compact { 16.0 } else { 26.0 }, 13.0)).child(bar_inner);

    column(children![bar, container().height(1.0).decoration(BoxDecoration::new().color(c.border))])
        .cross_axis_alignment(CrossAxisAlignment::Stretch)
        .main_axis_size(MainAxisSize::Min)
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
