//! The **Showcase** — apps built with Pebbles, to demonstrate what the framework can
//! do. A responsive grid of app cards (each a gradient thumbnail + name + tagline +
//! tags) with a "submit yours" call to action at the end.

use pebbles::prelude::*;

use crate::ui::{brand, brand_gradient, gap_h};

/// One showcase entry.
struct App {
    name: &'static str,
    tagline: &'static str,
    tags: &'static [&'static str],
    icon: IconData,
    grad: fn() -> Gradient,
}

fn indigo_teal() -> Gradient {
    Gradient::linear(Alignment::TOP_LEFT, Alignment::BOTTOM_RIGHT, [brand::TAN, brand::BROWN])
}
fn violet() -> Gradient {
    Gradient::linear(Alignment::TOP_LEFT, Alignment::BOTTOM_RIGHT, [palette::INDIGO, palette::VIOLET])
}
fn teal() -> Gradient {
    Gradient::linear(Alignment::TOP_LEFT, Alignment::BOTTOM_RIGHT, [palette::TEAL, palette::BLUE])
}
fn pink() -> Gradient {
    Gradient::linear(Alignment::TOP_LEFT, Alignment::BOTTOM_RIGHT, [palette::PINK, palette::AMBER])
}

const APPS: &[App] = &[
    App {
        name: "Pebbles Gallery",
        tagline: "This site — a routed docs + component catalog, built entirely in Pebbles.",
        tags: &["Docs", "Catalog", "Web"],
        icon: lucide::LAYOUT_DASHBOARD,
        grad: brand_gradient,
    },
    App {
        name: "Inventory Studio",
        tagline: "A desktop analytics dashboard: KPIs, charts, and a live data table.",
        tags: &["Desktop", "Data Table", "Charts"],
        icon: lucide::TABLE,
        grad: violet,
    },
    App {
        name: "Wallet",
        tagline: "A mobile finance app — animated balance, transaction lists, bottom nav.",
        tags: &["Mobile", "Motion", "Lists"],
        icon: lucide::CREDIT_CARD,
        grad: teal,
    },
    App {
        name: "Canvas Notes",
        tagline: "An infinite-canvas note app showing off custom painting and gestures.",
        tags: &["Canvas", "Gestures", "Custom"],
        icon: lucide::PALETTE,
        grad: pink,
    },
    App {
        name: "Kanban",
        tagline: "A drag-and-drop board — reorderable columns and cards, live filters.",
        tags: &["Drag & Drop", "State", "Desktop"],
        icon: lucide::LAYOUT_GRID,
        grad: indigo_teal,
    },
];

fn tag(label: &str) -> impl IntoWidget {
    let c = theme().colors;
    container()
        .decoration(BoxDecoration::new().color(c.secondary).radius(BorderRadius::all(999.0)))
        .padding(EdgeInsets::symmetric(9.0, 3.0))
        .child(text(label.to_string()).size(11.0).weight(600.0).color(c.muted_foreground))
}

fn app_card(a: &App) -> impl IntoWidget {
    let c = theme().colors;
    let tags: Vec<AnyWidget> = a.tags.iter().map(|t| tag(t).into_widget()).collect();
    container()
        .width(300.0)
        .decoration(
            BoxDecoration::new()
                .color(c.card)
                .radius(BorderRadius::all(14.0))
                .border(Border::new(c.border, 1.0)),
        )
        .child(
            column(children![
                // Thumbnail
                container()
                    .height(150.0)
                    .decoration(BoxDecoration::new().gradient((a.grad)()).radius(BorderRadius::all(0.0)))
                    .alignment(Alignment::CENTER)
                    .child(icon(a.icon).size(40.0).color(palette::WHITE)),
                container().padding(EdgeInsets::all(16.0)).child(
                    column(children![
                        text(a.name.to_string()).size(17.0).bold().color(c.foreground),
                        gap_h(5.0),
                        text(a.tagline.to_string()).size(13.0).line_height(1.5).color(c.muted_foreground),
                        gap_h(12.0),
                        wrap(tags).spacing(6.0).run_spacing(6.0),
                    ])
                    .cross_axis_alignment(CrossAxisAlignment::Start)
                    .main_axis_size(MainAxisSize::Min),
                ),
            ])
            .cross_axis_alignment(CrossAxisAlignment::Stretch)
            .main_axis_size(MainAxisSize::Min),
        )
}

/// The "add your app" card.
fn submit_card() -> impl IntoWidget {
    let c = theme().colors;
    container()
        .width(300.0)
        .decoration(
            BoxDecoration::new()
                .color(c.background)
                .radius(BorderRadius::all(14.0))
                .border(Border::new(c.border, 1.0)),
        )
        .padding(EdgeInsets::all(20.0))
        .alignment(Alignment::CENTER)
        .child(
            column(children![
                icon(lucide::PLUS).size(30.0).color(brand::BROWN),
                gap_h(12.0),
                text("Built something with Pebbles?").size(15.0).weight(600.0).color(c.foreground).align(TextAlign::Center),
                gap_h(6.0),
                text("Open a PR to add it here.").size(13.0).color(c.muted_foreground).align(TextAlign::Center),
            ])
            .cross_axis_alignment(CrossAxisAlignment::Center)
            .main_axis_size(MainAxisSize::Min),
        )
}

/// The showcase page body (title + responsive grid). Wrapped in chrome by [`showcase`].
fn body() -> impl IntoWidget {
    let c = theme().colors;
    let mut cards: Vec<AnyWidget> = APPS.iter().map(|a| app_card(a).into_widget()).collect();
    cards.push(submit_card().into_widget());

    scroll_view(
        container().padding(EdgeInsets::symmetric(40.0, 34.0)).child(center(
            container().constraints(BoxConstraints::loose(Size::new(1000.0, f64::INFINITY))).child(
                column(children![
                    text("Showcase").size(32.0).bold().color(c.foreground),
                    gap_h(8.0),
                    text("Apps built with Pebbles — desktop, mobile, and web. Proof the framework scales from a counter to a full product.")
                        .size(16.0)
                        .line_height(1.55)
                        .color(c.muted_foreground),
                    gap_h(30.0),
                    wrap(cards).spacing(20.0).run_spacing(20.0),
                    gap_h(20.0),
                ])
                .cross_axis_alignment(CrossAxisAlignment::Start)
                .main_axis_size(MainAxisSize::Min),
            ),
        )),
    )
}

pub fn showcase() -> Element {
    container()
        .color(theme().colors.background)
        .child(stack(children![
            column(children![crate::site_nav::top_nav(), expanded(body())])
                .cross_axis_alignment(CrossAxisAlignment::Stretch)
                .main_axis_size(MainAxisSize::Max),
            crate::site_nav::mobile_menu(Vec::new()),
        ]))
        .into_widget()
}
