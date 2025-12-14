#[cfg(test)]
mod tests {
    use public_api::{
        PublicApi,
        PublicItem,
        diff::PublicApiDiff,
    };
    use serde::Deserialize;
    use similar::{
        ChangeTag,
        TextDiff,
    };
    use std::fs::read_to_string;

    use crate::color::{
        Color,
        RGBColor,
        TerminalColor,
    };

    #[derive(Deserialize)]
    struct Root {
        toolchain: Toolchain,
    }

    #[derive(Deserialize)]
    struct Toolchain {
        channel: String,
    }

    fn build_public_api(manifest_path: Option<&str>) -> PublicApi {
        let Root {
            toolchain: Toolchain { channel },
        } = toml::from_str::<Root>(
            &read_to_string("rust-toolchain.toml").unwrap(),
        )
        .unwrap();

        // Install a compatible nightly toolchain if it is missing.
        rustup_toolchain::install(&channel).unwrap();

        // Build rustdoc JSON.
        let mut rustdoc_json_builder =
            rustdoc_json::Builder::default().toolchain(&channel);
        if let Some(path) = manifest_path {
            rustdoc_json_builder =
                rustdoc_json_builder.manifest_path(path);
        }
        let rustdoc_json = rustdoc_json_builder.build().unwrap();

        // Derive the public API from rustdoc JSON.
        public_api::Builder::from_rustdoc_json(rustdoc_json)
            .build()
            .unwrap()
    }

    fn public_api_diff() -> PublicApiDiff {
        let old_public_api = build_public_api(Some("master/Cargo.toml"));
        let new_public_api = build_public_api(None);

        PublicApiDiff::between(old_public_api, new_public_api)
    }

    fn diff(
        old: &str,
        new: &str,
        add_color: RGBColor,
        remove_color: RGBColor,
    ) -> String {
        let diff = TextDiff::from_lines(old, new);

        let mut res = String::new();
        for change in diff.iter_all_changes() {
            let (color, sign) = match change.tag() {
                ChangeTag::Delete => (add_color.into(), "- "),
                ChangeTag::Insert => (remove_color.into(), "+ "),
                ChangeTag::Equal => continue,
            };
            res.push_str(&format!(
                "{}",
                TerminalColor::color(
                    &format!("{}{}", sign, change),
                    &color,
                    &TerminalColor::Default
                )
            ));
        }
        res
    }

    fn diff_unified_line(
        old: &str,
        new: &str,
        add_color: RGBColor,
        remove_color: RGBColor,
    ) -> String {
        let diff = TextDiff::from_chars(old, new);

        let mut res = String::from("  ");
        for change in diff.iter_all_changes() {
            let color = match change.tag() {
                ChangeTag::Equal => {
                    res.push_str(&format!(
                        "{}",
                        change.to_string().trim_end_matches("\n")
                    ));
                    continue;
                }
                ChangeTag::Delete => remove_color,
                ChangeTag::Insert => add_color,
            };
            res.push_str(&format!(
                "{}",
                TerminalColor::color(
                    change.to_string().trim_end_matches("\n"),
                    &color.into(),
                    &TerminalColor::Default
                )
            ));
        }
        res.push('\n');
        res
    }

    fn vec_to_string<T: ToString>(vec: Vec<T>) -> String {
        vec.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[ignore]
    #[test]
    fn print_changes() {
        let api_diff = public_api_diff();
        let (new_changed, old_changed): (
            Vec<PublicItem>,
            Vec<PublicItem>,
        ) = api_diff.changed.into_iter().map(|x| (x.new, x.old)).unzip();

        let added = diff(
            "",
            &vec_to_string(api_diff.added),
            RGBColor::GREEN,
            RGBColor::RED,
        );
        let changed = diff_unified_line(
            &vec_to_string(old_changed),
            &vec_to_string(new_changed),
            RGBColor::GREEN,
            RGBColor::RED,
        );
        let removed = diff(
            &vec_to_string(api_diff.removed),
            "",
            RGBColor::GREEN,
            RGBColor::RED,
        );
        
        if !added.trim().is_empty() {
            println!("{}{}", "Added:\n", added);
        }

        if !changed.trim().is_empty() {
            println!("{}{}", "Changed:\n", changed);
        }

        if !removed.trim().is_empty() {
            println!("{}{}", "Removed:\n", removed);
        }
    }

    #[ignore]
    #[test]
    fn is_patch() {
        // Check if there are no changes to API
        let api_diff = public_api_diff();
        let added_items = diff(
            "",
            &vec_to_string(api_diff.added),
            RGBColor::GREEN,
            RGBColor::RED,
        );
        assert!(added_items.trim().is_empty(), "");
        is_minor();
    }

    #[ignore]
    #[test]
    fn is_minor() {
        // Check if there are only new items
        let api_diff = public_api_diff();
        let (new_changed, old_changed): (
            Vec<PublicItem>,
            Vec<PublicItem>,
        ) = api_diff.changed.into_iter().map(|x| (x.new, x.old)).unzip();
        let changed_items = diff_unified_line(
            &vec_to_string(old_changed),
            &vec_to_string(new_changed),
            RGBColor::GREEN,
            RGBColor::RED,
        );
        let removed_items = diff(
            &vec_to_string(api_diff.removed),
            "",
            RGBColor::GREEN,
            RGBColor::RED,
        );
        assert!(changed_items.trim().is_empty() && removed_items.trim().is_empty(), "");
    }
}
