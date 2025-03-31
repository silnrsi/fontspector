use fontspector_checkapi::{prelude::*, testfont, FileTypeConvert};

#[check(
    id = "name_id_1",
    rationale = "
        
        Presence of a name ID 1 entry is mandatory.
    
    ",
    proposal = "https://github.com/fonttools/fontbakery/pull/4657",
    title = "Font has a name with ID 1."
)]
fn name_id_1(t: &Testable, _context: &Context) -> CheckFnResult {
    let f = testfont!(t);
    Ok(if f.best_familyname().is_none() {
        Status::just_one_fail("nameid2-missing", "Name ID 1 (family) missing.")
    } else {
        Status::just_one_pass()
    })
}
