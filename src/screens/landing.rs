//! The marketing landing page — the front door of the site. A full-bleed, scrolling
//! page (hero → live product mocks → features → code → platforms → CTA → footer),
//! built entirely from Pebbles widgets. The widget catalog now lives behind the
//! "Browse components" button under [`crate::screens::docs_index`].

use pebbles::prelude::*;

use crate::screens::mock::{desktop_mock, mobile_mock};
use crate::state::to_components;
use crate::ui::{brand, brand_gradient, gap_h, gap_w, logo_mark};

/// The max content width; sections are full-bleed but their content is centered here.
const MAXW: f64 = 1080.0;

/// A full-bleed section band: an optional background behind content centered to [`MAXW`].
fn band(bg: Option<Color>, pad_v: f64, content: impl IntoWidget) -> impl IntoWidget {
    let mut deco = BoxDecoration::new();
    if let Some(bg) = bg {
        deco = deco.color(bg);
    }
    container()
        .decoration(deco)
        .padding(EdgeInsets::symmetric(28.0, pad_v))
        .child(center(
            container()
                .constraints(BoxConstraints::loose(Size::new(MAXW, f64::INFINITY)))
                .child(content),
        ))
}

/// A small pill (gradient or bordered) used for eyebrows and trust chips.
fn gradient_pill(label: &str) -> impl IntoWidget {
    container()
        .decoration(
            BoxDecoration::new()
                .gradient(brand_gradient())
                .radius(BorderRadius::all(999.0)),
        )
        .padding(EdgeInsets::symmetric(13.0, 6.0))
        .child(
            text(label.to_string())
                .size(12.0)
                .weight(600.0)
                .color(palette::WHITE),
        )
}

fn chip(ic: IconData, label: &str) -> impl IntoWidget {
    let c = theme().colors;
    container()
        .decoration(
            BoxDecoration::new()
                .color(c.card)
                .radius(BorderRadius::all(999.0))
                .border(Border::new(c.border, 1.0)),
        )
        .padding(EdgeInsets::symmetric(12.0, 7.0))
        .child(
            row(children![
                icon(ic).size(13.0).color(c.muted_foreground),
                gap_w(6.0),
                text(label.to_string())
                    .size(12.0)
                    .weight(500.0)
                    .color(c.foreground),
            ])
            .main_axis_size(MainAxisSize::Min),
        )
}

// ---------------------------------------------------------------------------
// Hero
// ---------------------------------------------------------------------------

fn hero() -> impl IntoWidget {
    let c = theme().colors;
    let dark = theme().dark;
    let hero_bg = if dark {
        Color::from_rgba8(0x1C, 0x16, 0x12, 0xFF)
    } else {
        Color::from_rgba8(0xFA, 0xF5, 0xEE, 0xFF)
    };

    let headline = column(children![
        text("Beautiful native UIs,").size(52.0).bold().color(c.foreground),
        text("written in Rust.").size(52.0).bold().color(brand::BROWN),
    ])
    .cross_axis_alignment(CrossAxisAlignment::Center)
    .main_axis_size(MainAxisSize::Min);

    let sub = container().constraints(BoxConstraints::loose(Size::new(660.0, f64::INFINITY))).child(
        text(
            "Pebbles is a Flutter-style, GPU-accelerated GUI framework for Rust. \
             Reactive signals, 60+ shadcn-styled widgets, one codebase for desktop, web, and mobile.",
        )
        .size(17.5)
        .line_height(1.55)
        .color(c.muted_foreground)
        .align(TextAlign::Center),
    );

    let ctas = row(children![
        button("Browse components")
            .variant(ButtonVariant::Primary)
            .size(ButtonSize::Lg)
            .leading(lucide::LAYOUT_GRID)
            .on_pressed(to_components),
        gap_w(12.0),
        button("View on GitHub")
            .variant(ButtonVariant::Outline)
            .size(ButtonSize::Lg)
            .leading(lucide::STAR)
            .on_pressed(|| eprintln!("open https://github.com/pebbles-hq/pebbles")),
    ])
    .main_axis_size(MainAxisSize::Min);

    let trust = wrap(children![
        chip(lucide::ZAP, "Vello GPU renderer"),
        chip(lucide::LAYERS, "Retained widget tree"),
        chip(lucide::MONITOR_SMARTPHONE, "Desktop · Web · Mobile"),
        chip(lucide::SHIELD_CHECK, "Type-safe, no GC"),
    ])
    .spacing(10.0)
    .run_spacing(10.0)
    .alignment(WrapAlignment::Center);

    // Desktop + phone mocks, side by side (phone bottom-aligned to the window).
    let showcase = row(children![desktop_mock(), gap_w(28.0), mobile_mock()])
        .main_axis_size(MainAxisSize::Min)
        .cross_axis_alignment(CrossAxisAlignment::End);

    band(
        Some(hero_bg),
        64.0,
        column(children![
            center(gradient_pill("v0.0.1 · pure Rust · GPU-native")),
            gap_h(26.0),
            center(headline),
            gap_h(22.0),
            center(sub),
            gap_h(30.0),
            center(ctas),
            gap_h(22.0),
            center(trust),
            gap_h(56.0),
            center(showcase),
        ])
        .cross_axis_alignment(CrossAxisAlignment::Stretch)
        .main_axis_size(MainAxisSize::Min),
    )
}

