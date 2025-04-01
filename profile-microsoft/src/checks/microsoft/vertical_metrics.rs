use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert};

#[check(
    id = "microsoft/vertical_metrics",
    rationale = "
        
        If OS/2.fsSelection.useTypoMetrics is not set, then
            hhea.ascender == OS/2.winAscent
            hhea.descender == OS/2.winDescent
            hhea.lineGap == 0
    
    ",
    proposal = "https://github.com/fonttools/fontbakery/pull/4657",
    title = "Checking hhea OS/2 vertical_metrics."
)]
fn vertical_metrics(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    let mut problems = vec![];
    let metrics = f.vertical_metrics()?;
    if f.use_typo_metrics()? {
        if metrics.hhea_ascent != metrics.os2_typo_ascender {
            problems.push(Status::fail(
                "mismatch",
                &format!(
                    "hhea.ascent != OS/2.sTypoAscender: {} != {}",
                    metrics.hhea_ascent, metrics.os2_typo_ascender
                ),
            ));
        }
        if metrics.hhea_descent != metrics.os2_typo_descender {
            problems.push(Status::fail(
                "mismatch",
                &format!(
                    "hhea.descent != OS/2.sTypoDescender: {} != {}",
                    metrics.hhea_descent, metrics.os2_typo_descender
                ),
            ));
        }
        if metrics.hhea_linegap != metrics.os2_typo_linegap {
            problems.push(Status::fail(
                "mismatch",
                &format!(
                    "hhea.lineGap != OS/2.sTypoLineGap: {} != {}",
                    metrics.hhea_linegap, metrics.os2_typo_linegap
                ),
            ));
        }
    } else {
        if metrics.hhea_ascent as u16 != metrics.os2_win_ascent {
            problems.push(Status::fail(
                "mismatch",
                &format!(
                    "hhea.ascent != OS/2.usWinAscent: {} != {}",
                    metrics.hhea_ascent, metrics.os2_win_ascent
                ),
            ));
        }
        if metrics.hhea_descent.unsigned_abs() != metrics.os2_win_descent {
            problems.push(Status::fail(
                "mismatch",
                &format!(
                    "hhea.descent != OS/2.usWinDescent: {} != {}",
                    metrics.hhea_descent, metrics.os2_win_descent
                ),
            ));
        }
    }
    return_result(problems)
}
