//! Lunola ecosystem shared features: unified launcher, recent files dashboard,
//! template gallery, and cross-application clipboard compatibility.
//!
//! This module provides a consistent experience across all three Lunola Creative Suite
//! apps (wordcraft, deckcraft, gridcraft). It adds:
//!
//! - A launcher screen accessible from the Backstage that lists all three apps
//! - An enhanced recent files dashboard with file type icons and filtering
//! - A template gallery with search and category filtering
//! - Cross-application clipboard format detection and conversion utilities

pub mod clipboard;
pub mod launcher;
pub mod recent_dashboard;
pub mod template_gallery;

pub use clipboard::{ClipboardFormat, clipboard_format_description, is_cross_app_compatible};
pub use launcher::{AppId, app_description, app_name, launch_app};
pub use recent_dashboard::{RecentEntry, recent_entries_filtered};
pub use template_gallery::{filter_templates_by_category, template_category_label};

use serde::{Deserialize, Serialize};

/// The Lunola Creative Suite apps that can be launched.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SuiteApp {
    Wordcraft,
    Deckcraft,
    Gridcraft,
}

impl SuiteApp {
    pub const ALL: [SuiteApp; 3] = [SuiteApp::Wordcraft, SuiteApp::Deckcraft, SuiteApp::Gridcraft];

    pub fn id(self) -> &'static str {
        match self {
            SuiteApp::Wordcraft => "wordcraft",
            SuiteApp::Deckcraft => "deckcraft",
            SuiteApp::Gridcraft => "gridcraft",
        }
    }

    pub fn display_name(self) -> &'static str {
        match self {
            SuiteApp::Wordcraft => "Lunola Scripta",
            SuiteApp::Deckcraft => "Lunola Veyra",
            SuiteApp::Gridcraft => "Lunola Cella",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            SuiteApp::Wordcraft => "Word processing and document editing",
            SuiteApp::Deckcraft => "Presentations and slides",
            SuiteApp::Gridcraft => "Spreadsheets and data",
        }
    }

    pub fn icon(self) -> &'static str {
        match self {
            SuiteApp::Wordcraft => "docText",
            SuiteApp::Deckcraft => "slide",
            SuiteApp::Gridcraft => "book",
        }
    }

    pub fn file_extensions(self) -> &'static [&'static str] {
        match self {
            SuiteApp::Wordcraft => &["docx", "doc", "rtf", "txt", "md"],
            SuiteApp::Deckcraft => &["pptx", "deckcraft", "potx", "ppsx"],
            SuiteApp::Gridcraft => &["xlsx", "csv", "xls"],
        }
    }

    pub fn from_id(id: &str) -> Option<SuiteApp> {
        Self::ALL.into_iter().find(|a| a.id() == id)
    }
}

/// A template entry in the gallery.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GalleryTemplate {
    pub name: String,
    pub description: String,
    pub category: TemplateCategory,
    pub app: SuiteApp,
}

/// Categories for organizing templates across apps.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TemplateCategory {
    Blank,
    Business,
    Personal,
    Education,
    Creative,
    Sample,
}

impl TemplateCategory {
    pub const ALL: [TemplateCategory; 6] = [
        TemplateCategory::Blank,
        TemplateCategory::Business,
        TemplateCategory::Personal,
        TemplateCategory::Education,
        TemplateCategory::Creative,
        TemplateCategory::Sample,
    ];

    pub fn id(self) -> &'static str {
        match self {
            TemplateCategory::Blank => "blank",
            TemplateCategory::Business => "business",
            TemplateCategory::Personal => "personal",
            TemplateCategory::Education => "education",
            TemplateCategory::Creative => "creative",
            TemplateCategory::Sample => "sample",
        }
    }

    pub fn from_id(id: &str) -> Option<TemplateCategory> {
        Self::ALL.into_iter().find(|c| c.id() == id)
    }
}
