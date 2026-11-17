use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(serde::Deserialize)]
struct Language {
    code: String,
    name: Option<String>,
    script: String,
    codepoints: Vec<[u32; 2]>,
}

const DATA: &str = include_str!("vendor/cldr/data.json");

macro_rules! ok(($result:expr) => ($result.unwrap()));

fn main() {
    let root = ok!(std::env::var("OUT_DIR"));
    let path = Path::new(&root).join("data.rs");
    let mut file = ok!(File::create(&path));
    let data: Vec<Language> = ok!(serde_json::from_str(DATA));
    let data = data
        .into_iter()
        .map(|language| {
            let total = language
                .codepoints
                .iter()
                .map(|[start, end]| (end - start + 1) as usize)
                .sum::<usize>();
            let ranges = language
                .codepoints
                .into_iter()
                .map(|[start, end]| format!("[{start},{end}]"))
                .collect::<Vec<_>>()
                .join(",");
            format!(
                "({:?}, {:?}, {:?}, {total}, &[{ranges}])",
                language.code, language.name, language.script,
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    ok!(write!(
        file,
        "const DATA: &[(&str, Option<&str>, &str, usize, &[[u32; 2]])] = &[{data}];",
    ));
}
