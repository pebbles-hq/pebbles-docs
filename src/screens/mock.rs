//! Live "product" mocks for the landing hero — a desktop app window and a phone,
//! both assembled from real Pebbles widgets (not screenshots). They render exactly
//! the way a shipped app would, which is the point: the landing page *is* a Pebbles
//! app showing off Pebbles apps.

use pebbles::prelude::*;

use crate::ui::brand;

// ---------------------------------------------------------------------------
// Desktop window mock
// ---------------------------------------------------------------------------

/// A macOS-style traffic-light dot.
fn dot(color: Color) -> impl IntoWidget {
    container().width(11.0).height(11.0).decoration(
        BoxDecoration::new()
            .color(color)
            .radius(BorderRadius::all(999.0)),
    )
}

/// A tiny left-rail nav row (icon + label), the `selected` one tinted.
fn rail_item(ic: IconData, label: &str, selected: bool) -> impl IntoWidget {
    let c = theme().colors;
    let fg = if selected {
        brand::BROWN
    } else {
        c.muted_foreground
    };
    let deco = if selected {
        BoxDecoration::new()
            .color(c.secondary)
            .radius(BorderRadius::all(8.0))
    } else {
        BoxDecoration::new()
    };
    container()
        .decoration(deco)
        .padding(EdgeInsets::symmetric(8.0, 7.0))
        .child(
            row(children![
                icon(ic).size(14.0).color(fg),
                gap_w(8.0),
                text(label.to_string())
                    .size(12.0)
                    .weight(if selected { 600.0 } else { 500.0 })
                    .color(fg),
            ])
            .main_axis_size(MainAxisSize::Min),
        )
}

/// One KPI tile in the dashboard body.
fn kpi(label: &str, value: &str, delta: &str, tint: Color) -> impl IntoWidget {
    let c = theme().colors;
    expanded(
        container()
            .decoration(
                BoxDecoration::new()
                    .color(c.card)
                    .radius(BorderRadius::all(10.0))
                    .border(Border::new(c.border, 1.0)),
            )
            .padding(EdgeInsets::all(12.0))
            .child(
                column(children![
                    row(children![
                        container().width(8.0).height(8.0).decoration(
                            BoxDecoration::new()
                                .color(tint)
                                .radius(BorderRadius::all(999.0))
                        ),
                        gap_w(6.0),
                        text(label.to_string()).size(11.0).color(c.muted_foreground),
                    ])
                    .main_axis_size(MainAxisSize::Min),
                    gap_h(8.0),
                    text(value.to_string())
                        .size(20.0)
                        .bold()
                        .color(c.foreground),
                    gap_h(2.0),
                    text(delta.to_string())
                        .size(10.5)
                        .weight(600.0)
                        .color(palette::GREEN),
                ])
                .cross_axis_alignment(CrossAxisAlignment::Start)
                .main_axis_size(MainAxisSize::Min),
            ),
    )
}

/// One bar in the mini chart (height in px, gradient fill).
fn bar(h: f64) -> impl IntoWidget {
    container().width(16.0).height(h).decoration(
        BoxDecoration::new()
            .gradient(Gradient::vertical([brand::TAN, brand::BROWN]))
            .radius(BorderRadius::all(4.0)),
    )
}

/// A transaction-style list row (dot + name + amount).
fn list_row(name: &str, meta: &str, amount: &str, tint: Color) -> impl IntoWidget {
    let c = theme().colors;
    row(children![
        container().width(26.0).height(26.0).decoration(
            BoxDecoration::new()
                .color(tint)
                .radius(BorderRadius::all(999.0))
        ),
        gap_w(10.0),
        expanded(
            column(children![
                text(name.to_string())
                    .size(12.0)
                    .weight(600.0)
                    .color(c.foreground),
                text(meta.to_string()).size(10.5).color(c.muted_foreground),
            ])
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .main_axis_size(MainAxisSize::Min),
        ),
        text(amount.to_string())
            .size(12.0)
            .weight(600.0)
            .color(c.foreground),
    ])
    .cross_axis_alignment(CrossAxisAlignment::Center)
}

