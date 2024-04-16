use super::FetchSource;
use crate::display::icons;
use std::path::PathBuf;
use sysinfo::{DiskExt, System, SystemExt};

const GB: u64 = 1024u64.pow(3);

pub struct Disk {
    size: u64,
    fstype: String,
}

impl FetchSource for Disk {
    fn info(&self) -> String {
        format!("{}GB ({})", self.size, self.fstype)
    }

    fn icon(&self) -> char {
        icons::DISK
    }

    fn text_prefix(&self) -> String {
        "root".into()
    }
}

impl Disk {
    pub fn get(system: &System) -> Self {
        let root_disk = system
            .disks()
            .iter()
            .find(|d| d.mount_point() == PathBuf::from("/"))
            .expect("There has to be a root partition!");
        let size = root_disk.total_space() / GB;
        let fstype = String::from_utf8_lossy(root_disk.file_system()).to_string();
        Self { size, fstype }
    }
}
