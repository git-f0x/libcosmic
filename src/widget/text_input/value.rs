// Copyright 2019 H�ctor Ram�n, Iced contributors
// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: MIT

use unicode_segmentation::UnicodeSegmentation;

/// The value of a [`TextInput`].
///
/// [`TextInput`]: crate::widget::TextInput
// TODO: Reduce allocations, cache results (?)
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Value {
    graphemes: Vec<String>,
}

impl Value {
    /// Creates a new [`Value`] from a string slice.
    pub fn new(string: &str) -> Self {
        let graphemes = UnicodeSegmentation::graphemes(string, true)
            .map(String::from)
            .collect();

        Self { graphemes }
    }

    /// Returns whether the [`Value`] is empty or not.
    ///
    /// A [`Value`] is empty when it contains no graphemes.
    #[must_use]
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the total amount of graphemes in the [`Value`].
    #[must_use]
    #[inline]
    pub fn len(&self) -> usize {
        self.graphemes.len()
    }

    /// Returns the position of the previous start of a word from the given
    /// grapheme `index`.
    #[must_use]
    pub fn previous_start_of_word(&self, index: usize) -> usize {
        let previous_string = &self.graphemes[..index.min(self.graphemes.len())].concat();

        UnicodeSegmentation::split_word_bound_indices(previous_string as &str)
            .filter(|(_, word)| !word.trim_start().is_empty())
            .next_back()
            .map_or(0, |(i, previous_word)| {
                index
                    - UnicodeSegmentation::graphemes(previous_word, true).count()
                    - UnicodeSegmentation::graphemes(
                        &previous_string[i + previous_word.len()..] as &str,
                        true,
                    )
                    .count()
            })
    }

    /// Returns the position of the next end of a word from the given grapheme
    /// `index`.
    #[must_use]
    pub fn next_end_of_word(&self, index: usize) -> usize {
        let next_string = &self.graphemes[index..].concat();

        UnicodeSegmentation::split_word_bound_indices(next_string as &str)
            .find(|(_, word)| !word.trim_start().is_empty())
            .map_or(self.len(), |(i, next_word)| {
                index
                    + UnicodeSegmentation::graphemes(next_word, true).count()
                    + UnicodeSegmentation::graphemes(&next_string[..i] as &str, true).count()
            })
    }

    /// Returns a new [`Value`] containing the graphemes from `start` until the
    /// given `end`.
    #[must_use]
    #[inline]
    pub fn select(&self, start: usize, end: usize) -> Self {
        let graphemes = self.graphemes[start.min(self.len())..end.min(self.len())].to_vec();

        Self { graphemes }
    }

    /// Returns a new [`Value`] containing the graphemes until the given
    /// `index`.
    #[must_use]
    #[inline]
    pub fn until(&self, index: usize) -> Self {
        let graphemes = self.graphemes[..index.min(self.len())].to_vec();

        Self { graphemes }
    }

    /// Inserts a new `char` at the given grapheme `index`.
    #[inline]
    pub fn insert(&mut self, index: usize, c: char) {
        self.graphemes.insert(index, c.to_string());

        self.graphemes = UnicodeSegmentation::graphemes(&self.to_string() as &str, true)
            .map(String::from)
            .collect();
    }

    /// Inserts a bunch of graphemes at the given grapheme `index`.
    #[inline]
    pub fn insert_many(&mut self, index: usize, mut value: Value) {
        let _ = self
            .graphemes
            .splice(index..index, value.graphemes.drain(..));
    }

    /// Removes the grapheme at the given `index`.
    #[inline]
    pub fn remove(&mut self, index: usize) {
        let _ = self.graphemes.remove(index);
    }

    /// Removes the graphemes from `start` to `end`.
    #[inline]
    pub fn remove_many(&mut self, start: usize, end: usize) {
        let _ = self.graphemes.splice(start..end, std::iter::empty());
    }

    /// Returns a new [`Value`] with all its graphemes replaced with the
    /// dot ('•') character.
    #[must_use]
    pub fn secure(&self) -> Self {
        Self {
            graphemes: std::iter::repeat(String::from("•"))
                .take(self.graphemes.len())
                .collect(),
        }
    }
}

impl ToString for Value {
    #[inline]
    fn to_string(&self) -> String {
        self.graphemes.concat()
    }
}