/// A desktop app window built from Pebbles widgets — a small analytics dashboard.
pub fn desktop_mock() -> impl IntoWidget {
    let c = theme().colors;

    // --- title bar ---
    let title_bar = container()
        .decoration(
            BoxDecoration::new()
                .color(c.muted)
                .radius(BorderRadius::all(0.0)),
        )
        .padding(EdgeInsets::symmetric(14.0, 11.0))
        .child(
            row(children![
                dot(Color::from_rgba8(0xFF, 0x5F, 0x57, 0xFF)),
                gap_w(7.0),
                dot(Color::from_rgba8(0xFE, 0xBC, 0x2E, 0xFF)),
                gap_w(7.0),
                dot(Color::from_rgba8(0x28, 0xC8, 0x40, 0xFF)),
                gap_w(14.0),
                text("Pebbles Studio")
                    .size(12.0)
                    .weight(600.0)
                    .color(c.muted_foreground),
                spacer(),
                icon(lucide::SEARCH).size(13.0).color(c.muted_foreground),
                gap_w(12.0),
                icon(lucide::BELL).size(13.0).color(c.muted_foreground),
            ])
            .cross_axis_alignment(CrossAxisAlignment::Center),
        );

    // --- left rail ---
    let rail = container()
        .width(146.0)
        .padding(EdgeInsets::all(10.0))
        .child(
            column(children![
                rail_item(lucide::LAYOUT_DASHBOARD, "Dashboard", true),
                gap_h(3.0),
                rail_item(lucide::TABLE, "Reports", false),
                gap_h(3.0),
                rail_item(lucide::USER, "Customers", false),
                gap_h(3.0),
                rail_item(lucide::CREDIT_CARD, "Billing", false),
                gap_h(3.0),
                rail_item(lucide::PALETTE, "Appearance", false),
            ])
            .cross_axis_alignment(CrossAxisAlignment::Stretch)
            .main_axis_size(MainAxisSize::Min),
        );

    // --- content ---
    let content = expanded(
        container().padding(EdgeInsets::all(16.0)).child(
            column(children![
                row(children![
                    column(children![
                        text("Overview").size(16.0).bold().color(c.foreground),
                        text("Last 30 days").size(11.0).color(c.muted_foreground),
                    ])
                    .cross_axis_alignment(CrossAxisAlignment::Start)
                    .main_axis_size(MainAxisSize::Min),
                    spacer(),
                    container()
                        .decoration(
                            BoxDecoration::new()
                                .gradient(crate::ui::brand_gradient())
                                .radius(BorderRadius::all(8.0)),
                        )
                        .padding(EdgeInsets::symmetric(12.0, 7.0))
                        .child(
                            row(children![
                                icon(lucide::PLUS).size(13.0).color(palette::WHITE),
                                gap_w(5.0),
                                text("New").size(12.0).weight(600.0).color(palette::WHITE),
                            ])
                            .main_axis_size(MainAxisSize::Min),
                        ),
                ])
                .cross_axis_alignment(CrossAxisAlignment::Center),
                gap_h(14.0),
                row(children![
                    kpi("Revenue", "$48.2k", "+12.4%", brand::BROWN),
                    gap_w(10.0),
                    kpi("Active", "2,940", "+4.1%", brand::TEAL),
                    gap_w(10.0),
                    kpi("Churn", "1.8%", "-0.3%", brand::NOSE),
                ]),
                gap_h(16.0),
                container()
                    .decoration(
                        BoxDecoration::new()
                            .color(c.card)
                            .radius(BorderRadius::all(10.0))
                            .border(Border::new(c.border, 1.0)),
                    )
                    .padding(EdgeInsets::all(14.0))
                    .child(
                        column(children![
                            text("Weekly sales")
                                .size(12.0)
                                .weight(600.0)
                                .color(c.foreground),
                            gap_h(12.0),
                            row(children![
                                bar(34.0),
                                gap_w(9.0),
                                bar(52.0),
                                gap_w(9.0),
                                bar(28.0),
                                gap_w(9.0),
                                bar(64.0),
                                gap_w(9.0),
                                bar(46.0),
                                gap_w(9.0),
                                bar(72.0),
                                gap_w(9.0),
                                bar(40.0),
                            ])
                            .cross_axis_alignment(CrossAxisAlignment::End)
                            .main_axis_size(MainAxisSize::Min),
                        ])
                        .cross_axis_alignment(CrossAxisAlignment::Start)
                        .main_axis_size(MainAxisSize::Min),
                    ),
                gap_h(14.0),
                list_row(
                    "Acme Inc.",
                    "Enterprise · seat +12",
                    "$2,400",
                    brand::BROWN
                ),
                gap_h(10.0),
                list_row("Globex", "Pro · renewed", "$780", brand::TEAL),
                gap_h(10.0),
                list_row("Umbrella", "Starter · upgraded", "$120", brand::NOSE),
            ])
            .cross_axis_alignment(CrossAxisAlignment::Stretch)
            .main_axis_size(MainAxisSize::Min),
        ),
    );

    // --- assembled window ---
    container()
        .width(600.0)
        .decoration(
            BoxDecoration::new()
                .color(c.background)
                .radius(BorderRadius::all(14.0))
                .border(Border::new(c.border, 1.0))
                .shadow(BoxShadow::new(
                    Color::from_rgba8(0x0B, 0x0F, 0x1E, 0x40),
                    Offset::new(0.0, 30.0),
                    60.0,
                    -12.0,
                )),
        )
        .child(clip_rrect(
            BorderRadius::all(14.0),
            column(children![
                title_bar,
                container()
                    .decoration(BoxDecoration::new().color(c.border))
                    .height(1.0),
                row(children![rail, content]).cross_axis_alignment(CrossAxisAlignment::Stretch),
            ])
            .cross_axis_alignment(CrossAxisAlignment::Stretch)
            .main_axis_size(MainAxisSize::Min),
        ))
}

// ---------------------------------------------------------------------------
// Phone mock
// ---------------------------------------------------------------------------

