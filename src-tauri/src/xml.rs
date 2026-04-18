
pub fn build_problem_xml(text: &str, is_bold: bool, hex_color: &str) -> String {
    let escape = |s: &str| {
        s.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&apos;")
    };

    let bold_attr = if is_bold { " bold=\"1\"" } else { "" };
    let color_attr = format!(" color=\"{}\"", hex_color);

    let mut tree = String::from("<r2dtotree><node name=\"1doc\">");

    for line in text.split('\n') {
        let content = if line.is_empty() { " " } else { line };
        tree.push_str(&format!(
            "<node name=\"1para\"><node name=\"1rtline\"><leaf name=\"1word\"{}{}>{}</leaf></node></node>",
            bold_attr,
            color_attr,
            escape(content)
        ));
    }

    tree.push_str("</node></r2dtotree>");
    let escaped_tree = escape(&tree);

    format!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<prob xmlns="urn:TI.Problem" ver="1.0" pbname="">
  <sym></sym>
  <card clay="0" h1="10000" h2="10000" w1="10000" w2="10000">
    <isDummyCard>0</isDummyCard>
    <wdgt xmlns:np="urn:TI.Notepad" type="TI.Notepad" ver="2.0">
      <np:mFlags>1024</np:mFlags>
      <np:value>3</np:value>
      <np:fmtxt>{}</np:fmtxt>
    </wdgt>
  </card>
</prob>"#,
        escaped_tree
    )
}