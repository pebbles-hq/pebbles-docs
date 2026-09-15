use pebbles::prelude::*;

use crate::ui::{doc, gap_w, screen};

pub fn chips() -> Element {
    screen("Chip")
        .description(
            "A compact tag pill with an optional leading icon and a deletable ✕ — the token widget for filters, tags and selections.",
        )
        .body(children![
            basic(),
            deletable(),
            icons(),
        ])
}

fn basic() -> impl IntoWidget {
    doc("Basic")
        .description("A label in a secondary pill. Chips take .style(..) like every component; .disabled(..) mutes them.")
        .body(
            row(children![
                chip("Rust"),
                gap_w(8.0),
                chip("Signals"),
                gap_w(8.0),
                chip("Vello"),
                gap_w(8.0),
                chip("shadcn-inspired"),
                gap_w(8.0),
                chip("disabled").disabled(true),
                gap_w(8.0),
                chip("styled").style(style().background(palette::blue::S500)),
            ])
            .main_axis_size(MainAxisSize::Min),
        )
}

fn deletable() -> impl IntoWidget {
    let tags = create_signal(vec!["Filters".to_string(), "Backlog".to_string(), "In Review".to_string()]);
    doc("Deletable")
        .description(".deletable(true).on_deleted(..) adds the ✕ affordance; the chip does NOT remove itself — the owner drops it from its list (controlled, like every value).")
        .body({
            let list = tags
                .get()
                .iter()
                .enumerate()
                .map(|(_i, label)| {
                    let label = label.clone();
                    chip(label.clone())
                        .deletable(true)
                        .on_deleted(move || {
                            tags.update(|t| t.retain(|x| *x != label));
                        })
                        .into_widget()
                })
                .collect::<Vec<_>>();
            row(children![
                row(list).main_axis_size(MainAxisSize::Min).spacing(8.0),
                gap_w(12.0),
                if tags.get().is_empty() {
                    muted("all removed — click a chip to add it back below").into_widget()
                } else {
                    gap_w(0.0).into_widget()
                },
            ])
            .main_axis_size(MainAxisSize::Min)
        })
}

fn icons() -> impl IntoWidget {
    doc("Icons")
        .description(
            ".icon(..) adds a leading glyph — avatars of contact chips, category markers on filter chips.",
        )
        .body(
            row(children![
                chip("Work").icon(tabler::BRIEFCASE),
                gap_w(8.0),
                chip("Personal").icon(tabler::HOME),
                gap_w(8.0),
                chip("Starred").icon(tabler::STAR).deletable(true).on_deleted(|| {}),
                gap_w(8.0),
                chip("Locked").icon(tabler::LOCK).disabled(true),
            ])
            .main_axis_size(MainAxisSize::Min),
        )
}
