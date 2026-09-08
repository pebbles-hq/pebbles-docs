//! The component index — the docs "home". A search box that live-filters, then every
//! component grouped by category in a grid. Clicking a card opens that widget in the
//! sidenav + content layout (see [`crate::app`]).
//!
//! The search box lives in this (parent) component and only *binds* the query signal;
//! the filtered grid is a separate child component ([`render_grid`]) that *reads* it.
//! So a keystroke re-renders only the grid — the text field keeps focus and caret.

use pebbles::prelude::*;

use crate::state::{NAV, navigate, to_landing};
use crate::ui::{brand, gap_h, gap_w};

/// Props for the reactive grid child: the shared search query.
struct GridProps {
    query: Signal<String>,
}

/// One clickable component card (icon + label) that navigates to its widget screen.
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
                        .decoration(
                            BoxDecoration::new()
                                .color(c.secondary)
                                .radius(BorderRadius::all(9.0))
                        )
                        .padding(EdgeInsets::all(8.0))
                        .child(icon(ic).size(16.0).color(brand::INDIGO)),
                    gap_w(11.0),
                    expanded(
                        text(label.to_string())
                            .size(13.5)
                            .weight(600.0)
                            .color(c.foreground)
                    ),
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
        // The GET STARTED group is just the Overview screen — not a component.
        if group.label == "GET STARTED" {
            continue;
        }
        let cards: Vec<AnyWidget> = group
            .routes
            .iter()
            .filter(|(_, _, label)| {
                q.is_empty()
                    || label.to_lowercase().contains(q)
                    || group.label.to_lowercase().contains(q)
            })
            .map(|(r, ic, label)| widget_card(r, *ic, label).into_widget())
            .collect();
        if cards.is_empty() {
            continue;
        }
        total += cards.len();

        items.push(
            row(children![
                text(group.label.to_string())
                    .size(12.0)
                    .weight(700.0)
                    .color(c.muted_foreground),
                gap_w(8.0),
                container()
                    .decoration(
                        BoxDecoration::new()
                            .color(c.muted)
                            .radius(BorderRadius::all(999.0))
                    )
                    .padding(EdgeInsets::symmetric(7.0, 1.0))
                    .child(
                        text(cards.len().to_string())
                            .size(10.5)
                            .weight(600.0)
                            .color(c.muted_foreground)
                    ),
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
                .alignment(Alignment::CENTER)
                .child(
                    column(children![
                        icon(lucide::SEARCH_X).size(26.0).color(c.muted_foreground),
                        gap_h(12.0),
                        text(format!("No components match “{}”", p.query.get()))
                            .size(15.0)
                            .weight(600.0)
                            .color(c.foreground),
                        gap_h(4.0),
                        text("Try a different term — like “button”, “list”, or “dialog”.")
                            .size(13.0)
                            .color(c.muted_foreground),
                    ])
                    .cross_axis_alignment(CrossAxisAlignment::Center)
                    .main_axis_size(MainAxisSize::Min),
                )
                .into_widget(),
        );
    }

    column(items)
        .cross_axis_alignment(CrossAxisAlignment::Stretch)
        .main_axis_size(MainAxisSize::Min)
}

/// The docs index page: a top bar with a search box + a scrolling, filtered grid.
pub fn docs_index() -> Element {
    let c = theme().colors;
    let dark = theme().dark;
    let query = create_signal(String::new());

    let brand_mark = pressable(
        row(children![
            icon(lucide::GEM).size(18.0).color(brand::INDIGO),
            gap_w(9.0),
            text("Pebbles").size(17.0).bold().color(c.foreground),
        ])
        .main_axis_size(MainAxisSize::Min)
        .cross_axis_alignment(CrossAxisAlignment::Center),
    )
    .radius(8.0)
    .on_tap(to_landing);

    let search = text_field()
        .leading(lucide::SEARCH)
        .placeholder("Search components…")
        .width(460.0)
        .bind(query);

    let theme_toggle =
        icon_button(if dark { lucide::SUN } else { lucide::MOON }).on_pressed(toggle_theme);

    let top_bar = container()
        .decoration(
            BoxDecoration::new()
                .color(c.background)
                .border(Border::new(c.border, 1.0)),
        )
        .padding(EdgeInsets::symmetric(24.0, 14.0))
        .child(
            row(children![
                brand_mark,
                spacer(),
                search,
                spacer(),
                theme_toggle,
                gap_w(8.0),
                button("Home")
                    .variant(ButtonVariant::Ghost)
                    .size(ButtonSize::Sm)
                    .leading(lucide::HOUSE)
                    .on_pressed(to_landing),
            ])
            .cross_axis_alignment(CrossAxisAlignment::Center),
        );

    let body = scroll_view(
        container().padding(EdgeInsets::symmetric(32.0, 30.0)).child(
            column(children![
                text("Components").size(30.0).bold().color(c.foreground),
                gap_h(6.0),
                text("Every widget in the Pebbles catalog — live and themeable. Pick one to open it in the docs.")
                    .size(15.0)
                    .line_height(1.5)
                    .color(c.muted_foreground),
                gap_h(30.0),
                component_props(render_grid, GridProps { query }),
            ])
            .cross_axis_alignment(CrossAxisAlignment::Stretch)
            .main_axis_size(MainAxisSize::Min),
        ),
    );

    column(children![top_bar, expanded(body)])
        .cross_axis_alignment(CrossAxisAlignment::Stretch)
        .main_axis_size(MainAxisSize::Max)
        .into_widget()
}
