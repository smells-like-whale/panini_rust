use std::{fmt::Display, ops::{Deref, DerefMut}};

use itertools::Itertools;

use crate::{util::utilities::{concat_by_line, split_on_first_gen}, xsampa::{self, is_initial}};

use super::{
    feature::{feature_table::FeatureTable, FeatureBundle, FeatureIndex}, metches, segment_modifier::SegModifier, segment_sound_change::{
        SCApl, SCSegAn, SCSegment, SCString
    }, Feature, SegModif, SegmentShell, TokenString
};


//This represents an uninterrupted utterence, like a "word", or a sequence of "words", without "spaces"
#[derive(Clone, Debug)]
pub struct SegmentStringShell<F: FeatureBundle> {
    segments: Vec<SegmentShell<F>>,
}

impl<F: FeatureBundle> Display for SegmentStringShell<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = self.segments.iter().map(|s| s.to_string()).collect_vec().concat();

        write!(f,"{str}")
    }
}


impl<F: FeatureBundle> SegmentStringShell<F> {
    pub fn print_words(self_arr: &[Self]) -> String {
        let words: Vec<String> = self_arr.iter().map(|word| word.to_long_string()).collect();
        let mut x = vec!["\n\n".to_string() + &F::Index::list_features()];
        x.extend(words);
        concat_by_line(&x, "  ")
    }

    pub fn new() -> Self {
        let segments = Vec::new();
        Self { segments }
    }
    
    pub fn as_arr(&self) -> &[SegmentShell<F>] {
        &self.segments
    }

    pub fn as_mut_arr(&mut self) -> &mut [SegmentShell<F>] {
        &mut self.segments
    }

