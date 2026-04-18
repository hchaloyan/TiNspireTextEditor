
use crate::utils::filename::sanitize_filename;
use crate::xml::problem::build_problem_xml;

#[test]
fn xml_wraps_plain_text() {
    let xml = build_problem_xml("Hello, world!", false, "rgb(0,0,0)");
    assert!(xml.contains(r#"xmlns="urn:TI.Problem""#));
    assert!(xml.contains("<np:fmtxt>"));
    assert!(xml.contains("Hello, world!"));
}

#[test]
fn xml_escapes_special_chars() {
    let xml = build_problem_xml("a < b & c > d", false, "rgb(0,0,0)");
    assert!(xml.contains("a &amp;lt; b &amp;amp; c &amp;gt; d"));
}

#[test]
fn xml_splits_newlines_into_paragraphs() {
    let xml = build_problem_xml("line1\nline2", false, "rgb(0,0,0)");
    assert_eq!(xml.matches("1para").count(), 2);
}

#[test]
fn xml_has_correct_widget_version() {
    let xml = build_problem_xml("test", false, "rgb(0,0,0)");
    assert!(xml.contains(r#"ver="2.0""#));
    assert!(xml.contains("<np:mFlags>1024</np:mFlags>"));
    assert!(xml.contains("<np:value>3</np:value>"));
}

#[test]
fn sanitize_strips_illegal_chars() {
    assert_eq!(sanitize_filename("my/file:name"), "my_file_name");
    assert_eq!(sanitize_filename(""), "untitled");
    assert_eq!(sanitize_filename("normal"), "normal");
}