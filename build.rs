use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use serde::Deserialize;

const DATA: &str = include_str!("vendor/speakeasy/data.json");

#[derive(Deserialize)]
struct Language {
    tag: String,
    name: String,
    native: String,
    codepoints: Vec<[u32; 2]>,
}

fn main() {
    let languages: Vec<Language> = serde_json::from_str(DATA).unwrap();
    assert!(!languages.is_empty());

    let count = languages.len();
    let ranges = languages
        .iter()
        .map(|language| {
            format!(
                "&[{}]",
                language
                    .codepoints
                    .iter()
                    .map(|[lower, upper]| format!("[{lower}, {upper}]"))
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        })
        .collect::<Vec<_>>()
        .join(", ");
    let totals = languages
        .iter()
        .map(|language| {
            language
                .codepoints
                .iter()
                .map(|[lower, upper]| upper - lower + 1)
                .sum::<u32>()
        })
        .collect::<Vec<_>>();
    let metadata = languages
        .iter()
        .map(|language| {
            format!(
                "Metadata {{ tag: {:?}, name: {:?}, native_name: {:?} }}",
                language.tag, language.name, language.native,
            )
        })
        .collect::<Vec<_>>()
        .join(", ");

    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("data.rs");
    let mut file = File::create(path).unwrap();
    write!(
        file,
        r#"
/// A Unicode codepoint
pub type Codepoint = u32;

/// A range of Unicode codepoints.
pub type Range<T> = [T; 2];

struct Metadata {{
    tag: &'static str,
    name: &'static str,
    native_name: &'static str,
}}

#[cfg(not(test))]
const LANGUAGE_COUNT: usize = {count};

#[cfg(test)]
const LANGUAGE_COUNT: usize = 5;

#[cfg(not(test))]
const RANGES: [&[Range<Codepoint>]; LANGUAGE_COUNT] = [{ranges}];

#[cfg(test)]
const RANGES: [&[Range<Codepoint>]; LANGUAGE_COUNT] = [&[[1, 3]], &[[4, 6]], &[[7, 9]], &[[8, 8]], &[[16,16]]];

#[cfg(not(test))]
const TOTALS: [u32; LANGUAGE_COUNT] = {totals:?};

#[cfg(test)]
const TOTALS: [u32; LANGUAGE_COUNT] = [3, 3, 3, 1, 1];

#[cfg(not(test))]
const METADATA: [Metadata; LANGUAGE_COUNT] = [{metadata}];

#[cfg(test)]
const METADATA: [Metadata; LANGUAGE_COUNT] = [
  Metadata {{ tag: "t1", name: "test1", native_name: "ntest1" }},
  Metadata {{ tag: "t2", name: "test2", native_name: "ntest2" }},
  Metadata {{ tag: "t3", name: "test3", native_name: "ntest3" }},
  Metadata {{ tag: "t4", name: "test4", native_name: "ntest4" }},
  Metadata {{ tag: "t5", name: "test5", native_name: "ntest5" }},
];
"#
    )
    .unwrap();
}
