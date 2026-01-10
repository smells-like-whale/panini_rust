use std::ops::Deref;

use crate::{util::utilities::concat_by_line, xsampa};

use super::{
    feature::{feature_table::FeatureTable, FeatureIndex},
    segment_string::SegmentStringShell,
    ASegment, Feature, SegModif, SegmentShell, TokenString,
};

//This represents an uninterrupted utterence, like a "word", or a sequence of "words", without "spaces"
#[derive(Debug)]
pub struct SegmentStr {
    segments: [SegmentShell<FeatureTable>],
}

impl SegmentStr {
    pub fn print_words(self_arr: &[&Self]) -> String {
        let words: Vec<String> = self_arr.iter().map(|&word| word.to_long_string()).collect();

        let mut x = vec!["\n\n".to_string() + &Feature::list_features()];

        x.extend(words);
        concat_by_line(&x, "  ")
    }

    pub fn to_string(&self) -> String {
        let mut str = String::new();

        for i in 0..self.segments.len() {
            str.push_str(&xsampa::xsampa(&self.segments[i].tokens().concat()));
        }
        str
    }

    pub fn to_long_string(&self) -> String {
        let mut str_vec = Vec::new(); //vec![fets];

        for str in self.to_string_vec() {
            str_vec.push(str);
        }

        concat_by_line(&str_vec, "")
    }

    fn to_string_vec(&self) -> Vec<String> {
        let mut str_vec: Vec<String> = Vec::new();

        for s in &self.segments {
            str_vec.push(s.to_short_debug_string());
        }

        str_vec
    }
}
