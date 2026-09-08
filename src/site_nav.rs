//! The site's single top navigation bar — identical on the landing page, the
//! component index, and every widget screen (used as the scaffold's `top`). Keeping
//! one bar everywhere is what makes the three surfaces feel like one site.

use pebbles::prelude::*;

use crate::state::{to_docs, to_landing};
use crate::ui::{brand_gradient, gap_w};

/// A muted text link in the nav.
fn nav_link(label: &str, on: impl Fn() + 'static) -> impl IntoWidget {
    let c = theme().colors;
    pressable(
        container()
            .padding(EdgeInsets::symmetric(10.0, 6.0))
            .child(text(label.to_string()).size(13.5).weight(500.0).color(c.muted_foreground)),
    )
    .radius(8.0)
    .on_tap(on)
}

/// The brand mark (gradient gem + wordmark) — clicking it returns to the landing page.
fn brand_mark() -> impl IntoWidget {
    let c = theme().colors;
    pressable(
        row(children![
            container()
                .decoration(BoxDecoration::new().gradient(brand_gradient()).radius(BorderRadius::all(9.0)))
                .padding(EdgeInsets::all(6.0))
                .child(icon(lucide::GEM).size(17.0).color(palette::WHITE)),
            gap_w(10.0),
            text("Pebbles").size(18.0).bold().color(c.foreground),
        ])
        .main_axis_size(MainAxisSize::Min)
        .cross_axis_alignment(CrossAxisAlignment::Center),
    )
    .radius(9.0)
    .on_tap(to_landing)
}

/// The shared top navigation bar (brand, links, theme toggle, primary CTA) with a
/// hairline rule beneath it. Full-bleed — the caller places it at the top of a
/// stretched column or hands it to `scaffold(..).top(top_nav())`.
pub fn top_nav() -> impl IntoWidget {
    let c = theme().colors;
    let dark = theme().dark;
    let theme_toggle = icon_button(if dark { lucide::SUN } else { lucide::MOON }).on_pressed(toggle_theme);

    let bar = container().padding(EdgeInsets::symmetric(26.0, 13.0)).child(
        row(children![
            brand_mark(),
            spacer(),
            row(children![
                nav_link("Components", to_docs),
                gap_w(2.0),
                nav_link("GitHub", || eprintln!("open https://github.com/pebbles-hq/pebbles")),
                gap_w(12.0),
                theme_toggle,
                gap_w(6.0),
                button("Get started").size(ButtonSize::Sm).trailing(lucide::ARROW_RIGHT).on_pressed(to_docs),
            ])
            .main_axis_size(MainAxisSize::Min)
            .cross_axis_alignment(CrossAxisAlignment::Center),
        ])
        .cross_axis_alignment(CrossAxisAlignment::Center),
    );

    column(children![bar, container().height(1.0).decoration(BoxDecoration::new().color(c.border))])
        .cross_axis_alignment(CrossAxisAlignment::Stretch)
        .main_axis_size(MainAxisSize::Min)
}
