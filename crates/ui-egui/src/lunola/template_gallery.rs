//! Template gallery for the Lunola Creative Suite.
//!
//! Provides a shared template gallery UI with search and category filtering.
//! Templates from all three apps are shown in a unified gallery.

use egui::{RichText, Sense, Ui};

use crate::WordApp;
use crate::lunola::{GalleryTemplate, SuiteApp, TemplateCategory};
use crate::theme::{Tokens, medium, regular};

/// Returns the display label for a template category.
pub fn template_category_label(category: TemplateCategory) -> &'static str {
    match category {
        TemplateCategory::Blank => "Blank",
        TemplateCategory::Business => "Business",
        TemplateCategory::Personal => "Personal",
        TemplateCategory::Education => "Education",
        TemplateCategory::Creative => "Creative",
        TemplateCategory::Sample => "Sample",
    }
}

/// Filter templates by category and search query.
pub fn filter_templates_by_category(templates: &[GalleryTemplate], category: Option<TemplateCategory>, query: &str) -> Vec<GalleryTemplate> {
    let q = query.trim().to_lowercase();
    templates
        .iter()
        .filter(|t| {
            if let Some(cat) = category {
                if t.category != cat {
                    return false;
                }
            }
            if !q.is_empty() {
                let name_match = t.name.to_lowercase().contains(&q);
                let desc_match = t.description.to_lowercase().contains(&q);
                if !name_match && !desc_match {
                    return false;
                }
            }
            true
        })
        .cloned()
        .collect()
}

/// Draws the template gallery UI.
pub fn show_template_gallery(app: &mut WordApp, ui: &mut Ui) {
    let t = Tokens::get(ui.ctx());
    ui.label(RichText::new("Template Gallery").font(medium(22.0)).color(t.text));
    ui.add_space(8.0);

    // Category filter buttons
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing = egui::vec2(8.0, 0.0);
        for cat in TemplateCategory::ALL {
            let selected = false; // Would be tracked in app state
            let label = template_category_label(cat);
            let btn = ui.selectable_label(selected, label);
            if btn.clicked() {
                // Would update filter state
            }
        }
    });
    ui.add_space(16.0);

    // Template grid
    let templates = builtin_gallery_templates();
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing = egui::vec2(16.0, 16.0);
        for template in &templates {
            template_card(app, ui, template, &t);
        }
    });
}

fn template_card(_app: &mut WordApp, ui: &mut Ui, template: &GalleryTemplate, t: &Tokens) {
    let (rect, resp) = ui.allocate_exact_size(egui::vec2(200.0, 180.0), Sense::click());
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, true, &template.name));

    let p = ui.painter();
    let hovered = resp.hovered();
    let bg = if hovered { t.hover } else { t.panel };
    p.rect_filled(rect, 6.0, bg);
    p.rect_stroke(rect, 6.0, egui::Stroke::new(if hovered { 2.0 } else { 1.0 }, if hovered { t.accent } else { t.border }), egui::StrokeKind::Inside);

    // Template name
    let name_galley = p.layout(template.name.clone(), medium(13.0), t.text, rect.width() - 16.0);
    p.galley(egui::pos2(rect.center().x - name_galley.size().x / 2.0, rect.top() + 12.0), name_galley, t.text);

    // Description
    let desc_galley = p.layout(template.description.clone(), regular(11.0), t.text_dim, rect.width() - 16.0);
    p.galley(egui::pos2(rect.center().x - desc_galley.size().x / 2.0, rect.top() + 34.0), desc_galley, t.text_dim);

    // Category badge
    let cat_label = template_category_label(template.category);
    let cat_galley = p.layout(cat_label.to_string(), regular(10.0), t.accent, rect.width() - 16.0);
    p.galley(egui::pos2(rect.center().x - cat_galley.size().x / 2.0, rect.bottom() - 24.0), cat_galley, t.accent);

    if resp.clicked() {
        // Would open the template in the appropriate app
        let _ = template.app; // Used for routing
    }
}

