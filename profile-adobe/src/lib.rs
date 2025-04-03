#![deny(clippy::unwrap_used, clippy::expect_used)]
mod checks;

use fontspector_checkapi::{Override, ProfileBuilder, Registry, StatusCode};

pub struct Adobe;
impl fontspector_checkapi::Plugin for Adobe {
    fn register(&self, cr: &mut Registry) -> Result<(), String> {
        let builder = ProfileBuilder::new()
            .include_profile("universal")
            .add_section("Adobe Fonts Checks")
            // "Adobe Fonts Checks" = ["adobefonts/family/consistent_upm", "adobefonts/nameid_1_win_english", "adobefonts/unsupported_tables", "adobefonts/STAT_strings"]
            .add_and_register_check(checks::adobefonts::family::consistent_upm)
            .add_and_register_check(checks::adobefonts::nameid_1_win_english)
            .add_and_register_check(checks::adobefonts::unsupported_tables)
            .add_and_register_check(checks::adobefonts::STAT_strings)
            .exclude_check("opentype/xavgcharwidth")
            .exclude_check("designspace_has_consistent_codepoints")
            .exclude_check("designspace_has_consistent_glyphset")
            .exclude_check("designspace_has_consistent_groups")
            .exclude_check("designspace_has_default_master")
            .exclude_check("designspace_has_sources")
            .exclude_check("name/no_copyright_on_description")
            .exclude_check("ufolint")
            .exclude_check("ufo_features_default_languagesystem")
            .exclude_check("ufo_recommended_fields")
            .exclude_check("ufo_required_fields")
            .exclude_check("ufo_unnecessary_fields")
            .exclude_check("STAT_strings")
            .exclude_check("transformed_components")
            .exclude_check("unreachable_glyphs")
            .exclude_check("whitespace_ink")
            .with_overrides("whitespace_glyphs", vec![
                Override::new("missing-whitespace-glyph-0x00A0", StatusCode::Warn,  "For Adobe, this is not as severe as assessed in the original check for 0x00A0.")
            ]);
        builder.build("adobefonts", cr)
    }
}

pluginator::plugin_implementation!(fontspector_checkapi::Plugin, Adobe);
