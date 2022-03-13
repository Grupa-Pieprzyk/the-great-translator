use crate::UNHANDLED_MARKER;
use anyhow::{bail, Context, Result};
use deepl_api::{DeepL, TranslatableTextList};
use itertools::Itertools;
use std::path::Path;

lazy_static::lazy_static! {
    pub static ref DEEPL: Option<DeepL> = {
        if let Ok(api_key) = std::env::var("DEEPL_API_KEY") {
            Some(DeepL::new(api_key))
        } else {
            None
        }

    };
}

pub fn translate(text: &str, from: String, to: String) -> Result<String> {
    if text.contains(UNHANDLED_MARKER) {
        bail!("'{text}' contains '{UNHANDLED_MARKER}'");
    }

    let texts = TranslatableTextList {
        source_language: Some(from),
        target_language: to,
        texts: vec![text.to_string()],
    };
    let translated = DEEPL
        .as_ref()
        .context("no DEEPL_API_KEY in env")?
        .translate(None, texts)
        .map_err(|e| anyhow::anyhow!(format!("failed to translate '{text}' :: {e}")))?
        .into_iter()
        .map(|t| t.text)
        .join(" ");

    Ok(translated)
}

pub fn lang_from_filename(filename: &Path) -> anyhow::Result<String> {
    Ok(filename
        .with_extension("")
        .file_name()
        .context(format!("bad filename: {filename:?}"))?
        .to_str()
        .context(format!("bad filename encoding :: {filename:?}"))?
        .to_string())
}
