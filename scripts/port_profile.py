from fontbakery.fonts_profile import profile_factory, get_module
import argparse

try:
    from tomlkit import comment, document, nl, table, array
except ImportError:
    print("Please install tomlkit: pip install tomlkit")
    exit(1)

parser = argparse.ArgumentParser()
parser.add_argument("--output", help="TOML file to write the profile to")
parser.add_argument("profile")

args = parser.parse_args()

if not args.output:
    output = args.profile.replace(".py", ".toml")
else:
    output = args.output

profile = getattr(get_module(args.profile), "PROFILE")


doc = document()
doc.add(comment("Ported from FontBakery profile " + args.profile))
if "include_profiles" in profile:
    doc.add(nl())
    doc.add(comment("Included profiles"))
    doc.add("include_profiles", profile["include_profiles"])
excluded = profile.get("exclude_checks", [])
if excluded:
    doc.add(nl())
    doc.add(comment("Excluded checks"))
    if len(excluded) > 2:
        excluded_toml = array()
        for check in excluded:
            excluded_toml.append(check)
    else:
        excluded_toml = excluded
    doc.add("exclude_checks", excluded_toml)

sections = table()
for section, checks in profile["sections"].items():
    checks_toml = array()
    for check in checks:
        checks_toml.append(check)
    sections.add(section, checks_toml)


doc.add("sections", sections)
if "overrides" in profile:
    doc.add(nl())
    doc.add(comment("Overrides"))
    doc.add("overrides", profile["overrides"])

configuration = profile.get("configuration_defaults", {})
if configuration:
    doc.add(nl())
    doc.add(comment("Configuration defaults"))
    config = table()
    for key, value in configuration.items():
        config.add(key, value)
    doc.add("configuration_defaults", config)

with open(output, "w") as f:
    f.write(doc.as_string())