// ---------------------------------------------------------------------------
// Feature grid
// ---------------------------------------------------------------------------

fn feature_card(ic: IconData, tint: Color, title: &str, desc: &str) -> impl IntoWidget {
    let c = theme().colors;
    container()
        .width(320.0)
        .decoration(
            BoxDecoration::new()
                .color(c.card)
                .radius(BorderRadius::all(14.0))
                .border(Border::new(c.border, 1.0)),
        )
        .padding(EdgeInsets::all(20.0))
        .child(
            column(children![
                container()
                    .decoration(
                        BoxDecoration::new()
                            .color(tint)
                            .radius(BorderRadius::all(10.0))
                    )
                    .padding(EdgeInsets::all(9.0))
                    .child(icon(ic).size(18.0).color(palette::WHITE)),
                gap_h(14.0),
                text(title.to_string())
                    .size(16.0)
                    .bold()
                    .color(c.foreground),
                gap_h(6.0),
                text(desc.to_string())
                    .size(13.5)
                    .line_height(1.5)
                    .color(c.muted_foreground),
            ])
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .main_axis_size(MainAxisSize::Min),
        )
}

fn features() -> impl IntoWidget {
    let c = theme().colors;
    band(
        None,
        56.0,
        column(children![
            text("Everything you need to ship a UI").size(30.0).bold().color(c.foreground).align(TextAlign::Center),
            gap_h(10.0),
            center(
                container().constraints(BoxConstraints::loose(Size::new(620.0, f64::INFINITY))).child(
                    text("A batteries-included toolkit — reactivity, layout, motion, and a full component library — with the ergonomics of a modern declarative framework.")
                        .size(15.0)
                        .line_height(1.55)
                        .color(c.muted_foreground)
                        .align(TextAlign::Center),
                ),
            ),
            gap_h(36.0),
            center(
                wrap(children![
                    feature_card(lucide::ZAP, brand::BROWN, "SolidJS-style reactivity", "Fine-grained signals drive the tree. One create_signal primitive for local and global state — no prop-drilling, no VDOM diffing."),
                    feature_card(lucide::LAYOUT_GRID, brand::TAN, "60+ styled components", "Buttons, inputs, dialogs, sheets, data tables, command palette — a full shadcn-flavored catalog, themeable light and dark."),
                    feature_card(lucide::CPU, brand::TEAL, "GPU rendering via Vello", "Every frame is rasterized on the GPU. Sub-pixel text, gradients, blurs and clips — smooth at any window size."),
                    feature_card(lucide::MONITOR_SMARTPHONE, palette::GREEN, "One codebase, everywhere", "The same widget tree runs on desktop (winit + wgpu), the web (WebGPU/wasm), and mobile shells."),
                    feature_card(lucide::WAND, brand::NOSE, "Motion built in", "Implicit and explicit animation widgets — animated containers, transitions, hero flights — with springs and curves."),
                    feature_card(lucide::SHIELD_CHECK, palette::AMBER, "Type-safe & fast", "Plain functions and closures, no macros to learn. Rust's borrow checker, zero GC pauses, predictable memory."),
                ])
                .spacing(18.0)
                .run_spacing(18.0)
                .alignment(WrapAlignment::Center),
            ),
        ])
        .cross_axis_alignment(CrossAxisAlignment::Stretch)
        .main_axis_size(MainAxisSize::Min),
    )
}

