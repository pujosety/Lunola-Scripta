//! Cross-application clipboard compatibility for the Lunola Creative Suite.
//!
//! Provides utilities to detect clipboard formats from other apps and determine
//! whether content can be shared between Lunola apps and external applications.

/// A clipboard format that can be detected or produced.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ClipboardFormat {
    PlainText,
    Html,
    Rtf,
    Png,
    Jpeg,
    Svg,
    Pdf,
    Docx,
    Pptx,
    Xlsx,
    LunolaNative,
}

impl ClipboardFormat {
    pub fn mime_type(self) -> &'static str {
        match self {
            ClipboardFormat::PlainText => "text/plain",
            ClipboardFormat::Html => "text/html",
            ClipboardFormat::Rtf => "application/rtf",
            ClipboardFormat::Png => "image/png",
            ClipboardFormat::Jpeg => "image/jpeg",
            ClipboardFormat::Svg => "image/svg+xml",
            ClipboardFormat::Pdf => "application/pdf",
            ClipboardFormat::Docx => "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
            ClipboardFormat::Pptx => "application/vnd.openxmlformats-officedocument.presentationml.presentation",
            ClipboardFormat::Xlsx => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
            ClipboardFormat::LunolaNative => "application/x-lunola",
        }
    }

    pub fn display_name(self) -> &'static str {
        match self {
            ClipboardFormat::PlainText => "Plain Text",
            ClipboardFormat::Html => "HTML",
            ClipboardFormat::Rtf => "Rich Text",
            ClipboardFormat::Png => "PNG Image",
            ClipboardFormat::Jpeg => "JPEG Image",
            ClipboardFormat::Svg => "SVG Image",
            ClipboardFormat::Pdf => "PDF",
            ClipboardFormat::Docx => "Word Document",
            ClipboardFormat::Pptx => "Presentation",
            ClipboardFormat::Xlsx => "Spreadsheet",
            ClipboardFormat::LunolaNative => "Lunola Native",
        }
    }

    pub fn from_mime(mime: &str) -> Option<ClipboardFormat> {
        let m = mime.to_ascii_lowercase();
        match m.as_str() {
            "text/plain" => Some(ClipboardFormat::PlainText),
            "text/html" => Some(ClipboardFormat::Html),
            "application/rtf" | "text/rtf" => Some(ClipboardFormat::Rtf),
            "image/png" => Some(ClipboardFormat::Png),
            "image/jpeg" => Some(ClipboardFormat::Jpeg),
            "image/svg+xml" => Some(ClipboardFormat::Svg),
            "application/pdf" => Some(ClipboardFormat::Pdf),
            "application/vnd.openxmlformats-officedocument.wordprocessingml.document" => Some(ClipboardFormat::Docx),
            "application/vnd.openxmlformats-officedocument.presentationml.presentation" => Some(ClipboardFormat::Pptx),
            "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet" => Some(ClipboardFormat::Xlsx),
            "application/x-lunola" => Some(ClipboardFormat::LunolaNative),
            _ => None,
        }
    }

    pub fn from_extension(ext: &str) -> Option<ClipboardFormat> {
        let e = ext.to_ascii_lowercase();
        match e.as_str() {
            "txt" | "md" => Some(ClipboardFormat::PlainText),
            "html" | "htm" => Some(ClipboardFormat::Html),
            "rtf" => Some(ClipboardFormat::Rtf),
            "png" => Some(ClipboardFormat::Png),
            "jpg" | "jpeg" => Some(ClipboardFormat::Jpeg),
            "svg" => Some(ClipboardFormat::Svg),
            "pdf" => Some(ClipboardFormat::Pdf),
            "docx" | "doc" => Some(ClipboardFormat::Docx),
            "pptx" | "potx" | "ppsx" => Some(ClipboardFormat::Pptx),
            "xlsx" | "xls" | "csv" => Some(ClipboardFormat::Xlsx),
            _ => None,
        }
    }
}

/// Returns a human-readable description of a clipboard format.
pub fn clipboard_format_description(format: ClipboardFormat) -> String {
    format.display_name().to_string()
}

/// Whether content in this format can be pasted between Lunola apps and external applications.
pub fn is_cross_app_compatible(format: ClipboardFormat) -> bool {
    matches!(
        format,
        ClipboardFormat::PlainText
            | ClipboardFormat::Html
            | ClipboardFormat::Rtf
            | ClipboardFormat::Png
            | ClipboardFormat::Jpeg
            | ClipboardFormat::Pdf
    )
}

