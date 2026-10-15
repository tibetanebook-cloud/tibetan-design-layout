//! Original editable Tibetan Pecha and book starting layouts.
use super::{CommandSpec, always, bad, cmd};
use crate::{Result, Session};
use serde_json::{Value, json};

struct Template {
    id: &'static str,
    name: &'static str,
    width_mm: f64,
    height_mm: f64,
    size: f64,
    bilingual: bool,
    border: bool,
}
const TEMPLATES: &[Template] = &[
    Template { id: "pechaLong", name: "Pecha — Long 420 × 100 mm", width_mm: 420.0, height_mm: 100.0, size: 28.0, bilingual: false, border: true },
    Template {
        id: "pechaClassic",
        name: "Pecha — Classic 300 × 100 mm",
        width_mm: 300.0,
        height_mm: 100.0,
        size: 27.0,
        bilingual: false,
        border: true,
    },
    Template {
        id: "pechaCompact",
        name: "Pecha — Compact 210 × 74 mm",
        width_mm: 210.0,
        height_mm: 74.0,
        size: 22.0,
        bilingual: false,
        border: true,
    },
    Template {
        id: "pechaLarge", name: "Pecha — Large 500 × 120 mm", width_mm: 500.0, height_mm: 120.0, size: 30.0, bilingual: false, border: true
    },
    Template {
        id: "pechaTitle",
        name: "Pecha — Title Leaf 300 × 100 mm",
        width_mm: 300.0,
        height_mm: 100.0,
        size: 30.0,
        bilingual: false,
        border: true,
    },
    Template {
        id: "pechaBilingual",
        name: "Pecha — Tibetan / English 300 × 140 mm",
        width_mm: 300.0,
        height_mm: 140.0,
        size: 24.0,
        bilingual: true,
        border: true,
    },
    Template {
        id: "pechaCommentary",
        name: "Pecha — Commentary 420 × 140 mm",
        width_mm: 420.0,
        height_mm: 140.0,
        size: 27.0,
        bilingual: true,
        border: true,
    },
    Template {
        id: "pechaModern",
        name: "Pecha — Unframed 300 × 100 mm",
        width_mm: 300.0,
        height_mm: 100.0,
        size: 27.0,
        bilingual: false,
        border: false,
    },
    Template { id: "bookA5", name: "Book — Tibetan Reading A5", width_mm: 148.0, height_mm: 210.0, size: 18.0, bilingual: false, border: false },
    Template { id: "book6x9", name: "Book — Paperback 6 × 9 in", width_mm: 152.4, height_mm: 228.6, size: 18.0, bilingual: false, border: false },
    Template { id: "bookBilingual", name: "Book — Bilingual A5", width_mm: 148.0, height_mm: 210.0, size: 17.0, bilingual: true, border: false },
    Template { id: "bookStudy", name: "Book — Study Workbook A4", width_mm: 210.0, height_mm: 297.0, size: 20.0, bilingual: false, border: false },
    Template {
        id: "bookPoetry", name: "Book — Poetry 5.5 × 8.5 in", width_mm: 139.7, height_mm: 215.9, size: 18.0, bilingual: false, border: false
    },
    Template {
        id: "bookIllustrated",
        name: "Book — Illustrated 8 × 10 in",
        width_mm: 203.2,
        height_mm: 254.0,
        size: 20.0,
        bilingual: false,
        border: false,
    },
    Template {
        id: "bookDictionary",
        name: "Book — Dictionary A4 Two Columns",
        width_mm: 210.0,
        height_mm: 297.0,
        size: 16.0,
        bilingual: true,
        border: false,
    },
    Template {
        id: "bookTitle", name: "Book — Title Page 6 × 9 in", width_mm: 152.4, height_mm: 228.6, size: 28.0, bilingual: false, border: false
    },
];
pub fn specs() -> Vec<CommandSpec> {
    vec![
        cmd!(noundo "template.pechaLong", "Pecha — Long 420 × 100 mm", ["File", "Template Library"], None, "{} — create an editable layout", always, |s, _| create(s, "pechaLong")),
        cmd!(noundo "template.pechaClassic", "Pecha — Classic 300 × 100 mm", ["File", "Template Library"], None, "{} — create an editable layout", always, |s, _| create(s, "pechaClassic")),
        cmd!(noundo "template.pechaCompact", "Pecha — Compact 210 × 74 mm", ["File", "Template Library"], None, "{} — create an editable layout", always, |s, _| create(s, "pechaCompact")),
        cmd!(noundo "template.pechaLarge", "Pecha — Large 500 × 120 mm", ["File", "Template Library"], None, "{} — create an editable layout", always, |s, _| create(s, "pechaLarge")),
        cmd!(noundo "template.pechaTitle", "Pecha — Title Leaf 300 × 100 mm", ["File", "Template Library"], None, "{} — create an editable layout", always, |s, _| create(s, "pechaTitle")),
        cmd!(noundo "template.pechaBilingual", "Pecha — Tibetan / English 300 × 140 mm", ["File", "Template Library"], None, "{} — create an editable layout", always, |s, _| create(s, "pechaBilingual")),
        cmd!(noundo "template.pechaCommentary", "Pecha — Commentary 420 × 140 mm", ["File", "Template Library"], None, "{} — create an editable layout", always, |s, _| create(s, "pechaCommentary")),
        cmd!(noundo "template.pechaModern", "Pecha — Unframed 300 × 100 mm", ["File", "Template Library"], None, "{} — create an editable layout", always, |s, _| create(s, "pechaModern")),
        cmd!(noundo "template.bookA5", "Book — Tibetan Reading A5", ["File", "Template Library"], None, "{} — create an editable layout", always, |s, _| create(s, "bookA5")),
        cmd!(noundo "template.book6x9", "Book — Paperback 6 × 9 in", ["File", "Template Library"], None, "{} — create an editable layout", always, |s, _| create(s, "book6x9")),
        cmd!(noundo "template.bookBilingual", "Book — Bilingual A5", ["File", "Template Library"], None, "{} — create an editable layout", always, |s, _| create(s, "bookBilingual")),
        cmd!(noundo "template.bookStudy", "Book — Study Workbook A4", ["File", "Template Library"], None, "{} — create an editable layout", always, |s, _| create(s, "bookStudy")),
        cmd!(noundo "template.bookPoetry", "Book — Poetry 5.5 × 8.5 in", ["File", "Template Library"], None, "{} — create an editable layout", always, |s, _| create(s, "bookPoetry")),
        cmd!(noundo "template.bookIllustrated", "Book — Illustrated 8 × 10 in", ["File", "Template Library"], None, "{} — create an editable layout", always, |s, _| create(s, "bookIllustrated")),
        cmd!(noundo "template.bookDictionary", "Book — Dictionary A4 Two Columns", ["File", "Template Library"], None, "{} — create an editable layout", always, |s, _| create(s, "bookDictionary")),
        cmd!(noundo "template.bookTitle", "Book — Title Page 6 × 9 in", ["File", "Template Library"], None, "{} — create an editable layout", always, |s, _| create(s, "bookTitle")),
    ]
}

