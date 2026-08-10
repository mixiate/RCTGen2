#![windows_subsystem = "windows"]

mod adjacent_track;
mod app;
mod containers;
mod drawing;
mod modals;
mod panels;
mod render;
mod settings;
mod sprites;
mod widgets;

use eframe::egui;
use std::sync::{Arc, Mutex};

fn main() -> anyhow::Result<()> {
    use anyhow::Context as _;

    let (render_tx, render_rx) = std::sync::mpsc::channel();
    let (app_tx, app_rx) = std::sync::mpsc::channel();
    let track_image = Arc::new(Mutex::new(None));

    let render_thread = {
        let track_image = track_image.clone();
        std::thread::spawn(move || render::render_thread(&render_rx, &app_tx, &track_image))
    };

    let data_directory = std::env::current_exe()?;
    let data_directory = data_directory
        .parent()
        .with_context(|| format!("Could not get parent directory of {}", data_directory.display()))?;
    let data_directory = data_directory.join("data");

    let config_dir = dirs::config_dir().context("Could not get config directory")?.join("RCTGen2");

    let icon = include_bytes!("../resources/icon.png");
    let icon = eframe::icon_data::from_png_bytes(icon).expect("Could not load icon");

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_min_inner_size([800.0, 600.0]).with_icon(icon),
        centered: true,
        wgpu_options: egui_wgpu::WgpuConfiguration {
            surface: egui_wgpu::SurfaceConfig {
                present_mode: eframe::wgpu::PresentMode::AutoVsync,
                desired_maximum_frame_latency: Some(1),
            },
            ..Default::default()
        },
        dithering: false,
        ..Default::default()
    };
    eframe::run_native(
        "RCTGen2",
        options,
        Box::new(|creation_context| {
            creation_context.egui_ctx.set_theme(egui::Theme::Dark);
            Ok(Box::new(app::RctGen2App::new(
                &creation_context.egui_ctx,
                app_rx,
                render_tx,
                track_image,
                &data_directory,
                config_dir,
            )))
        }),
    )?;

    let _result = render_thread.join();

    Ok(())
}
