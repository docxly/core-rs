use crate::utils::xml_helper::escape_text;

pub(super) const SECTION_PREAMBLE_BODY: &str = concat!(
    "<hp:secPr id=\"\" textDirection=\"HORIZONTAL\" spaceColumns=\"1134\" tabStop=\"8000\" tabStopVal=\"4000\" tabStopUnit=\"HWPUNIT\" outlineShapeIDRef=\"1\" memoShapeIDRef=\"0\" textVerticalWidthHead=\"0\" masterPageCnt=\"0\">",
    "<hp:grid lineGrid=\"0\" charGrid=\"0\" wonggojiFormat=\"0\"/>",
    "<hp:startNum pageStartsOn=\"BOTH\" page=\"0\" pic=\"0\" tbl=\"0\" equation=\"0\"/>",
    "<hp:visibility hideFirstHeader=\"0\" hideFirstFooter=\"0\" hideFirstMasterPage=\"0\" border=\"SHOW_ALL\" fill=\"SHOW_ALL\" hideFirstPageNum=\"0\" hideFirstEmptyLine=\"0\" showLineNumber=\"0\"/>",
    "<hp:lineNumberShape restartType=\"0\" countBy=\"0\" distance=\"0\" startNumber=\"0\"/>",
    "<hp:pagePr landscape=\"WIDELY\" width=\"59528\" height=\"84186\" gutterType=\"LEFT_ONLY\"><hp:margin header=\"4252\" footer=\"4252\" gutter=\"0\" left=\"8504\" right=\"8504\" top=\"5668\" bottom=\"4252\"/></hp:pagePr>",
    "<hp:footNotePr><hp:autoNumFormat type=\"DIGIT\" userChar=\"\" prefixChar=\"\" suffixChar=\")\" supscript=\"0\"/><hp:noteLine length=\"-1\" type=\"SOLID\" width=\"0.12 mm\" color=\"#000000\"/><hp:noteSpacing betweenNotes=\"283\" belowLine=\"567\" aboveLine=\"850\"/><hp:numbering type=\"CONTINUOUS\" newNum=\"1\"/><hp:placement place=\"EACH_COLUMN\" beneathText=\"0\"/></hp:footNotePr>",
    "<hp:endNotePr><hp:autoNumFormat type=\"DIGIT\" userChar=\"\" prefixChar=\"\" suffixChar=\")\" supscript=\"0\"/><hp:noteLine length=\"14692344\" type=\"SOLID\" width=\"0.12 mm\" color=\"#000000\"/><hp:noteSpacing betweenNotes=\"0\" belowLine=\"567\" aboveLine=\"850\"/><hp:numbering type=\"CONTINUOUS\" newNum=\"1\"/><hp:placement place=\"END_OF_DOCUMENT\" beneathText=\"0\"/></hp:endNotePr>",
    "<hp:pageBorderFill type=\"BOTH\" borderFillIDRef=\"1\" textBorder=\"PAPER\" headerInside=\"0\" footerInside=\"0\" fillArea=\"PAPER\"><hp:offset left=\"1417\" right=\"1417\" top=\"1417\" bottom=\"1417\"/></hp:pageBorderFill>",
    "<hp:pageBorderFill type=\"EVEN\" borderFillIDRef=\"1\" textBorder=\"PAPER\" headerInside=\"0\" footerInside=\"0\" fillArea=\"PAPER\"><hp:offset left=\"1417\" right=\"1417\" top=\"1417\" bottom=\"1417\"/></hp:pageBorderFill>",
    "<hp:pageBorderFill type=\"ODD\" borderFillIDRef=\"1\" textBorder=\"PAPER\" headerInside=\"0\" footerInside=\"0\" fillArea=\"PAPER\"><hp:offset left=\"1417\" right=\"1417\" top=\"1417\" bottom=\"1417\"/></hp:pageBorderFill>",
    "</hp:secPr><hp:ctrl><hp:colPr id=\"\" type=\"NEWSPAPER\" layout=\"LEFT\" colCount=\"1\" sameSz=\"1\" sameGap=\"0\"/></hp:ctrl>"
);

pub(super) struct HyperlinkFieldSpec<'a> {
    pub(super) char_pr: u32,
    pub(super) field_begin_id: u64,
    pub(super) field_id: u64,
    pub(super) command: &'a str,
    pub(super) path: &'a str,
    pub(super) text: &'a str,
    pub(super) trailing_empty_text: bool,
}

pub(super) fn render_section_preamble_run(char_pr: u32) -> String {
    format!(
        "<hp:run charPrIDRef=\"{char_pr}\">{body}</hp:run>",
        char_pr = char_pr,
        body = SECTION_PREAMBLE_BODY,
    )
}

pub(super) fn hyperlink_command(path: &str) -> String {
    format!("{};1;0;0;", path.replace("://", "\\://"))
}

pub(super) fn compat_hyperlink_path(url: &str) -> String {
    if url.ends_with('/') {
        url.to_string()
    } else {
        format!("{url}/")
    }
}

pub(super) fn render_hyperlink_run(spec: HyperlinkFieldSpec<'_>) -> String {
    format!(
        concat!(
            "<hp:run charPrIDRef=\"{char_pr}\">",
            "<hp:ctrl><hp:fieldBegin id=\"{begin_id}\" type=\"HYPERLINK\" name=\"\" editable=\"0\" dirty=\"1\" zorder=\"-1\" fieldid=\"{field_id}\">",
            "<hp:parameters cnt=\"6\" name=\"\">",
            "<hp:integerParam name=\"Prop\">0</hp:integerParam>",
            "<hp:stringParam name=\"Command\">{command}</hp:stringParam>",
            "<hp:stringParam name=\"Path\">{path}</hp:stringParam>",
            "<hp:stringParam name=\"Category\">HWPHYPERLINK_TYPE_URL</hp:stringParam>",
            "<hp:stringParam name=\"TargetType\">HWPHYPERLINK_TARGET_BOOKMARK</hp:stringParam>",
            "<hp:stringParam name=\"DocOpenType\">HWPHYPERLINK_JUMP_CURRENTTAB</hp:stringParam>",
            "</hp:parameters></hp:fieldBegin></hp:ctrl>",
            "<hp:t>{text}</hp:t>",
            "<hp:ctrl><hp:fieldEnd beginIDRef=\"{begin_id}\" fieldid=\"{field_id}\"/></hp:ctrl>",
            "{trailing}",
            "</hp:run>"
        ),
        char_pr = spec.char_pr,
        begin_id = spec.field_begin_id,
        field_id = spec.field_id,
        command = escape_text(spec.command),
        path = escape_text(spec.path),
        text = escape_text(spec.text),
        trailing = if spec.trailing_empty_text {
            "<hp:t/>"
        } else {
            ""
        },
    )
}
