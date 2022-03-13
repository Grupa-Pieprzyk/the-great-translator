mod json_translation_file;
mod parsing;
mod translations;

use std::{collections::HashSet, path::PathBuf};

use anyhow::{bail, Context, Result};
use clap::Parser;
use itertools::Itertools;
use json_translation_file::JsonTranslationFile;
use translations::{translate, lang_from_filename};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    /// glob pattern to search files by, eg. ./**/*.svelte
    pub files_glob: String,

    #[clap(short, long)]
    /// a directory containing translations in json files (eg. "pl.json", "en.json") to validate
    pub translations_dir: PathBuf,

    #[clap(short, long)]
    /// update missing entries in *.json files
    pub write: bool,

    #[clap(short, long)]
    /// ignore excess entries in *.json files
    pub ignore_excess: bool,

    #[clap(short, long)]
    /// (optional) enables auto-translations. This file should be updated first, separately, and will become the source for translations in other languages
    pub source_for_translations: Option<PathBuf>,

    #[clap(short, long)]
    /// (optional) deepl api key, used for auto translations, identical to setting DEEPL_API_KEY env var
    pub deepl_api_key: Option<String>,
}
pub const UNHANDLED_MARKER: &str = "---UHANDLED---";
fn main() -> Result<()> {
    // logging
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::try_init().ok();
    // end of logging

    let args = Args::parse();
    if let Some(value) = args.deepl_api_key{
        std::env::set_var("DEEPL_API_KEY", value);
    }
    let translation_markers = glob::glob(&args.files_glob)
        .context("bad glob pattern")?
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
        .context("bad path")?
        .iter()
        .map(|path| {
            std::fs::read_to_string(path)
                .context("opening source file")
                .and_then(|content| {
                    parsing::translation_part_parser::translation_parts(&content)
                        .context("extracting translation parts")
                })
        })
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .flatten()
        .unique()
        .map(|t| t.segments)
        .collect::<HashSet<_>>();

    let mut valid = true;

    let all_translation_paths = std::fs::read_dir(args.translations_dir)
        .context("reading translations dir")?
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
        .context("reading translation dir file")?;

    let from_json_file = |source| -> anyhow::Result<JsonTranslationFile> {std::fs::read_to_string(&source)
    .context("reading translation file")
    .and_then(|content| {
        serde_json::from_str(&content).context("parsing translation file")
    })};
    for translation_file_path in args.source_for_translations.iter().cloned().chain(
        all_translation_paths
            .into_iter()
            .filter(|e| {
                e.path()
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .map(|ext| ext == "json")
                    .unwrap_or(false)
            })
            .map(|dir_entry| dir_entry.path()),
    ) {
        let translation_file: json_translation_file::JsonTranslationFile =
            from_json_file(translation_file_path.clone())?;
        let file_keys = translation_file.keys().into_iter().collect::<HashSet<_>>();

        let translation = translation_file_path.display();
        log::info!("checking {translation}");
        let diff = translation_markers.difference(&file_keys).collect_vec();
        if args.write {
            let mut translation_file = translation_file;
            for unhandled in &diff {
                translation_file.set(
                    unhandled,
                    match args.source_for_translations.as_ref() {
                        Some(source)
                            if source.canonicalize()?
                                == translation_file_path.canonicalize()? =>
                        {
                            // throw an error because we need a source for the rest of translations
                            bail!("{unhandled:?} is not handled in translation source file {source:?}, it needs to be updated first");
                        }
                        Some(source) => {
                            let source_json = from_json_file(source.clone()).context("loading source file for translation")?;
                            let source_value = source_json.get(unhandled).context(format!("{unhandled:?} is not specified in source {source_json:?}, but it us required in order to generate a translation"))?;
                            translate(source_value, lang_from_filename(source)?, lang_from_filename(&translation_file_path)?)?
                            
                        }
                        None => format!("{}.{}", UNHANDLED_MARKER, unhandled.join(".")),
                    },
                );
                // log::info!("updating {translation} with {unhandled:?}");
            }
            if !diff.is_empty() {
                std::fs::write(
                    &translation_file_path,
                    serde_json::to_string_pretty(&translation_file)
                        .context("serializing updated translation file")?,
                )
                .context("saving updated translation to file")?;
            }
        } else {
            for unhandled in diff {
                valid = false;
                log::error!("no ${unhandled:?} in {translation}");
            }
        }
        if !args.ignore_excess {
            for excess in file_keys.difference(&translation_markers) {
                log::warn!("excess ${excess:?} in {translation}");
            }
        }
    }
    if let Ok(deepl) = translations::DEEPL.as_ref().context("no api key") {
        if let Ok(usage_information) = deepl.usage_information() {
            log::info!("deepl character limit: {}", usage_information.character_limit);
        }
    }
    

    if !valid {
        bail!("check above errors");
    }
    log::info!("OK!");
    Ok(())
}
