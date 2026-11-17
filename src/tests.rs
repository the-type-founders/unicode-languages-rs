use crate::Context;

macro_rules! ok(($result:expr) => ($result.unwrap()));

#[test]
fn empty() {
    let context = Context::default();
    assert!(context.detect([], 0.0).is_empty());
    // A private-use codepoint belongs to no language.
    assert!(context.detect([[0xE000, 0xE000]], 0.0).is_empty());
}

#[test]
fn english() {
    let languages = Context::default().detect([[0x61, 0x7A]], 1.0);
    let language = ok!(languages.iter().find(|language| language.code == "en"));
    assert_eq!(language.name, Some("English"));
    assert_eq!(language.script, "Latn");
    assert_eq!(language.count, 26);
    assert_eq!(language.total, 26);
}

#[test]
fn punjabi() {
    let languages = Context::default().detect([[0x0A00, 0x0A7F]], 1.0);
    let codes = languages
        .iter()
        .map(|language| (language.code, language.script))
        .collect::<Vec<_>>();
    assert_eq!(codes, [("pa", "Guru")]);
}

#[test]
fn serbian() {
    let languages = Context::default().detect([[0x0400, 0x04FF]], 1.0);
    let language = ok!(languages.iter().find(|language| language.code == "sr"));
    assert_eq!(language.script, "Cyrl");
    assert!(languages.iter().all(|language| language.code != "sr-Latn"));
}

#[test]
fn overlap() {
    // The range starts before the first letter and ends after the second.
    let languages = Context::default().detect([[0x60, 0x62]], 0.0);
    let language = ok!(languages.iter().find(|language| language.code == "en"));
    assert_eq!(language.count, 2);
    // The ranges cover the first and the last letter.
    let languages = Context::default().detect([[0x61, 0x61], [0x7A, 0x7A]], 0.0);
    let language = ok!(languages.iter().find(|language| language.code == "en"));
    assert_eq!(language.count, 2);
}

#[test]
fn order() {
    let languages = Context::default().detect([[0x61, 0x7A]], 0.5);
    assert!(languages.len() > 1);
    assert!(languages.windows(2).all(|pair| pair[0].code < pair[1].code));
}

#[test]
fn threshold() {
    let context = Context::default();
    // All letters but the last one are given.
    let find = |threshold| {
        context
            .detect([[0x61, 0x79]], threshold)
            .into_iter()
            .find(|language| language.code == "en")
    };
    assert!(find(1.0).is_none());
    assert_eq!(ok!(find(0.9)).count, 25);
}
