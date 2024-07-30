use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

pub struct AudioFileController {
    audio_file_folder: PathBuf,
}

impl AudioFileController {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<AudioFileController, io::Error> {
        let audio_file_folder = path.as_ref().to_path_buf();

        // If the directory doesn't exist, create it
        if !audio_file_folder.exists() {
            fs::create_dir_all(&audio_file_folder)?;
        } else if !audio_file_folder.is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                "Path exists but is not a directory",
            ));
        }

        Ok(AudioFileController { audio_file_folder })
    }

    pub fn get_audio_files(&self) -> io::Result<Vec<String>> {
        let mut audio_files = Vec::new();

        for entry in fs::read_dir(&self.audio_file_folder)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                if let Some(file_name) = path.file_name() {
                    if let Some(file_name_str) = file_name.to_str() {
                        audio_files.push(file_name_str.to_string());
                    }
                }
            }
        }

        Ok(audio_files)
    }

    pub fn receive_audio_file(&self, file_name: &str, content: &[u8]) -> io::Result<()> {
        let file_path = self.audio_file_folder.join(file_name);
        let mut file = fs::File::create(file_path)?;
        file.write_all(content)?;
        Ok(())
    }

    pub fn delete_audio_file(&self, file_name: &str) -> io::Result<()> {
        let file_path = self.audio_file_folder.join(file_name);
        fs::remove_file(file_path)
    }

    pub fn file_exists(&self, file_name: &str) -> bool {
        self.audio_file_folder.join(file_name).exists()
    }

    pub fn get_file_path(&self, file_name: &str) -> PathBuf {
        self.audio_file_folder.join(file_name)
    }
}
