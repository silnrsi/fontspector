use fontspector_checkapi::{prelude::*, skip, testfont, FileTypeConvert};

#[check(
    id = "fontbureau/ytlc_sanity",
    rationale = "
         This check follows the values of the ytlc axis proposed by Font Bureau.
    ",
    proposal = "https://github.com/fonttools/fontbakery/issues/3130",
    title = "Check if ytlc values are sane in vf"
)]
fn ytlc_sanity(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    skip!(!f.is_variable_font(), "not-a-vf", "Not a variable font");
    if let Some((_, min, _, max)) = f.axis_ranges().find(|(name, _, _, _)| name == "ytlc") {
        if min < 0.0 || max > 1000.0 {
            return Ok(Status::just_one_fail("invalid_range", &format!(
                "The range of ytlc values ({min} - {max}) does not conform to the expected range of ytlc which should be min value 0 to max value 1000",
            )));
        }
    }
    Ok(Status::just_one_pass())
}
