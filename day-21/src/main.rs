use std::fs::{File, OpenOptions, remove_file};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct TempFile {
    pub file_path: PathBuf,
    pub file: File,
}

impl TempFile {
    pub fn new() -> Result<Self, std::io::Error> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_nanos();
        let random_name = format!("tempfile_{}", timestamp);
        let file_path = env::temp_dir().join(random_name);

        let file = File::create(&file_path)?;

        Ok(Self { file_path, file })
    }

    pub fn write(&mut self, data: &[u8]) -> Result<(), std::io::Error> {
        let mut file = OpenOptions::new().write(true).open(&self.file_path)?;
        file.write_all(data)
    }

    pub fn read_to_string(&mut self) -> Result<String, std::io::Error> {
        let mut file = OpenOptions::new().read(true).open(&self.file_path)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        Ok(buffer)
    }

    pub fn path(&self) -> &PathBuf {
        &self.file_path
    }

    pub fn file(&self) -> &File {
        &self.file
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        if let Err(err) = remove_file(&self.file_path) {
            eprintln!("Failed to delete temporary file {:?}: {}", self.file_path, err);
        }
    }
}

fn main() {

}