/// Detect the most useful clipboard format from a list of MIME type strings.
pub fn detect_best_format(mimes: &[String]) -> Option<ClipboardFormat> {
    // Priority order: HTML > RTF > PlainText > images > PDF
    for mime in mimes {
        if let Some(fmt) = ClipboardFormat::from_mime(mime) {
            if matches!(fmt, ClipboardFormat::Html) {
                return Some(fmt);
            }
        }
    }
    for mime in mimes {
        if let Some(fmt) = ClipboardFormat::from_mime(mime) {
            if matches!(fmt, ClipboardFormat::Rtf) {
                return Some(fmt);
            }
        }
    }
    for mime in mimes {
        if let Some(fmt) = ClipboardFormat::from_mime(mime) {
            if matches!(fmt, ClipboardFormat::PlainText) {
                return Some(fmt);
            }
        }
    }
    for mime in mimes {
        if let Some(fmt) = ClipboardFormat::from_mime(mime) {
            if matches!(fmt, ClipboardFormat::Png | ClipboardFormat::Jpeg) {
                return Some(fmt);
            }
        }
    }
    for mime in mimes {
        if let Some(fmt) = ClipboardFormat::from_mime(mime) {
            if matches!(fmt, ClipboardFormat::Pdf) {
                return Some(fmt);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mime_types_round_trip() {
        for fmt in [
            ClipboardFormat::PlainText,
            ClipboardFormat::Html,
            ClipboardFormat::Rtf,
            ClipboardFormat::Png,
            ClipboardFormat::Jpeg,
            ClipboardFormat::Svg,
            ClipboardFormat::Pdf,
            ClipboardFormat::Docx,
            ClipboardFormat::Pptx,
            ClipboardFormat::Xlsx,
            ClipboardFormat::LunolaNative,
        ] {
            let mime = fmt.mime_type();
            assert_eq!(ClipboardFormat::from_mime(mime), Some(fmt), "mime: {mime}");
        }
    }

    #[test]
    fn extensions_map_to_formats() {
        assert_eq!(ClipboardFormat::from_extension("txt"), Some(ClipboardFormat::PlainText));
        assert_eq!(ClipboardFormat::from_extension("html"), Some(ClipboardFormat::Html));
        assert_eq!(ClipboardFormat::from_extension("rtf"), Some(ClipboardFormat::Rtf));
        assert_eq!(ClipboardFormat::from_extension("png"), Some(ClipboardFormat::Png));
        assert_eq!(ClipboardFormat::from_extension("jpg"), Some(ClipboardFormat::Jpeg));
        assert_eq!(ClipboardFormat::from_extension("pdf"), Some(ClipboardFormat::Pdf));
        assert_eq!(ClipboardFormat::from_extension("docx"), Some(ClipboardFormat::Docx));
        assert_eq!(ClipboardFormat::from_extension("pptx"), Some(ClipboardFormat::Pptx));
        assert_eq!(ClipboardFormat::from_extension("xlsx"), Some(ClipboardFormat::Xlsx));
        assert_eq!(ClipboardFormat::from_extension("xyz"), None);
    }

    #[test]
    fn cross_app_compatibility() {
        assert!(is_cross_app_compatible(ClipboardFormat::PlainText));
        assert!(is_cross_app_compatible(ClipboardFormat::Html));
        assert!(is_cross_app_compatible(ClipboardFormat::Rtf));
        assert!(is_cross_app_compatible(ClipboardFormat::Png));
        assert!(is_cross_app_compatible(ClipboardFormat::Jpeg));
        assert!(is_cross_app_compatible(ClipboardFormat::Pdf));
        assert!(!is_cross_app_compatible(ClipboardFormat::Docx));
        assert!(!is_cross_app_compatible(ClipboardFormat::Pptx));
        assert!(!is_cross_app_compatible(ClipboardFormat::Xlsx));
        assert!(!is_cross_app_compatible(ClipboardFormat::LunolaNative));
    }

    #[test]
    fn detect_best_format_prefers_html() {
        let mimes = vec!["text/plain".to_string(), "text/html".to_string()];
        assert_eq!(detect_best_format(&mimes), Some(ClipboardFormat::Html));
    }

    #[test]
    fn detect_best_format_falls_back_to_plain() {
        let mimes = vec!["text/plain".to_string()];
        assert_eq!(detect_best_format(&mimes), Some(ClipboardFormat::PlainText));
    }

    #[test]
    fn detect_best_format_empty_list() {
        assert_eq!(detect_best_format(&[]), None);
    }
}
