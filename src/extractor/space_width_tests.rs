//! Which code-32 width becomes a simple font's space width
//! (`code_32_space_width`): the yardstick every `TJ` word gap of the font
//! is judged against.

use super::*;
use lopdf::dictionary;

/// Proportional advances, distinct between neighbouring codes.
fn proportional(code: u16) -> i64 {
    380 + i64::from(code) * 37 % 300
}

/// A Type 1 font with widths for codes 1..=126 from `width`, plus `extra`
/// entries (an `/Encoding`, a descriptor).
fn font(
    width: impl Fn(u16) -> i64,
    extra: Vec<(&'static str, Object)>,
) -> (Document, lopdf::Dictionary) {
    let doc = Document::with_version("1.4");
    let mut font = dictionary! {
        "Type" => "Font",
        "Subtype" => "Type1",
        "BaseFont" => "ABCDEF+Probe",
        "FirstChar" => 1,
        "LastChar" => 126,
        "Widths" => (1..=126u16).map(|code| Object::Integer(width(code))).collect::<Vec<_>>(),
    };
    for (key, value) in extra {
        font.set(key, value);
    }
    (doc, font)
}

/// An `/Encoding` dictionary with these `/Differences`: a number starts a
/// run of codes, each name takes the next code.
fn differences(entries: &[&str]) -> (&'static str, Object) {
    let array = entries
        .iter()
        .map(|entry| match entry.parse::<i64>() {
            Ok(code) => Object::Integer(code),
            Err(_) => Object::Name(entry.as_bytes().to_vec()),
        })
        .collect::<Vec<_>>();
    (
        "Encoding",
        Object::Dictionary(dictionary! {
            "Type" => "Encoding",
            "Differences" => array,
        }),
    )
}

fn space_width(doc: &Document, font: &lopdf::Dictionary) -> u16 {
    parse_simple_font_widths(doc, font)
        .expect("width table")
        .space_width
}

#[test]
fn a_subset_in_its_own_order_takes_its_space_from_the_glyph_named_space() {
    // Elsevier's AdvTimes: letters from code 1 on, `K` at 32 (822 wide).
    let width = |code| match code {
        32 => 822,
        4 => 250,
        other => proportional(other),
    };
    let (doc, f) = font(
        width,
        vec![differences(&["1", "T", "h", "e", "space", "32", "K"])],
    );
    assert_eq!(space_width(&doc, &f), 250);
    // Without a space glyph the caller's estimate stands in.
    let (doc, f) = font(width, vec![differences(&["1", "T", "h", "e", "32", "K"])]);
    assert_eq!(space_width(&doc, &f), 250);
    // A width under the 0.45 em placeholder floor is replaced as well: the
    // `K` is no space whatever its width.
    let (doc, f) = font(
        |code| if code == 32 { 400 } else { proportional(code) },
        vec![differences(&["1", "T", "h", "e", "32", "K"])],
    );
    assert_eq!(space_width(&doc, &f), 250);
}

#[test]
fn of_two_space_glyphs_the_lowest_code_sets_the_width() {
    let width = |code| match code {
        4 => 200,
        9 => 300,
        32 => 822,
        other => proportional(other),
    };
    let (doc, f) = font(
        width,
        vec![differences(&[
            "1", "T", "4", "space", "9", "space", "32", "K",
        ])],
    );
    assert_eq!(space_width(&doc, &f), 200);
}

#[test]
fn a_code_32_that_reads_as_a_space_keeps_its_width() {
    let width = |code| if code == 32 { 278 } else { proportional(code) };
    for name in ["space", "uni0020", "nbspace"] {
        let (doc, f) = font(width, vec![differences(&["1", "T", "h", "32", name])]);
        assert_eq!(space_width(&doc, &f), 278, "{name}");
    }
}

#[test]
fn tex_math_fonts_keep_their_own_glyph_at_32() {
    // CMMI's ψ at 32 (651), alone or with a TeX-order `/Differences`
    // (letters at their ASCII codes), and CMSY's ← → pair (1000 each):
    // their offsets are math spacing, which the glyph's width keeps from
    // reading as word gaps.
    let psi = |code| if code == 32 { 651 } else { proportional(code) };
    let (doc, f) = font(psi, vec![]);
    assert_eq!(space_width(&doc, &f), 651);
    let (doc, f) = font(psi, vec![differences(&["32", "psi", "97", "a", "b"])]);
    assert_eq!(space_width(&doc, &f), 651);
    let arrows = |code| match code {
        32 | 33 => 1000,
        other => proportional(other),
    };
    let (doc, f) = font(arrows, vec![]);
    assert_eq!(space_width(&doc, &f), 1000);
    // A math extension font names no letters at all.
    let (doc, f) = font(
        |code| if code == 32 { 792 } else { proportional(code) },
        vec![differences(&["32", "parenleftBigg"])],
    );
    assert_eq!(space_width(&doc, &f), 792);
}

#[test]
fn a_placeholder_width_filling_unused_codes_is_no_space() {
    // JAMA's GuardianSansGR: 497 for codes 5–36, the space among them.
    let jama = |code| {
        if (5..=36).contains(&code) {
            497
        } else {
            proportional(code)
        }
    };
    let (doc, f) = font(jama, vec![]);
    assert_eq!(space_width(&doc, &f), 250);
    // A run as long, but narrower than any placeholder, is kept.
    let (doc, f) = font(
        |code| {
            if (5..=36).contains(&code) {
                300
            } else {
                proportional(code)
            }
        },
        vec![],
    );
    assert_eq!(space_width(&doc, &f), 300);
}

#[test]
fn a_short_run_of_equal_widths_is_glyphs() {
    // Tabular digits of a subset in its own order (Arial: 556 for five
    // codes up to 32).
    let (doc, f) = font(
        |code| {
            if (28..=32).contains(&code) {
                556
            } else {
                proportional(code)
            }
        },
        vec![],
    );
    assert_eq!(space_width(&doc, &f), 556);
}

#[test]
fn a_monospaced_font_keeps_its_wide_space() {
    let (doc, f) = font(|_| 600, vec![]);
    assert_eq!(space_width(&doc, &f), 600);
    // The advances alone do not say monospaced (one glyph differs), but
    // the descriptor's FixedPitch flag does; without the flag the run of
    // 600s is a placeholder.
    let widths = |code| match code {
        10..=40 => 600,
        65 => 500,
        _ => 0,
    };
    let mut doc = Document::with_version("1.4");
    let descriptor = doc.add_object(dictionary! {
        "Type" => "FontDescriptor",
        "FontName" => "ABCDEF+Probe",
        "Flags" => 1 | 32,
    });
    let (_, mut f) = font(widths, vec![]);
    f.set("FontDescriptor", descriptor);
    assert_eq!(space_width(&doc, &f), 600);
    let (doc, f) = font(widths, vec![]);
    assert_eq!(space_width(&doc, &f), 250);
}

#[test]
fn a_replaced_code_32_width_is_kept_for_column_gaps() {
    let jama = |code| {
        if (5..=36).contains(&code) {
            497
        } else {
            proportional(code)
        }
    };
    let (doc, f) = font(jama, vec![]);
    let info = parse_simple_font_widths(&doc, &f).expect("width table");
    assert_eq!((info.space_width, info.code_32_width), (250, Some(497)));
    // A code 32 that stays the space reports nothing extra.
    let (doc, f) = font(
        |code| if code == 32 { 651 } else { proportional(code) },
        vec![],
    );
    let info = parse_simple_font_widths(&doc, &f).expect("width table");
    assert_eq!((info.space_width, info.code_32_width), (651, None));
}

#[test]
fn a_type3_font_keeps_its_own_units() {
    // Glyph space of one unit per text-space unit: a space of 3 units is
    // three em, and no 1000-unit rule applies.
    let (doc, mut f) = font(
        |code| if code == 32 { 3 } else { 1 },
        vec![differences(&["1", "T", "h", "e", "32", "K"])],
    );
    f.set("Subtype", "Type3");
    f.set(
        "FontMatrix",
        vec![1.into(), 0.into(), 0.into(), 1.into(), 0.into(), 0.into()],
    );
    assert_eq!(space_width(&doc, &f), 3);
}
