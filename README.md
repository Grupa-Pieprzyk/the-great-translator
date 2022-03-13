# the-great-translator
Simple CLI tool for checking and updating i18n translations of your frontend app. Uses DEEPL to automatically generate missing ones and checks if all of them are handled properly.
## compiling
you'll need the rust compiler, you can get it here https://rustup.rs/

to compile run 
```bash
cargo build --release
```
output binary will be  `./target/release/the-great-translator

## running
run 
```bash
the-great-translator --help
```
and you will get this output:
```
the-great-translator 0.1.0

USAGE:
    the-great-translator [OPTIONS] --files-glob <FILES_GLOB> --translations-dir <TRANSLATIONS_DIR>

OPTIONS:
    -d, --deepl-api-key <DEEPL_API_KEY>
            (optional) deepl api key, used for auto translations, identical to setting DEEPL_API_KEY
            env var

    -f, --files-glob <FILES_GLOB>
            glob pattern to search files by, eg. ./**/*.svelte

    -h, --help
            Print help information

    -i, --ignore-excess
            ignore excess entries in *.json files

    -s, --source-for-translations <SOURCE_FOR_TRANSLATIONS>
            (optional) enables auto-translations. This file should be updated first, separately, and
            will become the source for translations in other languages

    -t, --translations-dir <TRANSLATIONS_DIR>
            a directory containing translations in json files (eg. "pl.json", "en.json") to validate

    -V, --version
            Print version information

    -w, --write
            update missing entries in *.json files
```
