use crate::track_editor::TrackEditorMessage;
use eframe::egui;
use std::sync::mpsc::Sender;

pub struct FileWatcher {
    watcher: notify::RecommendedWatcher,
    directory: Option<std::path::PathBuf>,
}

impl FileWatcher {
    pub fn try_new(app_tx: Sender<TrackEditorMessage>, egui_context: egui::Context) -> notify::Result<FileWatcher> {
        let watcher = notify::recommended_watcher(move |result: notify::Result<notify::Event>| {
            if let Ok(event) = result
                && event.kind.is_modify()
                && event.paths.iter().any(|path| {
                    path.extension()
                        .and_then(|ext| ext.to_str().map(|ext| matches!(ext, "obj" | "mtl")))
                        .unwrap_or(false)
                })
            {
                let _result = app_tx.send(TrackEditorMessage::ModelFileChanged);
                egui_context.request_repaint();
            }
        })?;
        Ok(FileWatcher {
            watcher,
            directory: None,
        })
    }

    pub fn set_directory(&mut self, directory: &std::path::Path) -> notify::Result<()> {
        use notify::Watcher as _;

        if let Some(directory) = &self.directory {
            self.watcher.unwatch(directory)?;
        }
        self.watcher.watch(directory, notify::RecursiveMode::Recursive)?;
        self.directory = Some(directory.to_path_buf());
        Ok(())
    }
}
