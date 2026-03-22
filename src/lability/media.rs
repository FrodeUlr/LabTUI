#![allow(dead_code)]
use crate::lability::config::MediaEntry;

/// Media repository – wraps the built-in list and custom entries
#[derive(Debug, Clone)]
pub struct MediaRepository {
    pub entries: Vec<MediaEntry>,
}

impl Default for MediaRepository {
    fn default() -> Self {
        Self {
            entries: crate::lability::config::default_media_list(),
        }
    }
}

impl MediaRepository {
    pub fn add(&mut self, entry: MediaEntry) {
        self.entries.push(entry);
    }

    pub fn find_by_id(&self, id: &str) -> Option<&MediaEntry> {
        self.entries.iter().find(|e| e.id == id)
    }

    pub fn ids(&self) -> Vec<&str> {
        self.entries.iter().map(|e| e.id.as_str()).collect()
    }
}
