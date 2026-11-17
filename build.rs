use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use serde::Deserialize;

const DATA: &str = include_str!("vendor/cldr/data.json");

#[derive(Deserialize)]
struct Language {
    code: String,
    name: Option<String>,
    script: String,
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
                .map(|[lower, upper]| (upper - lower + 1) as usize)
                .sum::<usize>()
        })
        .collect::<Vec<_>>();
    let metadata = languages
        .iter()
        .map(|language| {
            format!(
                "Metadata {{ code: {:?}, name: {:?}, script: {:?} }}",
                language.code, language.name, language.script,
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
    code: &'static str,
    name: Option<&'static str>,
    script: &'static str,
}}

#[cfg(not(test))]
const LANGUAGE_COUNT: usize = {count};

#[cfg(test)]
const LANGUAGE_COUNT: usize = 5;

#[cfg(not(test))]
static RANGES: [&[Range<Codepoint>]; LANGUAGE_COUNT] = [{ranges}];

#[cfg(test)]
static RANGES: [&[Range<Codepoint>]; LANGUAGE_COUNT] = [&[[1, 3]], &[[4, 6]], &[[7, 9]], &[[8, 8]], &[[16,16]]];

#[cfg(not(test))]
static TOTALS: [usize; LANGUAGE_COUNT] = {totals:?};

#[cfg(test)]
static TOTALS: [usize; LANGUAGE_COUNT] = [3, 3, 3, 1, 1];

#[cfg(not(test))]
static METADATA: [Metadata; LANGUAGE_COUNT] = [{metadata}];

#[cfg(test)]
static METADATA: [Metadata; LANGUAGE_COUNT] = [
  Metadata {{ code: "t1", name: Some("test1"), script: "s1" }},
  Metadata {{ code: "t2", name: Some("test2"), script: "s2" }},
  Metadata {{ code: "t3", name: Some("test3"), script: "s3" }},
  Metadata {{ code: "t4", name: Some("test4"), script: "s4" }},
  Metadata {{ code: "t5", name: Some("test5"), script: "s5" }},
];
"#
    )
    .unwrap();
}
