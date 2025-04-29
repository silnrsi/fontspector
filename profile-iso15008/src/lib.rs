#![deny(clippy::unwrap_used, clippy::expect_used)]
mod checks;

use fontspector_checkapi::{ProfileBuilder, Registry};

pub struct Iso15008;
impl fontspector_checkapi::Plugin for Iso15008 {
    fn register(&self, cr: &mut Registry) -> Result<(), String> {
        let builder = ProfileBuilder::new()
            .include_profile("universal")
            .add_section("Iso15008 Fonts Checks")
            .add_and_register_check(checks::iso15008::intercharacter_spacing)
            .add_and_register_check(checks::iso15008::interline_spacing)
            .add_and_register_check(checks::iso15008::interword_spacing)
            .add_and_register_check(checks::iso15008::proportions)
            .add_and_register_check(checks::iso15008::stem_width);
        builder.build("iso15008", cr)
    }
}

#[cfg(not(target_family = "wasm"))]
pluginator::plugin_implementation!(fontspector_checkapi::Plugin, Iso15008);
