mod app;
mod render;

use eframe::egui;
use std::sync::{Arc, Mutex};

fn main() -> anyhow::Result<()> {
    let (render_tx, render_rx) = std::sync::mpsc::channel();
    let (app_tx, app_rx) = std::sync::mpsc::channel();
    let render_texture = Arc::new(Mutex::new(None));

    let render_thread = {
        let render_texture = render_texture.clone();
        std::thread::spawn(move || render::render_thread(&render_rx, &app_tx, &render_texture))
    };

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_min_inner_size([800.0, 600.0]),
        centered: true,
        wgpu_options: egui_wgpu::WgpuConfiguration {
            desired_maximum_frame_latency: Some(1),
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
            Ok(Box::new(app::RctGen2App::new(app_rx, render_tx, render_texture)))
        }),
    )?;

    let _result = render_thread.join();

    Ok(())
}
