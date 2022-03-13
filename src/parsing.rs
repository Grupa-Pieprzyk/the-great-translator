use anyhow::{Context, Result};

pub mod translation_part_parser {
    use itertools::Itertools;
    use regex::Regex;

    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Translated {
        pub segments: Vec<String>,
    }

    pub fn translation_parts(content: &str) -> Result<Vec<Translated>> {
        lazy_static::lazy_static! {
            static ref RE: Regex = Regex::new(r#"_\(["']((\S+\.)+\S+)["']\)"#)
                .context("bad regex")
                .unwrap();
        }

        Ok(RE
            .captures_iter(content)
            .map(|m| Translated {
                segments: m[1].split('.').map(Into::into).collect_vec(),
            })
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::translation_part_parser::translation_parts;
    use anyhow::Context;

    const CONTENT: &str = r#"
    <div>
        <div class="text">
            not translated
        </div>
        <div class="icon">
            <Sliders />
        </div>
        <div class="text">
            {$_('sidebar_buttons.settings')}
        </div>
    </div>
    "#;
    #[test]
    pub fn test_example_serde_file() -> anyhow::Result<()> {
        assert_eq!(
            translation_parts(CONTENT)?
                .first()
                .context("no translation entry found")?
                .segments,
            vec!["sidebar_buttons", "settings"],
        );
        Ok(())
    }
}
