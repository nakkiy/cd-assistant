use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct DirEntry {
    pub name: String,
    pub path: PathBuf,
    pub expanded: bool,
    pub children: Vec<DirEntry>,
    pub is_loaded: bool,
    pub link_status: LinkStatus,
    pub link_target: Option<PathBuf>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinkStatus {
    Normal,
    SymlinkOk,
    SymlinkBroken,
}

impl DirEntry {
    pub fn new(path: PathBuf) -> Self {
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "/".to_string());

        let (link_status, link_target) = match fs::symlink_metadata(&path) {
            Ok(meta) => {
                if meta.file_type().is_symlink() {
                    let target = path.read_link().ok();
                    let status = match fs::metadata(&path) {
                        Ok(target_meta) => {
                            if target_meta.is_dir() {
                                LinkStatus::SymlinkOk
                            } else {
                                LinkStatus::Normal
                            }
                        }
                        Err(_) => LinkStatus::SymlinkBroken,
                    };
                    (status, target)
                } else {
                    (LinkStatus::Normal, None)
                }
            }
            Err(_) => (LinkStatus::Normal, None),
        };

        Self {
            name,
            path,
            expanded: false,
            children: vec![],
            is_loaded: false,
            link_status,
            link_target,
        }
    }

    pub fn load_children(&mut self) {
        if self.is_loaded {
            return;
        }
        self.children = match fs::read_dir(&self.path) {
            Ok(entries) => entries
                .filter_map(|e| e.ok())
                .filter(|e| e.path().is_dir())
                .map(|e| DirEntry::new(e.path()))
                .collect(),
            Err(_) => vec![],
        };
        self.children.sort_by_key(|c| c.name.clone());
        self.is_loaded = true;
    }

    pub fn load_only(&mut self, target: &Path) -> Option<usize> {
        self.children = match fs::read_dir(&self.path) {
            Ok(entries) => entries
                .filter_map(|e| e.ok())
                .filter(|e| e.path().is_dir())
                .map(|e| DirEntry::new(e.path()))
                .filter(|e| target.starts_with(&e.path))
                .collect(),
            Err(_) => vec![],
        };
        self.children.sort_by_key(|c| c.name.clone());
        self.is_loaded = false;
        self.children.first().map(|_| 0)
    }

    pub fn collapse_all(&mut self) {
        self.expanded = false;
        for child in self.children.iter_mut() {
            child.collapse_all();
        }
    }
}
