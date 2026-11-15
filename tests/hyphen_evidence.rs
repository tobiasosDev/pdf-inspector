//! PDF-path policy regressions. Gold expectations precede implementation.
use pdf_inspector::process_pdf_mem;
#[test]
fn hyphen_evidence_unknown() {
    // Independent English/German PDF gold; no document witness. Accepted limitation: unwitnessed re-pair → repair, re-fuse → refuse, re-sent → resent; joined outputs intentionally record the rare meaning-changing coinage limitation, not successful disambiguation. Preserved: re-cover, re-sign, re-form have both closed and hyphenated lexicon entries, so lexical joining is vetoed. This class depends on the lexicon's hyphenated entries. Open: German Unter-suchung and other unwitnessed German syllable breaks stay split; no German word source exists in fork or repo.
    let md = process_pdf_mem(include_bytes!("fixtures/hyphen-evidence-unknown.pdf"))
        .unwrap()
        .markdown
        .unwrap();
    assert!(
        md.contains("We must repair the devices after reset."),
        "unexpected PDF output: {md}"
    );
    assert!(
        md.contains("The technician will refuse the electrical panel."),
        "unexpected PDF output: {md}"
    );
    assert!(
        md.contains("The invitation was resent yesterday."),
        "unexpected PDF output: {md}"
    );
    assert!(
        md.contains("Case 4: re- cover remains in this example."),
        "unexpected PDF output: {md}"
    );
    assert!(
        md.contains("Case 5: re- sign remains in this example."),
        "unexpected PDF output: {md}"
    );
    assert!(
        md.contains("Case 6: re- form remains in this example."),
        "unexpected PDF output: {md}"
    );
    assert!(
        md.contains("Die Unter- suchung geht weiter."),
        "unexpected PDF output: {md}"
    );
    assert!(
        md.contains("Case 8: Kraft- fahrzeugversicherung remains in this example."),
        "unexpected PDF output: {md}"
    );
    assert!(
        md.contains("Case 9: Wach- stube remains in this example."),
        "unexpected PDF output: {md}"
    );
    assert!(
        md.contains("Case 10: Druck- erzeugnis remains in this example."),
        "unexpected PDF output: {md}"
    );
    assert!(
        md.contains("Case 11: Stau- becken remains in this example."),
        "unexpected PDF output: {md}"
    );
    assert!(
        !md.contains("We must re- pair the devices after reset."),
        "unexpected PDF output: {md}"
    );
    assert!(
        !md.contains("The technician will re- fuse the electrical panel."),
        "unexpected PDF output: {md}"
    );
    assert!(
        !md.contains("The invitation was re- sent yesterday."),
        "unexpected PDF output: {md}"
    );
    assert!(
        !md.contains("Case 4: recover remains in this example."),
        "unexpected PDF output: {md}"
    );
    assert!(
        !md.contains("Case 5: resign remains in this example."),
        "unexpected PDF output: {md}"
    );
    assert!(
        !md.contains("Case 6: reform remains in this example."),
        "unexpected PDF output: {md}"
    );
    assert!(
        !md.contains("Die Untersuchung geht weiter."),
        "unexpected PDF output: {md}"
    );
    assert!(
        !md.contains("Case 8: Kraftfahrzeugversicherung remains in this example."),
        "unexpected PDF output: {md}"
    );
    assert!(
        !md.contains("Case 9: Wachstube remains in this example."),
        "unexpected PDF output: {md}"
    );
    assert!(
        !md.contains("Case 10: Druckerzeugnis remains in this example."),
        "unexpected PDF output: {md}"
    );
    assert!(
        !md.contains("Case 11: Staubecken remains in this example."),
        "unexpected PDF output: {md}"
    );
}
#[test]
fn hyphen_evidence_plain() {
    // Independent English/German PDF gold; policy: U+00AD joins first; document hard spelling wins over closed spelling; otherwise English lexical evidence joins only a known closed word with no known hyphenated spelling. Mode plain.
    let md = process_pdf_mem(include_bytes!("fixtures/hyphen-evidence-plain.pdf"))
        .unwrap()
        .markdown
        .unwrap();
    assert!(
        md.contains("We must repair the devices after reset."),
        "unexpected PDF output: {md}"
    );
    assert!(
        md.contains("The technician will refuse the electrical panel."),
        "unexpected PDF output: {md}"
    );
    assert!(
        md.contains("The invitation was resent yesterday."),
        "unexpected PDF output: {md}"
    );
    assert!(
        md.contains("Case 4: recover remains in this example."),
        "unexpected PDF output: {md}"
    );
    assert!(
        md.contains("Case 5: resign remains in this example."),
        "unexpected PDF output: {md}"
    );
    assert!(
        md.contains("Case 6: reform remains in this example."),
        "unexpected PDF output: {md}"
    );
    assert!(
        md.contains("Die Untersuchung geht weiter."),
        "unexpected PDF output: {md}"
    );
    assert!(
        md.contains("Case 8: Kraftfahrzeugversicherung remains in this example."),
        "unexpected PDF output: {md}"
    );
    assert!(
        md.contains("Case 9: Wachstube remains in this example."),
        "unexpected PDF output: {md}"
    );
    assert!(
        md.contains("Case 10: Druckerzeugnis remains in this example."),
        "unexpected PDF output: {md}"
    );
    assert!(
        md.contains("Case 11: Staubecken remains in this example."),
        "unexpected PDF output: {md}"
    );
    assert!(
        !md.contains("We must re- pair the devices after reset."),
        "unexpected PDF output: {md}"
    );
    assert!(
        !md.contains("The technician will re- fuse the electrical panel."),
        "unexpected PDF output: {md}"
    );
    assert!(
        !md.contains("The invitation was re- sent yesterday."),
        "unexpected PDF output: {md}"
    );
    assert!(
        !md.contains("Case 4: re- cover remains in this example."),
        "unexpected PDF output: {md}"
    );
    assert!(
        !md.contains("Case 5: re- sign remains in this example."),
        "unexpected PDF output: {md}"
    );
    assert!(
        !md.contains("Case 6: re- form remains in this example."),
        "unexpected PDF output: {md}"
    );
    assert!(
        !md.contains("Die Unter- suchung geht weiter."),
        "unexpected PDF output: {md}"
    );
    assert!(
        !md.contains("Case 8: Kraft- fahrzeugversicherung remains in this example."),
        "unexpected PDF output: {md}"
    );
    assert!(
        !md.contains("Case 9: Wach- stube remains in this example."),
        "unexpected PDF output: {md}"
    );
    assert!(
        !md.contains("Case 10: Druck- erzeugnis remains in this example."),
        "unexpected PDF output: {md}"
    );
    assert!(
        !md.contains("Case 11: Stau- becken remains in this example."),
        "unexpected PDF output: {md}"
    );
}
#[test]
fn hyphen_evidence_hyphen() {
    // Independent English/German PDF gold; policy: U+00AD joins first; document hard spelling wins over closed spelling; otherwise English lexical evidence joins only a known closed word with no known hyphenated spelling. Mode hyphen.
    let md = process_pdf_mem(include_bytes!("fixtures/hyphen-evidence-hyphen.pdf"))
        .unwrap()
        .markdown
        .unwrap();
    assert!(
        md.contains("We must re-pair the devices after reset."),
        "unexpected PDF output: {md}"
    );
    assert!(
        md.contains("The technician will re-fuse the electrical panel."),
        "unexpected PDF output: {md}"
    );
    assert!(
        md.contains("The invitation was re-sent yesterday."),
        "unexpected PDF output: {md}"
    );
    assert!(
        md.contains("Case 4: re-cover remains in this example."),
        "unexpected PDF output: {md}"
    );
    assert!(
        md.contains("Case 5: re-sign remains in this example."),
        "unexpected PDF output: {md}"
    );
    assert!(
        md.contains("Case 6: re-form remains in this example."),
        "unexpected PDF output: {md}"
    );
    assert!(
        md.contains("Die Unter-suchung geht weiter."),
        "unexpected PDF output: {md}"
    );
    assert!(
        md.contains("Case 8: Kraft-fahrzeugversicherung remains in this example."),
        "unexpected PDF output: {md}"
    );
    assert!(
        md.contains("Case 9: Wach-stube remains in this example."),
        "unexpected PDF output: {md}"
    );
    assert!(
        md.contains("Case 10: Druck-erzeugnis remains in this example."),
        "unexpected PDF output: {md}"
    );
    assert!(
        md.contains("Case 11: Stau-becken remains in this example."),
        "unexpected PDF output: {md}"
    );
    assert!(
        !md.contains("We must repair the devices after reset."),
        "unexpected PDF output: {md}"
    );
    assert!(
        !md.contains("The technician will refuse the electrical panel."),
        "unexpected PDF output: {md}"
    );
    assert!(
        !md.contains("The invitation was resent yesterday."),
        "unexpected PDF output: {md}"
    );
    assert!(
        !md.contains("Case 4: recover remains in this example."),
        "unexpected PDF output: {md}"
    );
    assert!(
        !md.contains("Case 5: resign remains in this example."),
        "unexpected PDF output: {md}"
    );
    assert!(
        !md.contains("Case 6: reform remains in this example."),
        "unexpected PDF output: {md}"
    );
    assert!(
        !md.contains("Die Untersuchung geht weiter."),
        "unexpected PDF output: {md}"
    );
    assert!(
        !md.contains("Case 8: Kraftfahrzeugversicherung remains in this example."),
        "unexpected PDF output: {md}"
    );
    assert!(
        !md.contains("Case 9: Wachstube remains in this example."),
        "unexpected PDF output: {md}"
    );
    assert!(
        !md.contains("Case 10: Druckerzeugnis remains in this example."),
        "unexpected PDF output: {md}"
    );
    assert!(
        !md.contains("Case 11: Staubecken remains in this example."),
        "unexpected PDF output: {md}"
    );
}
#[test]
fn hyphen_evidence_conflict() {
    // Independent English/German PDF gold; policy: U+00AD joins first; document hard spelling wins over closed spelling; otherwise English lexical evidence joins only a known closed word with no known hyphenated spelling. Mode conflict.
    let md = process_pdf_mem(include_bytes!("fixtures/hyphen-evidence-conflict.pdf"))
        .unwrap()
        .markdown
        .unwrap();
    assert!(
        md.contains("We must re-pair the devices after reset."),
        "unexpected PDF output: {md}"
    );
    assert!(
        md.contains("The technician will re-fuse the electrical panel."),
        "unexpected PDF output: {md}"
    );
    assert!(
        md.contains("The invitation was re-sent yesterday."),
        "unexpected PDF output: {md}"
    );
    assert!(
        md.contains("Case 4: re-cover remains in this example."),
        "unexpected PDF output: {md}"
    );
    assert!(
        md.contains("Case 5: re-sign remains in this example."),
        "unexpected PDF output: {md}"
    );
    assert!(
        md.contains("Case 6: re-form remains in this example."),
        "unexpected PDF output: {md}"
    );
    assert!(
        md.contains("Die Unter-suchung geht weiter."),
        "unexpected PDF output: {md}"
    );
    assert!(
        md.contains("Case 8: Kraft-fahrzeugversicherung remains in this example."),
        "unexpected PDF output: {md}"
    );
    assert!(
        md.contains("Case 9: Wach-stube remains in this example."),
        "unexpected PDF output: {md}"
    );
    assert!(
        md.contains("Case 10: Druck-erzeugnis remains in this example."),
        "unexpected PDF output: {md}"
    );
    assert!(
        md.contains("Case 11: Stau-becken remains in this example."),
        "unexpected PDF output: {md}"
    );
    assert!(
        !md.contains("We must repair the devices after reset."),
        "unexpected PDF output: {md}"
    );
    assert!(
        !md.contains("The technician will refuse the electrical panel."),
        "unexpected PDF output: {md}"
    );
    assert!(
        !md.contains("The invitation was resent yesterday."),
        "unexpected PDF output: {md}"
    );
    assert!(
        !md.contains("Case 4: recover remains in this example."),
        "unexpected PDF output: {md}"
    );
    assert!(
        !md.contains("Case 5: resign remains in this example."),
        "unexpected PDF output: {md}"
    );
    assert!(
        !md.contains("Case 6: reform remains in this example."),
        "unexpected PDF output: {md}"
    );
    assert!(
        !md.contains("Die Untersuchung geht weiter."),
        "unexpected PDF output: {md}"
    );
    assert!(
        !md.contains("Case 8: Kraftfahrzeugversicherung remains in this example."),
        "unexpected PDF output: {md}"
    );
    assert!(
        !md.contains("Case 9: Wachstube remains in this example."),
        "unexpected PDF output: {md}"
    );
    assert!(
        !md.contains("Case 10: Druckerzeugnis remains in this example."),
        "unexpected PDF output: {md}"
    );
    assert!(
        !md.contains("Case 11: Staubecken remains in this example."),
        "unexpected PDF output: {md}"
    );
}
#[test]
fn hyphen_evidence_soft() {
    // Independent English/German PDF gold; policy: U+00AD joins first; document hard spelling wins over closed spelling; otherwise English lexical evidence joins only a known closed word with no known hyphenated spelling. Mode soft.
    let md = process_pdf_mem(include_bytes!("fixtures/hyphen-evidence-soft.pdf"))
        .unwrap()
        .markdown
        .unwrap();
    assert!(
        md.contains("We must repair the devices after reset."),
        "unexpected PDF output: {md}"
    );
    assert!(
        md.contains("The technician will refuse the electrical panel."),
        "unexpected PDF output: {md}"
    );
    assert!(
        md.contains("The invitation was resent yesterday."),
        "unexpected PDF output: {md}"
    );
    assert!(
        md.contains("Case 4: recover remains in this example."),
        "unexpected PDF output: {md}"
    );
    assert!(
        md.contains("Case 5: resign remains in this example."),
        "unexpected PDF output: {md}"
    );
    assert!(
        md.contains("Case 6: reform remains in this example."),
        "unexpected PDF output: {md}"
    );
    assert!(
        md.contains("Die Untersuchung geht weiter."),
        "unexpected PDF output: {md}"
    );
    assert!(
        md.contains("Case 8: Kraftfahrzeugversicherung remains in this example."),
        "unexpected PDF output: {md}"
    );
    assert!(
        md.contains("Case 9: Wachstube remains in this example."),
        "unexpected PDF output: {md}"
    );
    assert!(
        md.contains("Case 10: Druckerzeugnis remains in this example."),
        "unexpected PDF output: {md}"
    );
    assert!(
        md.contains("Case 11: Staubecken remains in this example."),
        "unexpected PDF output: {md}"
    );
    assert!(
        !md.contains("We must re- pair the devices after reset."),
        "unexpected PDF output: {md}"
    );
    assert!(
        !md.contains("The technician will re- fuse the electrical panel."),
        "unexpected PDF output: {md}"
    );
    assert!(
        !md.contains("The invitation was re- sent yesterday."),
        "unexpected PDF output: {md}"
    );
    assert!(
        !md.contains("Case 4: re- cover remains in this example."),
        "unexpected PDF output: {md}"
    );
    assert!(
        !md.contains("Case 5: re- sign remains in this example."),
        "unexpected PDF output: {md}"
    );
    assert!(
        !md.contains("Case 6: re- form remains in this example."),
        "unexpected PDF output: {md}"
    );
    assert!(
        !md.contains("Die Unter- suchung geht weiter."),
        "unexpected PDF output: {md}"
    );
    assert!(
        !md.contains("Case 8: Kraft- fahrzeugversicherung remains in this example."),
        "unexpected PDF output: {md}"
    );
    assert!(
        !md.contains("Case 9: Wach- stube remains in this example."),
        "unexpected PDF output: {md}"
    );
    assert!(
        !md.contains("Case 10: Druck- erzeugnis remains in this example."),
        "unexpected PDF output: {md}"
    );
    assert!(
        !md.contains("Case 11: Stau- becken remains in this example."),
        "unexpected PDF output: {md}"
    );
}
