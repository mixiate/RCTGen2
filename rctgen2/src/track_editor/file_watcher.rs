use eframe::egui;

enum Message {
    ModelFileChanged,
}

pub enum Status {
    Delay(std::time::Duration),
    ReloadModels,
}

pub struct FileWatcher {
    watcher: notify::RecommendedWatcher,
    directory: Option<std::path::PathBuf>,
    rx: std::sync::mpsc::Receiver<Message>,
    model_file_changed_time: Option<std::time::Instant>,
}

impl FileWatcher {
    pub fn try_new(egui_context: egui::Context) -> notify::Result<FileWatcher> {
        let (tx, rx) = std::sync::mpsc::channel::<Message>();

        let watcher = notify::recommended_watcher(move |result: notify::Result<notify::Event>| {
            if let Ok(event) = result
                && event.kind.is_modify()
                && event.paths.iter().any(|path| {
                    path.extension()
                        .and_then(|ext| ext.to_str().map(|ext| matches!(ext, "obj" | "mtl")))
                        .unwrap_or(false)
                })
            {
                let _result = tx.send(Message::ModelFileChanged);
                egui_context.request_repaint();
            }
        })?;
        Ok(FileWatcher {
            watcher,
            directory: None,
            rx,
            model_file_changed_time: None,
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

    pub fn check(&mut self) -> Option<Status> {
        for message in self.rx.try_iter() {
            match message {
                Message::ModelFileChanged => self.model_file_changed_time = Some(std::time::Instant::now()),
            }
        }

        if let Some(time) = self.model_file_changed_time {
            if let Some(time_left) = std::time::Duration::from_millis(250).checked_sub(time.elapsed()) {
                Some(Status::Delay(time_left))
            } else {
                self.model_file_changed_time = None;
                Some(Status::ReloadModels)
            }
        } else {
            None
        }
    }
}