// ---------------------------------------------------------------------------
// Code sample
// ---------------------------------------------------------------------------

fn code_line(text_str: &str, tint: Color) -> impl IntoWidget {
    text(text_str.to_string())
        .font_family("JetBrains Mono")
        .size(13.0)
        .line_height(1.7)
        .color(tint)
}

fn code_block() -> impl IntoWidget {
    let ink = Color::from_rgba8(0x0D, 0x11, 0x1B, 0xFF);
    let dot = |col: Color| {
        container().width(11.0).height(11.0).decoration(
            BoxDecoration::new()
                .color(col)
                .radius(BorderRadius::all(999.0)),
        )
    };
    let base = Color::from_rgba8(0xE6, 0xE9, 0xF2, 0xFF);
    let dim = Color::from_rgba8(0x8B, 0x93, 0xA7, 0xFF);
    let kw = brand::TEAL;
    let str_c = Color::from_rgba8(0x86, 0xEF, 0xAC, 0xFF);

    container()
        .width(520.0)
        .decoration(
            BoxDecoration::new()
                .color(ink)
                .radius(BorderRadius::all(14.0))
                .shadow(BoxShadow::new(
                    Color::from_rgba8(0x0B, 0x0F, 0x1E, 0x40),
                    Offset::new(0.0, 24.0),
                    50.0,
                    -12.0,
                )),
        )
        .child(
            column(children![
                container().padding(EdgeInsets::all(14.0)).child(
                    row(children![
                        dot(Color::from_rgba8(0xFF, 0x5F, 0x57, 0xFF)),
                        gap_w(7.0),
                        dot(Color::from_rgba8(0xFE, 0xBC, 0x2E, 0xFF)),
                        gap_w(7.0),
                        dot(Color::from_rgba8(0x28, 0xC8, 0x40, 0xFF)),
                        gap_w(14.0),
                        text("counter.rs")
                            .font_family("JetBrains Mono")
                            .size(12.0)
                            .color(dim),
                    ])
                    .main_axis_size(MainAxisSize::Min)
                    .cross_axis_alignment(CrossAxisAlignment::Center),
                ),
                container()
                    .padding(EdgeInsets::only(20.0, 4.0, 20.0, 22.0))
                    .child(
                        column(children![
                            code_line("fn counter() -> impl IntoWidget {", base),
                            code_line("    let count = create_signal(0);", kw),
                            code_line("    column(children![", base),
                            code_line("        text(count.get().to_string()).size(48.0),", base),
                            code_line("        button(\"+\")", str_c),
                            code_line(
                                "            .on_pressed(move || count.update(|n| *n += 1)),",
                                base
                            ),
                            code_line("    ])", base),
                            code_line("}", base),
                        ])
                        .cross_axis_alignment(CrossAxisAlignment::Start)
                        .main_axis_size(MainAxisSize::Min),
                    ),
            ])
            .cross_axis_alignment(CrossAxisAlignment::Stretch)
            .main_axis_size(MainAxisSize::Min),
        )
}

fn code_section() -> impl IntoWidget {
    let c = theme().colors;
    let dark = theme().dark;
    let bg = if dark {
        Color::from_rgba8(0x1C, 0x16, 0x12, 0xFF)
    } else {
        Color::from_rgba8(0xF8, 0xF4, 0xED, 0xFF)
    };

    let copy = container()
        .constraints(BoxConstraints::loose(Size::new(420.0, f64::INFINITY)))
        .child(
            column(children![
                gradient_pill("Developer experience"),
                gap_h(18.0),
                text("If you know Flutter or Solid, you already know Pebbles")
                    .size(28.0)
                    .bold()
                    .color(c.foreground),
                gap_h(14.0),
                text(
                    "Components are plain functions returning a widget tree. State is a signal. \
                 Events are closures. No boilerplate, no build step, no runtime surprises — \
                 just `cargo run`.",
                )
                .size(15.5)
                .line_height(1.6)
                .color(c.muted_foreground),
                gap_h(22.0),
                column(children![
                    bullet(lucide::CHECK, "Function components — no structs or traits"),
                    gap_h(10.0),
                    bullet(lucide::CHECK, "Signals for local and global state alike"),
                    gap_h(10.0),
                    bullet(
                        lucide::CHECK,
                        "Hot, incremental builds; runs on every desktop OS"
                    ),
                ])
                .cross_axis_alignment(CrossAxisAlignment::Start)
                .main_axis_size(MainAxisSize::Min),
            ])
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .main_axis_size(MainAxisSize::Min),
        );

    band(
        Some(bg),
        60.0,
        center(
            wrap(children![copy, code_block()])
                .spacing(48.0)
                .run_spacing(36.0)
                .alignment(WrapAlignment::Center),
        ),
    )
}

