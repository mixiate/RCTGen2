pub struct FilePath(std::path::PathBuf);

impl FilePath {
    fn try_new(file_path: std::path::PathBuf) -> anyhow::Result<Self> {
        use anyhow::Context as _;
        file_path.parent().context("Could not get parent directory")?;
        Ok(FilePath(file_path))
    }

    pub fn directory(&self) -> &std::path::Path {
        self.0.parent().unwrap()
    }
}

impl std::ops::Deref for FilePath {
    type Target = std::path::Path;
    #[inline]
    fn deref(&self) -> &std::path::Path {
        std::path::Path::new(&self.0)
    }
}

pub struct Track {
    pub file_path: FilePath,
    pub desc: make_track::track_desc::Desc,
    pub track_index: usize,
}

impl Track {
    pub fn load(file_path: std::path::PathBuf) -> anyhow::Result<Track> {
        let file_path = FilePath::try_new(file_path)?;
        let desc = make_track::track_desc::Desc::load(&file_path)?;
        Ok(Track {
            file_path,
            desc,
            track_index: 0,
        })
    }

    pub fn open() -> anyhow::Result<Option<Track>> {
        if let Some(file_path) = rfd::FileDialog::new().add_filter("json", &["json"]).pick_file() {
            Ok(Some(Track::load(file_path)?))
        } else {
            Ok(None)
        }
    }
}
