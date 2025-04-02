use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert};

const SUPPORTED_TABLES: [&str; 41] = [
    "avar", "BASE", "CFF ", "CFF2", "cmap", "cvar", "cvt ", "DSIG", "feat", "fpgm", "fvar", "gasp",
    "GDEF", "glyf", "GPOS", "GSUB", "gvar", "hdmx", "head", "hhea", "hmtx", "HVAR", "kern", "loca",
    "LTSH", "maxp", "meta", "morx", "MVAR", "name", "OS/2", "PCLT", "post", "prep", "STAT", "SVG ",
    "VDMX", "vhea", "vmtx", "VORG", "VVAR",
];

#[check(
    id = "adobefonts/unsupported_tables",
    rationale = "
        
        Adobe Fonts' font-processing pipeline does not support all kinds of tables
        that can be included in OpenType font files.⏎
        Fonts that do not pass this check are guaranteed to be rejected by the pipeline.
    
    ",
    proposal = "https://github.com/fonttools/fontbakery/pull/3870",
    title = "Does the font have any unsupported tables?"
)]
fn unsupported_tables(t: &Testable, context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let bad_tables = f
        .font()
        .table_directory
        .table_records()
        .iter()
        .map(|record| record.tag().to_string())
        .filter(|tag| !SUPPORTED_TABLES.contains(&tag.as_str()))
        .collect::<Vec<_>>();
    Ok(if bad_tables.is_empty() {
        Status::just_one_pass()
    } else {
        Status::just_one_fail(
            "unsupported-tables",
            &format!(
                "The following unsupported font tables were found:\n\n{}",
                bullet_list(context, bad_tables)
            ),
        )
    })
}
