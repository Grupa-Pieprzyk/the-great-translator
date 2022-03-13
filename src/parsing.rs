use anyhow::{Context, Result};
use xmlparser::Token;

use self::translation_part_parser::{translation_parts, Translated};

pub mod translation_part_parser {
    use itertools::Itertools;
    use nom::{
        branch::alt,
        bytes::complete::{tag, take_while, take_while1},
        combinator::opt,
        error::{context, VerboseError},
        multi::separated_list1,
        sequence::{delimited, preceded, tuple},
        AsChar, IResult,
    };
    use regex::Regex;

    use super::*;

    use derive_more::From;

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Translated {
        pub segments: Vec<String>,
    }

    type ParserRes<T, U> = IResult<T, U, VerboseError<T>>;
    type Res<'a, T> = ParserRes<&'a str, T>;

    macro_rules! quoted {
        ($parser:expr) => {{
            alt((
                delimited(tag("\""), $parser(), tag("\"")),
                delimited(tag("'"), $parser(), tag("'")),
            ))
        }};
    }

    // pub fn translation_part(input: &str) -> Res<Translated> {
    //     let segments =
    //         || separated_list1(tag("."), take_while1(|c: char| c.is_alphanum() || c == '_'));

    //     let (input, segments) =
    //         preceded(tag("_"), delimited(tag("("), quoted!(segments), tag(")")))(input)?;

    //     Ok((input, Translated { segments }.into()))
    // }

    // pub fn translation_parts<'parts, 'content: 'parts>(
    //     content: &'content str,
    // ) -> Res<'content, Vec<Option<Translated<'parts>>>> {
    //     let whitespace = || take_while(|c| c != '_');
    //     preceded(
    //         whitespace(),
    //         separated_list1(whitespace(), opt(translation_part)),
    //     )(content)
    // }

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
    use super::translation_part_parser::{translation_parts, Translated};
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
