use std::fs;
use std::fs::{read_dir, remove_file};
use std::path::{Path, PathBuf};

use eyre::Result;

use crate::dirs::TRACKED_CONFIGS;
use crate::file::{create_dir_all, make_symlink_or_file};
use crate::hash::hash_to_str;

pub struct Tracker {}

impl Tracker {
    pub fn track(path: &Path) -> Result<()> {
        let tracking_path = TRACKED_CONFIGS.join(hash_to_str(&path));
        if !tracking_path.exists() {
            create_dir_all(&*TRACKED_CONFIGS)?;
            make_symlink_or_file(path, &tracking_path)?;
        }
        Ok(())
    }

    pub fn list_all() -> Result<Vec<PathBuf>> {
        let mut output = vec![];
        if !TRACKED_CONFIGS.exists() {
            return Ok(output);
        }
        for path in read_dir(&*TRACKED_CONFIGS)? {
            let mut path = path?.path();
            if path.is_symlink() {
                path = fs::read_link(path)?;
            } else if cfg!(target_os = "windows") {
                path = PathBuf::from(fs::read_to_string(&path)?.trim());
            } else {
                continue;
            }
            if path.exists() {
                output.push(path);
            }
        }
        Ok(output)
    }

    pub fn clean() -> Result<()> {
        if TRACKED_CONFIGS.is_dir() {
            for path in read_dir(&*TRACKED_CONFIGS)? {
                let path = path?.path();
                if !path.exists() {
                    remove_file(&path)?;
                }
            }
        }
        Ok(())
    }
}
