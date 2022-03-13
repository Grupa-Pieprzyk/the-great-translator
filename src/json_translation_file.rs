use indexmap::IndexMap;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JsonTranslationFile {
    Value(String),
    Subtree(IndexMap<String, Box<JsonTranslationFile>>),
}

impl Default for JsonTranslationFile {
    fn default() -> Self {
        Self::Subtree(Default::default())
    }
}

impl JsonTranslationFile {
    pub fn get<'map, 'path>(&'map self, path: &'path [String]) -> Option<&'map String> {
        match self {
            JsonTranslationFile::Value(v) => Some(v),
            JsonTranslationFile::Subtree(t) => path
                .first()
                .and_then(|key| t.get(key).and_then(|v| v.get(&path[1..]))),
        }
    }

    pub fn set(&mut self, path: &[String], val: String) {
        match self {
            Self::Value(_) => unreachable!("bad config"),
            Self::Subtree(t) => match path {
                [first, _second, ..] => t
                    .entry(first.clone())
                    .or_insert_with(|| Box::new(Self::Subtree(Default::default())))
                    .set(&path[1..], val),
                [first] => {
                    t.insert(first.clone(), Box::new(Self::Value(val)));
                }
                [] => unreachable!("this should be covered earlier"),
            },
        }
    }

    pub fn keys(&self) -> Vec<Vec<String>> {
        match self {
            Self::Value(_) => vec![vec![]],
            Self::Subtree(t) => t
                .iter()
                .flat_map(|(key, subtree)| {
                    subtree
                        .keys()
                        .into_iter()
                        .map(|keys| std::iter::once(key.clone()).chain(keys).collect_vec())
                })
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_FILE: &str = include_str!("../test_data/en.json");
    #[test]
    fn test_structure_preserved() {
        let normal_json: serde_json::Value =
            serde_json::from_str(EXAMPLE_FILE).expect("bad json file");
        let parsed: JsonTranslationFile =
            serde_json::from_str(EXAMPLE_FILE).expect("could not parse");
        let normal = serde_json::to_string_pretty(&normal_json).expect("could not serialize");
        println!("--normal--\n{normal}\n\n");
        let custom = serde_json::to_string_pretty(&parsed).expect("could not serialize");
        println!("--custom--\n{custom}");
        // assert_eq!(&normal, &custom);
    }

    #[test]
    fn test_set_single() {
        let mut config = JsonTranslationFile::default();
        assert!(&config.keys().is_empty());
        config.set(&["a".to_string()], "something".to_string());
        assert_eq!(config.keys(), vec![vec!["a"]]);
    }

    #[test]
    fn test_set_multiple() {
        let mut config = JsonTranslationFile::default();
        assert!(&config.keys().is_empty());
        config.set(&["a".to_string(), "b".to_string()], "something".to_string());
        assert_eq!(config.keys(), vec![vec!["a", "b"]]);
        assert_eq!(
            config.get(&["a".to_string(), "b".to_string()]),
            Some(&"something".to_string())
        )
    }

    #[test]
    fn test_keys() {
        let parsed: JsonTranslationFile =
            serde_json::from_str(EXAMPLE_FILE).expect("could not parse");
        assert_eq!(
            &parsed.keys()[..7],
            vec![
                vec!["Exited"],
                vec!["Running"],
                vec!["Launching"],
                vec!["Updating"],
                vec!["Error"],
                vec!["forms", "username"],
                vec!["forms", "password"],
            ]
        )
    }
}
