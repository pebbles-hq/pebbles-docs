//! Shared UI helpers, plus a **props** demonstration. Everything returns
//! `impl IntoWidget`, so screens compose with no `.into_widget()`.
//!
//! ### On props in Rust
//! Props work (see [`stat_card`]), but the idiomatic split is: **local state** →
//! `create_signal` in the component; **shared/app state** → a global signal/`Store`
//! (no prop-drilling); **props** → only for reusable, parameterized widgets.

use pebbles::prelude::*;

pub use pebbles::prelude::{gap_h, gap_w};

/// Brand accent colors, sampled from the Pebbles mascot (`assets/pebbles.png`) — the
/// warm fur browns + cream, the pink nose, and the console teal. Kept independent of
/// the shadcn theme so the accents read the same in light and dark mode.
pub mod brand {
    use pebbles::prelude::Color;

    /// Light fur tan (gradient start).
    pub const TAN: Color = Color::from_rgba8(0xC9, 0x9E, 0x76, 0xFF);
    /// Primary fur brown — the signature brand hue.
    pub const BROWN: Color = Color::from_rgba8(0x9C, 0x6E, 0x4C, 0xFF);
    /// Deep fur / outline brown (gradient end).
    pub const COCOA: Color = Color::from_rgba8(0x5F, 0x40, 0x2E, 0xFF);
    /// Pink nose — a soft accent.
    pub const NOSE: Color = Color::from_rgba8(0xDD, 0x93, 0x94, 0xFF);
    /// Console-text teal — a cool accent.
    pub const TEAL: Color = Color::from_rgba8(0x4E, 0x9C, 0x8C, 0xFF);
    /// Warm near-black (the mascot's outline) — behind the hero in dark mode.
    pub const INK: Color = Color::from_rgba8(0x25, 0x1F, 0x1B, 0xFF);
}

/// The signature diagonal brand gradient — warm fur tones (tan → brown → cocoa).
pub fn brand_gradient() -> Gradient {
    Gradient::linear(Alignment::TOP_LEFT, Alignment::BOTTOM_RIGHT, [brand::TAN, brand::BROWN, brand::COCOA])
}

/// The Pebbles mascot logo, decoded once per thread and cached (used in the top nav
/// and the footer). `None` if the bundled PNG fails to decode.
pub fn logo() -> Option<Image> {
    thread_local! {
        static LOGO: Option<Image> = image_from_bytes(include_bytes!("../assets/pebbles.png"));
    }
    LOGO.with(|l| l.clone())
}

/// The mascot logo at `size`×`size` (contained, never cropped), falling back to a gem
/// glyph if the bundled image can't be decoded. The brand mark in the nav/footer.
pub fn logo_mark(size: f64) -> AnyWidget {
    match logo() {
        Some(img) => ImageView::image(img).fit(ImageFit::Contain).width(size).height(size).into_widget(),
        None => icon(lucide::GEM).size(size * 0.6).color(brand::BROWN).into_widget(),
    }
}

/// A scrollable, padded screen with a heading + subtitle.
#[derive(Clone, Default)]
pub struct Screen {
    title: String,
    description: Option<String>,
}

/// Create a [`Screen`] with a heading.
pub fn screen(title: &str) -> Screen {
    Screen {
        title: title.to_string(),
        ..Default::default()
    }
}

impl Screen {
    /// The muted one-liner under the heading.
    pub fn description(mut self, sub: &str) -> Self {
        self.description = Some(sub.to_string());
        self
    }
    /// The screen body (sections / docs). Builds and returns the `Element`.
    pub fn body(self, body: impl IntoChildren) -> Element {
        let mut items: Vec<AnyWidget> = vec![
            heading(self.title.clone()).into_widget(),
            gap_h(4.0).into_widget(),
        ];
        if let Some(sub) = &self.description {
            items.push(subtitle(sub.clone()).into_widget());
        }
        items.push(gap_h(24.0).into_widget());
        items.extend(body.into_children());
        scroll_view(
            container().padding(EdgeInsets::all(30.0)).child(
                column(items)
                    .cross_axis_alignment(CrossAxisAlignment::Stretch)
                    .main_axis_size(MainAxisSize::Min),
            ),
        )
        .into_widget()
    }
}

/// A labeled sub-section within a screen.
pub fn section(title: &str, body: impl IntoWidget) -> Element {
    column(children![
        text(title.to_string())
            .size(12.0)
            .semibold()
            .color(theme().colors.muted_foreground),
        gap_h(12.0),
        body,
        gap_h(28.0),
    ])
    .cross_axis_alignment(CrossAxisAlignment::Stretch)
    .main_axis_size(MainAxisSize::Min)
    .into_widget()
}

/// A documentation-style section: a title, a descriptive sentence, then examples.
/// The house style for the showcase screens.
#[derive(Clone, Default)]
pub struct Doc {
    title: String,
    description: Option<String>,
}

/// Create a [`Doc`] with a section title.
pub fn doc(title: &str) -> Doc {
    Doc {
        title: title.to_string(),
        ..Default::default()
    }
}

impl Doc {
    /// The one-sentence explanation under the title.
    pub fn description(mut self, desc: &str) -> Self {
        self.description = Some(desc.to_string());
        self
    }
    /// The example body. Builds and returns the `Element`.
    pub fn body(self, body: impl IntoWidget) -> Element {
        let c = theme().colors;
        let mut items: Vec<AnyWidget> = vec![
            text(self.title)
                .size(16.0)
                .semibold()
                .color(c.foreground)
                .into_widget(),
            gap_h(4.0).into_widget(),
        ];
        if let Some(desc) = &self.description {
            items.push(
                text(desc.clone())
                    .size(13.5)
                    .line_height(1.45)
                    .color(c.muted_foreground)
                    .into_widget(),
            );
        }
        items.push(gap_h(16.0).into_widget());
        items.push(body.into_widget());
        items.push(gap_h(34.0).into_widget());
        column(items)
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .main_axis_size(MainAxisSize::Min)
            .into_widget()
    }
}

// ---------------------------------------------------------------------------
// Props demo: a reusable stat card via `component_props`.
// ---------------------------------------------------------------------------

pub struct StatCardProps {
    pub title: String,
    pub value: String,
    pub icon: IconKind,
    pub tint: Color,
}

/// A reusable stat tile — the canonical props case: a parameterized widget reused
/// with different inputs.
pub fn stat_card(title: &str, value: &str, icon: IconKind, tint: Color) -> impl IntoWidget {
    component_props(
        render_stat_card,
        StatCardProps {
            title: title.into(),
            value: value.into(),
            icon,
            tint,
        },
    )
}

fn render_stat_card(p: &StatCardProps) -> Card {
    let c = theme().colors;
    card().child(
        column(children![
            row(children![
                container()
                    .decoration(
                        BoxDecoration::new()
                            .color(p.tint)
                            .radius(BorderRadius::all(8.0))
                    )
                    .padding(EdgeInsets::all(8.0))
                    .child(icon(p.icon).size(18.0).color(palette::WHITE)),
                gap_w(10.0),
                text(p.title.clone()).size(13.0).color(c.muted_foreground),
            ])
            .main_axis_size(MainAxisSize::Min),
            gap_h(10.0),
            text(p.value.clone()).size(26.0).bold().color(c.foreground),
        ])
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .main_axis_size(MainAxisSize::Min),
    )
}
