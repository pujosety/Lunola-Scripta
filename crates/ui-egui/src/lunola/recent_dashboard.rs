//! Recent files dashboard for the Lunola Creative Suite.
//!
//! Provides a shared data structure and filtering logic for recent files entries
//! across all three apps. Each entry tracks the file path, which app it belongs to,
//! when it was last accessed, and its file type.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::lunola::{ClipboardFormat, SuiteApp};

/// A single entry in the recent files dashboard.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RecentEntry {
    /// The file path (or identifier for web-saved files).
    pub path: String,
    /// The display file name.
    pub name: String,
    /// Which Lunola app this file belongs to.
    pub app: SuiteApp,
    /// When the file was last accessed (unix timestamp in seconds).
    pub last_accessed: f64,
    /// The detected clipboard format / file type.
    pub format: Option<ClipboardFormat>,
    /// Whether the file is known to exist on disk.
    pub exists: bool,
}

impl RecentEntry {
    /// Create a new recent entry from a path and app.
    pub fn new(path: impl Into<String>, app: SuiteApp) -> Self {
        let path = path.into();
        let name = std::path::Path::new(&path).file_name().and_then(|n| n.to_str()).unwrap_or(&path).to_string();
        let ext = std::path::Path::new(&path).extension().and_then(|e| e.to_str()).unwrap_or("");
        let format = ClipboardFormat::from_extension(ext);
        Self { path, name, app, last_accessed: 0.0, format, exists: true }
    }

    /// The icon name for this entry's app.
    pub fn icon(&self) -> &'static str {
        self.app.icon()
    }

    /// A formatted label showing which app this belongs to.
    pub fn app_label(&self) -> String {
        format!("{}", self.app.display_name())
    }
}

/// Filter recent entries by app, search query, and/or format.
pub fn recent_entries_filtered(entries: &[RecentEntry], app_filter: Option<SuiteApp>, query: &str) -> Vec<RecentEntry> {
    let q = query.trim().to_lowercase();
    entries
        .iter()
        .filter(|e| {
            if let Some(app) = app_filter {
                if e.app != app {
                    return false;
                }
            }
            if !q.is_empty() {
                let name_match = e.name.to_lowercase().contains(&q);
                let path_match = e.path.to_lowercase().contains(&q);
                if !name_match && !path_match {
                    return false;
                }
            }
            true
        })
        .cloned()
        .collect()
}

/// Group entries by their app, returning a map from app to sorted entries (most recent first).
pub fn group_by_app(entries: &[RecentEntry]) -> HashMap<SuiteApp, Vec<RecentEntry>> {
    let mut map: HashMap<SuiteApp, Vec<RecentEntry>> = HashMap::new();
    for entry in entries {
        map.entry(entry.app).or_default().push(entry.clone());
    }
    for list in map.values_mut() {
        list.sort_by(|a, b| b.last_accessed.partial_cmp(&a.last_accessed).unwrap_or(std::cmp::Ordering::Equal));
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recent_entry_new_extracts_name_and_format() {
        let entry = RecentEntry::new("/home/me/doc.docx", SuiteApp::Wordcraft);
        assert_eq!(entry.name, "doc.docx");
        assert_eq!(entry.app, SuiteApp::Wordcraft);
        assert_eq!(entry.format, Some(ClipboardFormat::Docx));
    }

    #[test]
    fn recent_entry_for_spreadsheet() {
        let entry = RecentEntry::new("C:/Users/me/budget.xlsx", SuiteApp::Gridcraft);
        assert_eq!(entry.name, "budget.xlsx");
        assert_eq!(entry.format, Some(ClipboardFormat::Xlsx));
    }

    #[test]
    fn filter_by_app() {
        let entries = vec![
            RecentEntry::new("/a.docx", SuiteApp::Wordcraft),
            RecentEntry::new("/b.pptx", SuiteApp::Deckcraft),
            RecentEntry::new("/c.xlsx", SuiteApp::Gridcraft),
        ];
        let filtered = recent_entries_filtered(&entries, Some(SuiteApp::Wordcraft), "");
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].name, "a.docx");
    }

    #[test]
    fn filter_by_query() {
        let entries = vec![
            RecentEntry::new("/home/letter.docx", SuiteApp::Wordcraft),
            RecentEntry::new("/home/report.docx", SuiteApp::Wordcraft),
            RecentEntry::new("/home/budget.xlsx", SuiteApp::Gridcraft),
        ];
        let filtered = recent_entries_filtered(&entries, None, "budget");
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].name, "budget.xlsx");
    }

    #[test]
    fn group_by_app_sorts_by_recent() {
        let mut a = RecentEntry::new("/a.docx", SuiteApp::Wordcraft);
        a.last_accessed = 100.0;
        let mut b = RecentEntry::new("/b.docx", SuiteApp::Wordcraft);
        b.last_accessed = 200.0;
        let entries = vec![a.clone(), b.clone()];
        let map = group_by_app(&entries);
        let wordcraft_entries = map.get(&SuiteApp::Wordcraft).unwrap();
        assert_eq!(wordcraft_entries[0].path, "/b.docx");
        assert_eq!(wordcraft_entries[1].path, "/a.docx");
    }
}
