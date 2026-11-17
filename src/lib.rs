//! Detection of languages from Unicode codepoints.

include!(concat!(env!("OUT_DIR"), "/data.rs"));

use std::cmp;

/// A language.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Language {
    /// The BCP 47 code.
    pub code: &'static str,
    /// The proper name in English, if known.
    pub name: Option<&'static str>,
    /// The ISO 15924 code of the script.
    pub script: &'static str,
    /// The number of codepoints found in the language.
    pub count: usize,
    /// The total number of codepoints in the language.
    pub total: usize,
}

/// Detect languages given ranges of Unicode codepoints.
pub fn detect<T>(codepoints: T, threshold: f64) -> Vec<Language>
where
    T: IntoIterator<Item = Range<Codepoint>>,
{
    let mut counts = [0; LANGUAGE_COUNT];

    for [input_lower, input_upper] in codepoints {
        for i in 0..LANGUAGE_COUNT {
            for [range_lower, range_upper] in RANGES[i] {
                if input_lower <= *range_upper && *range_lower <= input_upper {
                    let count = cmp::min(input_upper, *range_upper)
                        - cmp::max(input_lower, *range_lower)
                        + 1;
                    counts[i] += count as usize;
                }

                if input_upper <= *range_upper {
                    break;
                }
            }
        }
    }

    let mut result = Vec::new();

    for i in 0..LANGUAGE_COUNT {
        let score = counts[i] as f64 / TOTALS[i] as f64;
        if score >= threshold && counts[i] > 0 {
            result.push(Language {
                code: METADATA[i].code,
                name: METADATA[i].name,
                script: METADATA[i].script,
                count: counts[i],
                total: TOTALS[i],
            });
        }
    }

    // Sort by the share of codepoints found in descending order.
    result.sort_by(|one, other| (other.count * one.total).cmp(&(one.count * other.total)));

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_an_empty_array() {
        let result = detect([], 0.5);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn it_takes_a_vector() {
        let codepoints = vec![[1, 3]];

        let result = detect(codepoints, 1.0);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].code, "t1");
        assert_eq!(result[0].name, Some("test1"));
    }

    #[test]
    fn it_takes_an_array() {
        let codepoints = [[1, 3]];

        let result = detect(codepoints, 1.0);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].code, "t1");
        assert_eq!(result[0].name, Some("test1"));
    }

    #[test]
    fn it_returns_an_empty_array_with_an_invalid_codepoint() {
        let result = detect([[256, 256]], 0.5);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn it_returns_the_test_language() {
        let result = detect([[1, 1]], 0.0);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].code, "t1");
        assert_eq!(result[0].name, Some("test1"))
    }

    #[test]
    fn it_does_not_return_if_threshold_not_met() {
        let result = detect([[1, 2]], 1.0);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn it_returns_if_threshold_is_met() {
        let result = detect([[1, 3]], 1.0);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].code, "t1");
        assert_eq!(result[0].name, Some("test1"));
    }

    #[test]
    fn it_returns_if_threshold_is_partially_met() {
        let result = detect([[1, 2]], 0.6);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].code, "t1");
        assert_eq!(result[0].name, Some("test1"));
    }

    #[test]
    fn it_returns_multiple_languages() {
        let result = detect([[1, 1], [4, 4]], 0.0);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].code, "t1");
        assert_eq!(result[0].name, Some("test1"));
        assert_eq!(result[1].code, "t2");
        assert_eq!(result[1].name, Some("test2"));
    }

    #[test]
    fn it_returns_overlapping_languages() {
        let result = detect([[8, 8]], 0.0);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].code, "t4");
        assert_eq!(result[0].name, Some("test4"));
        assert_eq!(result[1].code, "t3");
        assert_eq!(result[1].name, Some("test3"));
    }

    #[test]
    fn it_returns_correct_counts_on_partial_range_matches() {
        let result = detect([[3, 5]], 0.0);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].code, "t2");
        assert_eq!(result[0].name, Some("test2"));
        assert_eq!(result[0].count, 2);
        assert_eq!(result[1].code, "t1");
        assert_eq!(result[1].name, Some("test1"));
        assert_eq!(result[1].count, 1);
    }

    #[test]
    fn it_returns_sorted_results() {
        let result = detect([[1, 1], [4, 6]], 0.0);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].code, "t2");
        assert_eq!(result[1].code, "t1");
    }

    #[test]
    fn it_handles_ranges_correctly() {
        let result = detect([[12, 20]], 0.0);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].code, "t5");
    }
}
