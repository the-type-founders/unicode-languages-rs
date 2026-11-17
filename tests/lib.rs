use unicode_languages::detect;

macro_rules! ok(($result:expr) => ($result.unwrap()));

#[test]
fn english() {
    let result = detect([[0x61, 0x7A]], 1.0);
    let language = ok!(result.iter().find(|language| language.code == "en"));
    assert_eq!(language.script, "Latn");
    assert_eq!(language.name, Some("English"));
    assert_eq!(language.count, 26);
    assert_eq!(language.total, 26);
}

#[test]
fn punjabi() {
    let result = detect([[0x0A00, 0x0A7F]], 1.0);
    let codes = result
        .iter()
        .map(|language| (language.code, language.script))
        .collect::<Vec<_>>();
    assert_eq!(codes, [("pa", "Guru")]);
}

#[test]
fn serbian() {
    let result = detect([[0x0400, 0x04FF]], 1.0);
    let language = ok!(result.iter().find(|language| language.code == "sr"));
    assert_eq!(language.script, "Cyrl");
    assert!(result.iter().all(|language| language.code != "sr-Latn"));
}
