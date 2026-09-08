#![windows_subsystem = "windows"]

mod adjacent_track;
mod app;
mod drawing;
mod file_watcher;
mod render;
mod settings;
mod sprite_cache;
mod start_screen;
mod track_editor;
mod ui;

use eframe::egui;

fn main() -> anyhow::Result<()> {
    use anyhow::Context as _;

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
            creation_context.egui_ctx.options_mut(|options| options.sync_window_theme = false);
            creation_context.egui_ctx.set_theme(egui::Theme::Dark);
            Ok(Box::new(app::RctGen2App::new(data_directory, config_dir)))
        }),
    )?;

    Ok(())
}