/// A phone-app list row (avatar + two lines + trailing chevron).
fn phone_row(ic: IconData, tint: Color, title: &str, sub: &str) -> impl IntoWidget {
    let c = theme().colors;
    row(children![
        container()
            .width(34.0)
            .height(34.0)
            .decoration(
                BoxDecoration::new()
                    .color(tint)
                    .radius(BorderRadius::all(10.0))
            )
            .alignment(Alignment::CENTER)
            .child(icon(ic).size(16.0).color(palette::WHITE)),
        gap_w(11.0),
        expanded(
            column(children![
                text(title.to_string())
                    .size(13.0)
                    .weight(600.0)
                    .color(c.foreground),
                text(sub.to_string()).size(11.0).color(c.muted_foreground),
            ])
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .main_axis_size(MainAxisSize::Min),
        ),
        icon(lucide::CHEVRON_RIGHT)
            .size(15.0)
            .color(c.muted_foreground),
    ])
    .cross_axis_alignment(CrossAxisAlignment::Center)
}

/// A bottom-nav icon, the active one tinted.
fn tab(ic: IconData, active: bool) -> impl IntoWidget {
    let c = theme().colors;
    expanded(center(icon(ic).size(19.0).color(if active {
        brand::BROWN
    } else {
        c.muted_foreground
    })))
}

/// A phone running a Pebbles app — the same widgets, a mobile shell.
pub fn mobile_mock() -> impl IntoWidget {
    let c = theme().colors;

    // The app screen (inside the bezel).
    let screen = column(children![
        // Gradient hero header with a balance card.
        container()
            .decoration(
                BoxDecoration::new()
                    .gradient(crate::ui::brand_gradient())
                    .radius(BorderRadius::all(0.0)),
            )
            .padding(EdgeInsets::only(18.0, 22.0, 18.0, 20.0))
            .child(
                column(children![
                    row(children![
                        text("Wallet").size(15.0).bold().color(palette::WHITE),
                        spacer(),
                        icon(lucide::BELL).size(16.0).color(palette::WHITE),
                    ])
                    .cross_axis_alignment(CrossAxisAlignment::Center),
                    gap_h(16.0),
                    text("Total balance")
                        .size(11.0)
                        .color(Color::from_rgba8(0xFF, 0xFF, 0xFF, 0xCC)),
                    gap_h(3.0),
                    text("$12,480.55").size(27.0).bold().color(palette::WHITE),
                ])
                .cross_axis_alignment(CrossAxisAlignment::Start)
                .main_axis_size(MainAxisSize::Min),
            ),
        // List body.
        expanded(
            container()
                .color(c.background)
                .padding(EdgeInsets::all(16.0))
                .child(
                    column(children![
                        text("Recent activity")
                            .size(11.0)
                            .weight(600.0)
                            .color(c.muted_foreground),
                        gap_h(12.0),
                        phone_row(
                            lucide::CREDIT_CARD,
                            brand::BROWN,
                            "Apple Store",
                            "Today · Shopping"
                        ),
                        gap_h(14.0),
                        phone_row(lucide::HOUSE, brand::TEAL, "Rent", "Yesterday · Housing"),
                        gap_h(14.0),
                        phone_row(
                            lucide::COFFEE,
                            brand::NOSE,
                            "Blue Bottle",
                            "Yesterday · Food"
                        ),
                        gap_h(14.0),
                        phone_row(lucide::GAUGE, palette::GREEN, "Payday", "Mon · Income"),
                    ])
                    .cross_axis_alignment(CrossAxisAlignment::Stretch)
                    .main_axis_size(MainAxisSize::Min),
                ),
        ),
        // Bottom nav.
        container()
            .decoration(
                BoxDecoration::new()
                    .color(c.card)
                    .border(Border::new(c.border, 1.0))
            )
            .padding(EdgeInsets::symmetric(8.0, 12.0))
            .child(
                row(children![
                    tab(lucide::HOUSE, true),
                    tab(lucide::TABLE, false),
                    tab(lucide::CREDIT_CARD, false),
                    tab(lucide::USER, false),
                ])
                .cross_axis_alignment(CrossAxisAlignment::Center),
            ),
    ])
    .cross_axis_alignment(CrossAxisAlignment::Stretch)
    .main_axis_size(MainAxisSize::Max);

    // The device bezel: a dark rounded frame clipping a 250×520 screen.
    container()
        .decoration(
            BoxDecoration::new()
                .color(brand::INK)
                .radius(BorderRadius::all(40.0))
                .shadow(BoxShadow::new(
                    Color::from_rgba8(0x0B, 0x0F, 0x1E, 0x45),
                    Offset::new(0.0, 30.0),
                    60.0,
                    -10.0,
                )),
        )
        .padding(EdgeInsets::all(10.0))
        .child(
            container()
                .width(250.0)
                .height(520.0)
                .decoration(
                    BoxDecoration::new()
                        .color(c.background)
                        .radius(BorderRadius::all(30.0)),
                )
                .child(clip_rrect(BorderRadius::all(30.0), screen)),
        )
}