fn bullet(ic: IconData, label: &str) -> impl IntoWidget {
    let c = theme().colors;
    row(children![
        container()
            .decoration(
                BoxDecoration::new()
                    .color(c.secondary)
                    .radius(BorderRadius::all(999.0))
            )
            .padding(EdgeInsets::all(4.0))
            .child(icon(ic).size(13.0).color(brand::BROWN)),
        gap_w(11.0),
        text(label.to_string()).size(14.5).color(c.foreground),
    ])
    .cross_axis_alignment(CrossAxisAlignment::Center)
    .main_axis_size(MainAxisSize::Min)
}

// ---------------------------------------------------------------------------
// CTA band + footer
// ---------------------------------------------------------------------------

fn cta() -> impl IntoWidget {
    band(
        None,
        20.0,
        container()
            .decoration(BoxDecoration::new().gradient(brand_gradient()).radius(BorderRadius::all(24.0)))
            .padding(EdgeInsets::symmetric(48.0, 52.0))
            .child(
                column(children![
                    text("Ready to build something native?").size(30.0).bold().color(palette::WHITE).align(TextAlign::Center),
                    gap_h(12.0),
                    text("Explore every widget in the interactive catalog — live, themeable, and copy-paste ready.")
                        .size(15.5)
                        .line_height(1.5)
                        .color(Color::from_rgba8(0xFF, 0xFF, 0xFF, 0xE0))
                        .align(TextAlign::Center),
                    gap_h(26.0),
                    center(
                        button("Explore the components")
                            .size(ButtonSize::Lg)
                            .color(palette::WHITE)
                            .text_color(brand::BROWN)
                            .trailing(lucide::ARROW_RIGHT)
                            .on_pressed(to_components),
                    ),
                ])
                .cross_axis_alignment(CrossAxisAlignment::Stretch)
                .main_axis_size(MainAxisSize::Min),
            ),
    )
}

fn footer() -> impl IntoWidget {
    let c = theme().colors;
    band(
        None,
        30.0,
        column(children![
            container()
                .decoration(BoxDecoration::new().color(c.border))
                .height(1.0),
            gap_h(22.0),
            row(children![
                row(children![
                    logo_mark(24.0),
                    gap_w(8.0),
                    text("Pebbles").size(15.0).bold().color(c.foreground),
                ])
                .main_axis_size(MainAxisSize::Min)
                .cross_axis_alignment(CrossAxisAlignment::Center),
                spacer(),
                text("Built with Pebbles · Apache-2.0")
                    .size(12.5)
                    .color(c.muted_foreground),
            ])
            .cross_axis_alignment(CrossAxisAlignment::Center),
        ])
        .cross_axis_alignment(CrossAxisAlignment::Stretch)
        .main_axis_size(MainAxisSize::Min),
    )
}

// ---------------------------------------------------------------------------
// Page
// ---------------------------------------------------------------------------

pub fn landing() -> Element {
    // Paint the theme background behind everything so transparent sections (and any
    // area the fixed window clear-color would show through) track light/dark on toggle.
    container()
        .color(theme().colors.background)
        .child(scroll_view(
            column(children![
                crate::site_nav::top_nav(),
                hero(),
                features(),
                code_section(),
                cta(),
                gap_h(10.0),
                footer(),
            ])
            .cross_axis_alignment(CrossAxisAlignment::Stretch)
            .main_axis_size(MainAxisSize::Min),
        ))
        .into_widget()
}
