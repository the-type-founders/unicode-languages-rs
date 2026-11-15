# Unicode Languages [![Package][package-img]][package-url] [![Documentation][documentation-img]][documentation-url] [![Build][build-img]][build-url]

This library detects language support given a list of Unicode codepoints. This is primarily useful for finding out what languages a font supports.

```rust
// Input codepoints as a vector of Unicode ranges
let codepoints = vec![[65, 121]];

// Detect languages with a threshold of 0.5
let results = detect(codepoints, 0.5);

// results[0].tag = "en"
// results[1].tag = "nl"
// ...
```

The result is a vector of `Match` structs, with the following signature:

```rust
struct Match {
  // BCP 47 language tag.
  tag: &'static str,
  // English name.
  name: &'static str,
  // Name in native script.
  native: &'static str,
  // Number of codepoints matched.
  count: u32,
  // Score (number of codepoints matched divided by the total).
  score: f64,
}
```

The language data is derived from [Adobe Font's speakeasy library](https://github.com/typekit/speakeasy). The data is vendored in `vendor/speakeasy/data.json`, which is regenerated from the submodule by running `make -C vendor/speakeasy`, and statically compiled as part of the library.

## License

This library is licensed under the Apache-2.0 license. Copyright 2023, [The Type Founders](https://thetypefounders.com).

[build-img]: https://github.com/the-type-founders/unicode-languages-rs/actions/workflows/build.yml/badge.svg
[build-url]: https://github.com/the-type-founders/unicode-languages-rs/actions/workflows/build.yml
[documentation-img]: https://docs.rs/unicode-languages/badge.svg
[documentation-url]: https://docs.rs/unicode-languages
[package-img]: https://img.shields.io/crates/v/unicode-languages.svg
[package-url]: https://crates.io/crates/unicode-languages
