use serde::Serialize;

use crate::error::AppError;
use crate::output::{self, Ctx};

#[derive(Serialize)]
struct ModelEntry {
    id: &'static str,
    name: &'static str,
    tier: &'static str,
    audio: bool,
    max_resolution: &'static str,
    notes: &'static str,
}

pub fn run(ctx: Ctx) -> Result<(), AppError> {
    // Snapshot of models available on ModelArk as of 2026-08-06.
    // Source: https://docs.byteplus.com/en/docs/ModelArk/1330310
    let models = vec![
        ModelEntry {
            id: "dreamina-seedance-2-5-260628",
            name: "Seedance 2.5",
            tier: "standard",
            audio: true,
            max_resolution: "720p",
            notes: "NOT YET OPEN: BytePlus has published the model and its pricing, but \
                    Playground/API access is still rolling out -- calls return ModelNotOpen \
                    until you activate it in the Ark Console. Up to 30s per generation; \
                    references: 30 images, 10 videos, 10 audio clips.",
        },
        ModelEntry {
            id: "dreamina-seedance-2-0-260128",
            name: "Seedance 2.0",
            tier: "standard",
            audio: true,
            max_resolution: "4k",
            notes: "Best generally-available quality. 480p/720p/1080p/4k (4k is 10-bit and \
                    capped at 15 RPM / concurrency 1). Multimodal references: 1-9 images, \
                    1-3 videos, 1-3 audio clips.",
        },
        ModelEntry {
            id: "dreamina-seedance-2-0-fast-260128",
            name: "Seedance 2.0 Fast",
            tier: "fast",
            audio: true,
            max_resolution: "720p",
            notes: "Same features as 2.0 with lower latency and cost.",
        },
        ModelEntry {
            id: "dreamina-seedance-2-0-mini-260615",
            name: "Seedance 2.0 Mini",
            tier: "mini",
            audio: true,
            max_resolution: "720p",
            notes: "Cheapest 2.0-series tier. Same capability set as 2.0 Fast.",
        },
        ModelEntry {
            id: "seedance-1-5-pro-251215",
            name: "Seedance 1.5 Pro",
            tier: "standard",
            audio: true,
            max_resolution: "1080p",
            notes: "Previous generation with audio. Supports draft mode.",
        },
        ModelEntry {
            id: "seedance-1-0-pro-250528",
            name: "Seedance 1.0 Pro",
            tier: "standard",
            audio: false,
            max_resolution: "1080p",
            notes: "Silent video only. Text-to-video + first(+last) frame.",
        },
        ModelEntry {
            id: "seedance-1-0-pro-fast-251015",
            name: "Seedance 1.0 Pro Fast",
            tier: "fast",
            audio: false,
            max_resolution: "1080p",
            notes: "Silent video only. Lower latency / cost.",
        },
    ];

    output::print_success_or(ctx, &models, |m| {
        use owo_colors::OwoColorize;
        let mut table = comfy_table::Table::new();
        table.set_header(vec!["id", "name", "tier", "audio", "max res"]);
        for entry in m {
            table.add_row(vec![
                entry.id.cyan().to_string(),
                entry.name.bold().to_string(),
                entry.tier.to_string(),
                if entry.audio { "yes" } else { "no" }.to_string(),
                entry.max_resolution.to_string(),
            ]);
        }
        println!("{table}");
    });
    Ok(())
}
