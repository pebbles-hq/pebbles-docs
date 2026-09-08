//! The marketing landing page — the front door of the site. A full-bleed, scrolling
//! page (hero → live product mocks → features → code → platforms → CTA → footer),
//! built entirely from Pebbles widgets. The widget catalog now lives behind the
//! "Browse components" button under [`crate::screens::docs_index`].

use pebbles::prelude::*;

use crate::screens::mock::{desktop_mock, mobile_mock};
use crate::state::to_components;
use crate::ui::{brand, brand_gradient, gap_h, gap_w, is_compact, logo_mark};
use pebbles_code_editor::{EditorTheme, code_editor, lang};

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
    let compact = is_compact();
    let hsize = if compact { 34.0 } else { 52.0 };
    let hero_bg = if dark {
        Color::from_rgba8(0x1C, 0x16, 0x12, 0xFF)
    } else {
        Color::from_rgba8(0xFA, 0xF5, 0xEE, 0xFF)
    };

    let headline = column(children![
        text("Beautiful native UIs,").size(hsize).bold().color(c.foreground),
        text("written in Rust.").size(hsize).bold().color(brand::BROWN),
    ])
    .cross_axis_alignment(CrossAxisAlignment::Center)
    .main_axis_size(MainAxisSize::Min);

    let sub = container().constraints(BoxConstraints::loose(Size::new(660.0, f64::INFINITY))).child(
        text(
            "Pebbles brings good ideas from across software to Rust — a widget model in the \
             spirit of Flutter, reactivity like SolidJS, styling like Tailwind, GPU-rendered \
             with Vello. One codebase for desktop, web, and mobile.",
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

    // Desktop + phone mocks side by side; on a compact viewport the desktop window is
    // too wide, so show just the phone.
    let showcase: AnyWidget = if compact {
        mobile_mock().into_widget()
    } else {
        row(children![desktop_mock(), gap_w(28.0), mobile_mock()])
            .main_axis_size(MainAxisSize::Min)
            .cross_axis_alignment(CrossAxisAlignment::End)
            .into_widget()
    };

    // The hero owns the top nav so both share one background — no separate white strip.
    // The bar is transparent and scrolls with everything else.
    container()
        .decoration(BoxDecoration::new().color(hero_bg))
        .child(
            column(children![
                crate::site_nav::hero_nav(),
                band(
                    None,
                    if compact { 24.0 } else { 40.0 },
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
                ),
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
                    feature_card(lucide::ZAP, brand::BROWN, "Fine-grained reactivity", "Signals in the spirit of SolidJS drive the tree. One create_signal primitive for local and global state — no prop-drilling, no VDOM diffing."),
                    feature_card(lucide::LAYOUT_GRID, brand::TAN, "60+ styled components", "Buttons, inputs, dialogs, sheets, data tables, command palette — a full themed catalog, light and dark, with utility styling in the spirit of Tailwind."),
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

const COUNTER_SRC: &str = "fn counter() -> impl IntoWidget {\n    let count = create_signal(0);\n    column(children![\n        text(count.get().to_string()).size(48.0),\n        button(\"+\").on_pressed(move || {\n            count.update(|n| *n += 1);\n        }),\n    ])\n}";

fn code_block() -> impl IntoWidget {
    // The hero sample is rendered by our own pebbles-code-editor package (read-only).
    let cw = (media_query().size.width - 72.0).clamp(280.0, 520.0);
    let sig = create_signal(String::from(COUNTER_SRC));
    container()
        .width(cw)
        .decoration(BoxDecoration::new().radius(BorderRadius::all(14.0)).shadow(BoxShadow::new(
            Color::from_rgba8(0x0B, 0x0F, 0x1E, 0x40),
            Offset::new(0.0, 24.0),
            50.0,
            -12.0,
        )))
        .clip()
        .child(
            code_editor(sig)
                .language(Box::new(lang::Rust))
                .theme(EditorTheme::dark())
                .read_only(true)
                .title("counter.rs")
                .font_size(13.0),
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
                text("Familiar ideas, brought to Rust")
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
        .child(stack(children![
            scroll_view(
                column(children![
                    hero(),
                    features(),
                    code_section(),
                    cta(),
                    gap_h(10.0),
                    footer(),
                ])
                .cross_axis_alignment(CrossAxisAlignment::Stretch)
                .main_axis_size(MainAxisSize::Min),
            ),
            crate::site_nav::mobile_menu(Vec::new()),
        ])
        .fit(StackFit::Expand)
        .alignment(Alignment::TOP_LEFT))
        .into_widget()
}