fn text(s: &mut Session, rect: [f64; 4], content: &str, size: f64, tibetan: bool) -> Result<()> {
    s.execute("frame.create", &json!({"rect": rect, "content": "text", "text": content, "caret": false}))?;
    s.execute("type.char", &json!({"attrs": {"fontFamily": if tibetan { "Noto Serif Tibetan" } else { "Source Serif 4" }, "size": size, "leading": {"kind": "points", "value": size * 1.6}}}))?;
    s.execute("type.para", &json!({"attrs": {"hyphenate": false}}))?;
    Ok(())
}

fn create(s: &mut Session, id: &str) -> Result<Value> {
    let t = TEMPLATES.iter().find(|t| t.id == id).ok_or_else(|| bad("template", "unknown template"))?;
    let mm = 72.0 / 25.4;
    let (w, h) = (t.width_mm * mm, t.height_mm * mm);
    let pecha = id.starts_with("pecha");
    let margin = if pecha { 12.0 * mm } else { 18.0 * mm };
    let created = s.execute("file.new", &json!({"title": t.name, "width": w, "height": h, "pages": 1, "facingPages": false, "margins": margin}))?;
    if t.border {
        s.execute("frame.create", &json!({"rect": [margin - 4.0, margin - 4.0, w - margin + 4.0, h - margin + 4.0], "content": "none"}))?;
        s.execute("object.fill", &json!({"swatch": "[None]"}))?;
        s.execute("object.stroke", &json!({"swatch": "[Black]", "weight": 0.75}))?;
    }
    if id.ends_with("Title") {
        text(s, [margin, h * 0.3, w - margin, h * 0.7], "༄༅། །དཔེ་ཆའི་མཚན་བྱང་།", t.size, true)?;
        s.execute("type.para", &json!({"attrs": {"align": "center"}}))?;
    } else {
        let body = "༄༅། །བོད་ཡིག་འདིར་བྲིས།";
        if t.bilingual {
            let middle = (margin + h - margin) * 0.5;
            if id == "bookDictionary" {
                text(s, [margin, margin, w * 0.5 - 9.0, h - margin], body, t.size, true)?;
                text(s, [w * 0.5 + 9.0, margin, w - margin, h - margin], "English definitions here", 11.0, false)?;
            } else {
                text(s, [margin, margin, w - margin, middle - 9.0], body, t.size, true)?;
                if id == "pechaCommentary" {
                    text(s, [margin, middle + 9.0, w - margin, h - margin], "འགྲེལ་བཤད་འདིར་བྲིས།", 20.0, true)?;
                } else {
                    text(s, [margin, middle + 9.0, w - margin, h - margin], "English translation here", 12.0, false)?;
                }
            }
        } else if id == "bookIllustrated" {
            s.execute("frame.create", &json!({"rect": [margin, margin, w - margin, h * 0.6], "content": "graphic"}))?;
            s.execute("object.stroke", &json!({"swatch": "[Black]", "weight": 0.5}))?;
            text(s, [margin, h * 0.65, w - margin, h - margin], body, t.size, true)?;
        } else {
            text(s, [margin, margin, w - margin, h - margin], body, t.size, true)?;
        }
        if id == "bookStudy" {
            for line in 0..8 {
                let y = h * 0.42 + f64::from(line) * 36.0;
                s.execute("line.create", &json!({"a": [margin, y], "b": [w - margin, y]}))?;
                s.execute("object.stroke", &json!({"swatch": "[Black]", "weight": 0.35}))?;
            }
        }
        text(s, [margin, h - margin + 8.0, w - margin, h - 8.0], "༡", 10.0, true)?;
    }
    Ok(created)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn every_template_creates_editable_document_and_roundtrips() {
        for t in TEMPLATES {
            let mut s = Session::new();
            s.execute(&format!("template.{}", t.id), &json!({})).unwrap();
            let doc = &s.doc().unwrap().doc;
            assert_eq!(doc.title, t.name);
            assert!(!doc.stories.is_empty(), "{} has editable text", t.id);
            let bytes = crate::cmd::file::to_bytes(doc);
            let restored = crate::cmd::file::from_bytes(&bytes).unwrap();
            assert_eq!(restored.title, t.name);
            assert_eq!(restored.stories.len(), doc.stories.len());
        }
    }
    #[test]
    fn unknown_template_does_not_create_document() {
        let mut s = Session::new();
        assert!(create(&mut s, "invalid").is_err());
        assert!(s.active().is_none());
    }
}
