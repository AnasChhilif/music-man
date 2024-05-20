use id3::{Tag, TagLike};

pub struct Info {
    pub artist: String,
    pub title: String,
    pub album: String,
}

impl Info {
    pub fn new() -> Info {
        Info {
            artist: String::new(),
            title: String::new(),
            album: String::new(),
        }
    }

    pub fn fill_info(&mut self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let tag = Tag::read_from_path(file_path)?;
        if let Some(artist) = tag.artist() {
            self.artist = artist.to_owned();
        }
        if let Some(title) = tag.title() {
            self.title = title.to_owned();
        }
        if let Some(album) = tag.album() {
            self.album = album.to_owned();
        }
        Ok(())
    }
}
