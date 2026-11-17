//! Detection of languages from Unicode codepoints.

#![allow(clippy::type_complexity)]

#[cfg(test)]
mod tests;

include!(concat!(env!("OUT_DIR"), "/data.rs"));

/// A context.
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct Context {}

/// A language.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Language {
    /// The BCP 47 tag.
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

impl Context {
    /// Detect languages given ranges of Unicode codepoints.
    ///
    /// Each range is closed, containing both the first and the last codepoint. A single codepoint
    /// is given as a range whose ends coincide. The ranges should not overlap.
    ///
    /// The threshold is the minimum share of the codepoints of a language, from zero to one, that
    /// should be found for the language to be reported. The languages are sorted by this share in
    /// descending order with ties broken by code.
    pub fn detect<T>(&self, codepoints: T, threshold: f64) -> Vec<Language>
    where
        T: IntoIterator<Item = [u32; 2]>,
    {
        let mut counts = vec![0; DATA.len()];
        for [start, end] in codepoints {
            for (count, (.., ranges)) in counts.iter_mut().zip(DATA) {
                for (lower, upper) in ranges.iter().cloned() {
                    if start < upper && lower <= end {
                        *count += ((end + 1).min(upper) - start.max(lower)) as usize;
                    }
                    // The ranges are sorted, and the rest lies beyond.
                    if end < upper {
                        break;
                    }
                }
            }
        }
        let mut languages = DATA
            .iter()
            .zip(counts)
            .filter(|((.., total, _), count)| {
                *count > 0 && *count as f64 / *total as f64 >= threshold
            })
            .map(|(&(code, name, script, total, _), count)| Language {
                code,
                name,
                script,
                count,
                total,
            })
            .collect::<Vec<_>>();
        languages.sort_by(|one, other| {
            (other.count * one.total)
                .cmp(&(one.count * other.total))
                .then_with(|| one.code.cmp(other.code))
        });
        languages
    }
}