    pub fn from_arr<'a>(arr: &'a [SegmentShell<F>]) -> Self {
        SegmentStringShell { segments: arr.to_vec() }
    }

    pub fn remove(&mut self, i: usize) {
        self.segments.remove(i);
    }

    // pub fn last_mut(&mut self) -> Option<&mut Segment> {
    //     let get_mut = self.get_mut(self.len());
    //     get_mut 
    // }

    // pub fn clean(&mut self) {
    //     let mut temp: Option<&mut Segment> = None;
    //     for mut seg in self.as_mut_arr() {
    //         match temp {
    //             Some(prev) => {
    //                 match seg {
    //                     Segment::Seg(aseg) => {
    //                         if seg.is_initial() {
    //                             temp = Some(seg);
    //                         } else {
    //                             prev.tokens().append(aseg.tokens)
    //                         }
    //                     },
    //                     Segment::WordEdge => todo!(),
    //                 }
    //             },
    //             None => {
    //                 match seg {
    //                     Segment::Seg(_) => {
    //                         temp = Some(seg)
    //                     },
    //                     Segment::WordEdge => {temp = None},
    //                 }
    //             },// This should only happen at the start of the loop, or if self is empty
    //         }
    //     }
    // } 
    

    pub fn from_string(str: &str) -> Self {
        let bookended_str = "#".to_string()+str+"#";
        let segment_tokens = xsampa::xsampa_split_segments(&bookended_str);
        // println!("{:?}", segment_tokens);
        let segments: Vec<SegmentShell<F>> = segment_tokens
            .iter()
            .map(|token| SegmentShell::from_string(token))
            .collect();
        // let s = segments.iter().map(|s| s.to_string()).collect_vec();
        // println!("{:?}", s);
        Self { segments }
    }

    pub fn to_tokens(&self) -> TokenString {
        let mut str = Vec::new();

        for seg in self.as_arr() {
            match seg {
                SegmentShell::Seg(aseg) => str.push(aseg.tokens.concat()),
                SegmentShell::Escape(s) => str.push(s.to_string()),
                SegmentShell::WordEdge => {},
            } 
        }
        
        str
    }


    pub fn to_string(&self) -> String {
        self.segments.iter().map(
            |seg| seg.to_string()
        ).collect_vec().concat()
    }
    
    pub fn to_debug_string(&self) -> String {
        let mut str = String::new();
        
        for i in 0..self.segments.len() {
            str.push_str(&&self.segments[i].tokens().concat());
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

    pub fn push(&mut self, segment: SegmentShell<F>) {
        self.segments.push(segment);
    }

    pub fn append(&mut self, seg_arr: &[SegmentShell<F>]) {
        for seg in seg_arr {
            self.push(seg.clone());
        }
    }

    pub fn pop(&mut self) -> Option<SegmentShell<F>> {
        self.segments.pop()
    }

    pub fn peek(&self, i: isize) -> Option<SegmentShell<F>> {
        if i < 0 {
            return None;
        } else {
            Some(self.segments.get(i.unsigned_abs())?.to_owned())
        }
    }
    
    pub fn matches_forward(&self, sc_segs: &[SCSegAn<F>]) -> bool {
        //Special case of an empty segment
        if self.segments.len() == 0 {
            return sc_segs == &[SCSegAn::SCSeg(SCSegment::WordEdge)];
        }
        let mut i = 0;
        while i < sc_segs.len() {
            if sc_segs[i].is_skip() {
                // println!("Match: {} with {} ?",self.segments[i].to_string(), sc_segs[i+1]);
                let b = self.segments[i].matches_left(&sc_segs[i+1]);

                if !b {
                    let b_skipped = self.segments[i].matches_left(&sc_segs[i]);
                    if b_skipped {
                        // println!("Skipping {} to match: {} with {} ?",sc_segs[i+1], self.segments[i].to_string(),sc_segs[i+1]);
                        if !self.segments[i+1].matches_left(&sc_segs[i+1]) {
                            return false;
                        }
                    } else {
                        return false;
                    }
                    // i+=1;
                } 
                i+=1;
                
            } else if !self.segments[i].matches_left(&sc_segs[i]) {
                return false;
            }
            // if !self.segments[i].matches_left(&sc_segs[i]) {
            //     return false;
            // }
            i+=1;
        }
        true
    }
    
    //This is being finicky again
    pub fn matches_backwards(&self, sc_segs: &[SCSegAn<F>]) -> bool {
        let sc_len = sc_segs.len();
        let self_len = self.segments.len();

        if self_len == 0 {
            return sc_segs.is_empty();
        }

        let mut i = 0;
        while i < sc_segs.len() {
            // println!("self_len: {self_len}, i: {i}");
            let seg_opt = self.segments.get(self_len-i-1);
            // println!("{:?}", seg_opt);
            match seg_opt {
                Some(seg) => {
                    if sc_segs[sc_len-i-1].is_skip() {
                        // println!("Match: {} with {} ?",seg.to_string(), sc_segs[sc_len-i-2]);
                        let b = seg.matches_left(&sc_segs[sc_len-i-2]);

                        if !b {
                            let b_skipped = seg.matches_left(&sc_segs[sc_len-i-1]);
                            if b_skipped {
                                // println!("Skipping {} to match: {} with {} ?",sc_segs[sc_len-i-2], seg.to_string(),sc_segs[sc_len-i-2]);
                                if !self.segments[self_len-i-2].matches_left(&sc_segs[sc_len-i-2]) {
                                    return false;
                                }
                            } else {
                                return false;
                            }
                            // i+=1;
                        } 
                        i+=1;
                        
                    } else if !seg.matches_left(&sc_segs[sc_len-i-1]) {
                        return false;
                    }
                    // if !seg.matches_left(&sc_segs[sc_len-i-1]) {
                    //     return false;
                    // }
                    // i+=1;
                },
                None => {
                    return false;
                },
            }
            i+=1;
        }
        true
    }

    pub fn split_on_first<'a>(&'a self, scstring: &SCString<F>) -> Option<(Self, Self, Self)> {
        let (prefix, pivot, tail) = split_on_first_gen(&self.segments, &scstring.segments, &metches)?;
        Some(
            (
                SegmentStringShell { segments: prefix.to_vec() }, 
                SegmentStringShell { segments: pivot.to_vec() }, 
                SegmentStringShell { segments: tail.to_vec() }
            )
        )
    }
    
                    // println!("{self_str} against {scseg_str}");
    //Add the ability to add or remove segments
    pub fn apply_scstring(&self, scstr: &SCApl<F>) -> (Vec<SegModifier<F::Index>>, Self) {
        let mut result = SegmentStringShell::new();
        let mut modifs = Vec::new();
        let mut skip_first = false;

        match &scstr.segments.get(0) {
            Some(SCSegment::TokenSeg(seg_init)) => {
                // println!("I reached TokenSeg");
                if seg_init[0].starts_with(':') || seg_init[0].starts_with('_'){
                    
                    for modif in seg_init {
                        let temp = SegModifier::from_string(modif).unwrap_or(SegModifier::default());
                        modifs.push(temp);
                    }

                    skip_first = true;
                }
            },
            Some(SCSegment::FeatureSeg(_)) => {
                // println!("I reached FeatureSeg");
            },
            // Some(SCSegment::SkipSeg(_)) => {
            //     // println!("I reached SkipFeatureSeg");
            // },
            Some(SCSegment::WordEdge) => {
                // println!("I reached WordEdge");
            },
            None => {return (modifs, result)},
        }
       
        // println!("woot");
        let seg_tmp = scstr.to_segments();
        if seg_tmp.is_some() {
            // println!("I am a valid SegmentString");
            let mut seg = seg_tmp.unwrap();
            // println!("woot {:?}", seg);

            if skip_first {
                seg.remove(0);
            }
            // println!("{:?}{:?}", modifs, seg);
            return (modifs, seg);
        }
        

        let start = if skip_first {1} else {0};

        for i in start..self.segments.len() {
            match scstr.segments.get(i) {
                Some(val) => {
                    match val {
                        SCSegment::TokenSeg(tok) => {
                            // println!("I reached TokenSeg");
                            let new_seg = SegmentShell::from_string(&tok.concat());
                            result.push(new_seg);
                        },
                        SCSegment::FeatureSeg(fet) => {
                            // println!("fet == {fet:?}");
                            // println!("{:?}", self.segments[i]);
                            let new_seg = self.segments[i].clone().apply_feature_list_mut_and_return(fet); 
                            // println!("I pushed {new_seg:?}");
                            result.push(new_seg);
                        },
                        // SCSegment::SkipSeg(_vec) => todo!(),
                        SCSegment::WordEdge => eprintln!("segment_string::SegmentString::apply_scstring(): Cannot apply a word delimiter"),
                    } 
                },
                None => result.push(self.segments[i].clone()),
            }
        }

        (modifs, result)
    }

} 

impl<F: FeatureBundle> Deref for SegmentStringShell<F> {
    type Target = [SegmentShell<F>];

    fn deref(&self) -> &Self::Target {
        &self.segments
    }
}

impl<F: FeatureBundle> DerefMut for SegmentStringShell<F> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.segments
    }
}