/// Returns the built-in gallery templates across all apps.
pub fn builtin_gallery_templates() -> Vec<GalleryTemplate> {
    vec![
        GalleryTemplate {
            name: "Blank Document".into(),
            description: "Start with a clean page".into(),
            category: TemplateCategory::Blank,
            app: SuiteApp::Wordcraft,
        },
        GalleryTemplate {
            name: "Blank Presentation".into(),
            description: "Start with a blank slide".into(),
            category: TemplateCategory::Blank,
            app: SuiteApp::Deckcraft,
        },
        GalleryTemplate {
            name: "Blank Workbook".into(),
            description: "Start with a clean sheet".into(),
            category: TemplateCategory::Blank,
            app: SuiteApp::Gridcraft,
        },
        GalleryTemplate {
            name: "Business Letter".into(),
            description: "Professional letter template".into(),
            category: TemplateCategory::Business,
            app: SuiteApp::Wordcraft,
        },
        GalleryTemplate {
            name: "Quarterly Report".into(),
            description: "Business presentation template".into(),
            category: TemplateCategory::Business,
            app: SuiteApp::Deckcraft,
        },
        GalleryTemplate {
            name: "Budget Planner".into(),
            description: "Personal finance spreadsheet".into(),
            category: TemplateCategory::Business,
            app: SuiteApp::Gridcraft,
        },
        GalleryTemplate {
            name: "Resume".into(),
            description: "Professional CV template".into(),
            category: TemplateCategory::Personal,
            app: SuiteApp::Wordcraft,
        },
        GalleryTemplate {
            name: "Photo Album".into(),
            description: "Picture presentation template".into(),
            category: TemplateCategory::Personal,
            app: SuiteApp::Deckcraft,
        },
        GalleryTemplate {
            name: "Lesson Plan".into(),
            description: "Educational presentation".into(),
            category: TemplateCategory::Education,
            app: SuiteApp::Deckcraft,
        },
        GalleryTemplate {
            name: "Grade Book".into(),
            description: "Student grades spreadsheet".into(),
            category: TemplateCategory::Education,
            app: SuiteApp::Gridcraft,
        },
        GalleryTemplate {
            name: "Creative Portfolio".into(),
            description: "Showcase your work".into(),
            category: TemplateCategory::Creative,
            app: SuiteApp::Deckcraft,
        },
        GalleryTemplate {
            name: "Studio Handbook".into(),
            description: "Sample document with styles".into(),
            category: TemplateCategory::Sample,
            app: SuiteApp::Wordcraft,
        },
        GalleryTemplate {
            name: "Northern Lights".into(),
            description: "Sample presentation deck".into(),
            category: TemplateCategory::Sample,
            app: SuiteApp::Deckcraft,
        },
        GalleryTemplate {
            name: "Sales Data".into(),
            description: "Sample spreadsheet with charts".into(),
            category: TemplateCategory::Sample,
            app: SuiteApp::Gridcraft,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_by_category() {
        let templates = builtin_gallery_templates();
        let blank = filter_templates_by_category(&templates, Some(TemplateCategory::Blank), "");
        assert_eq!(blank.len(), 3);
        assert!(blank.iter().all(|t| t.category == TemplateCategory::Blank));
    }

    #[test]
    fn filter_by_query() {
        let templates = builtin_gallery_templates();
        let results = filter_templates_by_category(&templates, None, "budget");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Budget Planner");
    }

    #[test]
    fn filter_combined() {
        let templates = builtin_gallery_templates();
        let results = filter_templates_by_category(&templates, Some(TemplateCategory::Business), "letter");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Business Letter");
    }

    #[test]
    fn category_labels() {
        assert_eq!(template_category_label(TemplateCategory::Blank), "Blank");
        assert_eq!(template_category_label(TemplateCategory::Business), "Business");
        assert_eq!(template_category_label(TemplateCategory::Sample), "Sample");
    }
}
