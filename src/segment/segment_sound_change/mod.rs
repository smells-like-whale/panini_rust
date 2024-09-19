use std::ops::Deref;

use itertools::Itertools;
use sound_change_utilities::parse_scstring;

use crate::{
    segment::feature::parse_featurebundle_list, 
    util::utilities::split_sc, 
    xsampa
};

use super::{segment_string::SegmentString, ASegment, FeatureBundle, Segment, TokenString};

pub mod sound_change_segment_string;
pub mod sound_change_utilities;
//pub mod sound_change_main;

pub trait PhonObject {
    fn apply(&self, sc: &SoundChange) -> Self;
    fn apply_mut(&mut self, sc: &SoundChange);
}

#[derive(Debug, Eq, PartialEq)]
pub enum SCSegment {
    TokenSeg(TokenString),
    FeatureSeg(FeatureBundle),
    WordEdge,
}

impl SCSegment {
    pub fn to_segment(&self) -> Option<Segment> {
        match self {
            SCSegment::TokenSeg(tok) => Some(Segment::from_string(&tok.concat())),
            SCSegment::FeatureSeg(_) => None,
            SCSegment::WordEdge => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            SCSegment::TokenSeg(tok) => xsampa::xsampa(&tok.concat()),
            SCSegment::FeatureSeg(fet) => fet.print(),
            SCSegment::WordEdge => "#".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct SCString {
    pub segments: Vec<SCSegment>,
}

impl Deref for SCString {
    type Target = [SCSegment];

    fn deref(&self) -> &Self::Target {
        &self.segments
    }
}

impl SCString {
    pub fn to_segments(&self) -> Option<SegmentString> {
        let iter = self.segments.iter().map(
            |scstr| scstr.to_segment()
        );

        let mut result = SegmentString::new();

        for seg_option in iter {
            match seg_option {
                Some(seg) => result.push(seg),
                None => return None,
            }
        }
        
        Some(result)
    }

    pub fn to_string(&self) -> String {
        let mut str_result = String::new();

        for scs in &self.segments {
            str_result.push_str(&scs.to_string());
        }

        str_result
    }
}

/**
Transforms a subsequence of tokens, of the form [from] to [to] 
when the subsequence [before] occurs before the subsequence, and [after], after
Eg: let sound_change_example = SoundChange::new("k->p/[+Vowel]__t"); 
"Attested in romanian, Lat:octō -> Rom:opt", with additional terminal vowel deletion.
Note that this struct uses SegmentString instead of Word, the Word struct contains a massive
amount of information that SoundChange and its associated functions will never use.
*/
#[derive(Debug)]
pub struct SoundChange {
    //This is the segment, or sequence of segments, that will be modified
    from: SCString,  
    /*
    Describes how self.from will be modified
    If segments are to be added or removed then all segment modifiying tokens 
    must be of the string type
    */
    to: SCString,
    //The necessary context preceeding the modified segment
    before: SCString,
    //The necessary context followng the modified segment
    after: SCString, 
}

impl SoundChange {
    pub fn new(str: &str) -> Option<Self> {
        let (str_from, str_to, str_before, str_after) = split_sc(str)?;

        let from = parse_scstring(&str_from)?;
        // println!("{:?}", from);
        let to = parse_scstring(&str_to)?;
        // println!("{:?}", to);
        let before = parse_scstring(&str_before)?;
        // println!("{:?}", before);
        let after = parse_scstring(&str_after)?;
        // println!("{:?}", from);
        // println!("SegmentSoundChange::new() is working"); 
        Some(Self {
            from,
            to,
            before,
            after,
        })
    }

    pub fn new_vec(str_arr: &[&str]) -> Option<Vec<Self>> {
        let mut result = Vec::new();
        
        for str in str_arr {
            match SoundChange::new(str) {
                Some(sc) => result.push(sc),
                None => return None,
            }
        }

        Some(result)
    }

    pub fn to_string(&self) -> String {
        self.from.to_string()+"->"+&self.to.to_string()+"/"+&self.before.to_string()+"__"+&self.after.to_string()
    }
}
