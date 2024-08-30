use itertools::Itertools;

use crate::util::utilities::{concat_by_line, split_sc, unbracket_arr};

use super::{
    super::segment::{Feature, FeatureBundle, Segment},
    xsampa,
};

pub trait FeaturePhonObject: Sized+Clone {
    fn apply_seg_prim(&self, sc: &FeatureSoundChange) -> Self;
    fn apply_seg_prim_mut(&mut self, sc: &FeatureSoundChange);
    fn apply_seg_prim_vec(&self, scs: &[FeatureSoundChange]) -> Self {
        let mut tstr = self.clone();
        for sc in scs {
            tstr.apply_seg_prim_mut(sc);
        }
        tstr
    }
    fn apply_seg_prim_mut_vec(&mut self, scs: &[FeatureSoundChange]) {
        for sc in scs {
            self.apply_seg_prim_mut(sc);
        }
    }
}

pub enum Conjunct<T> {
    All(Vec<T>),
    Any(Vec<T>),
}
impl<T> Conjunct<T> {
    pub fn all_one(elem: T) -> Conjunct<T> {
        Conjunct::All(vec![elem])
    }
    pub fn any_one(elem: T) -> Conjunct<T> {
        Conjunct::Any(vec![elem])
    }

    pub fn unwrap(&self) -> &[T] {
        match self {
            Self::All(x) => &x,
            Self::Any(x) => &x,
        }
    }

    pub fn unwrap_mut(self) -> Vec<T> {
        match self {
            Self::All(x) => x,
            Self::Any(x) => x,
        }
    }
    
    pub fn is_empty(&self) -> bool {
        match self {
            Self::All(x) => x.is_empty(),
            Self::Any(x) => x.is_empty(),
        }
    }

}

#[derive(Debug)]
pub struct FeatureSoundChange {
    from: Vec<FeatureBundle>,  //from and before are a sequence of segments
    to: Vec<FeatureBundle>,    // represented by FeatureBundle
    before: Vec<FeatureBundle>,
    after: Vec<FeatureBundle>, 
}

impl FeatureSoundChange {
    pub fn new(str: &str) -> Option<Self> {
        let (str_from, str_to, str_before, str_after) = split_sc(str)?;

        let from = prs(&str_from)?;
        // println!("{:?}", from);
        let to = prs(&str_to)?;
        // println!("{:?}", to);
        let before = prs(&str_before)?;
        // println!("{:?}", before);
        let after = prs(&str_after)?;
        // println!("{:?}", from);
        println!("SegmentSoundChange::new() is working"); 
        Some(Self {
            from,
            to,
            before,
            after,
        })
    }

    //pub fn to_string(&self) -> String {
    //    let arr = &[
    //        Feature::list_features(),
    //        self.from.to_short_debug_string(),
    //        self.to.to_short_debug_string(),
    //        self.after.to_short_debug_string(),
    //        self.before.to_short_debug_string()
    //    ];
    //    util::concat_by_line(arr, "")
    //}
}

// #[derive(Clone, Debug)]
// pub struct SimpleSoundChange {
//     from: FeatureBundle,
//     to: FeatureBundle,
//     after: FeatureBundle,
//     before: FeatureBundle,
// }
// // [x,y,z]->[a,b,c]/[X,Y,Z]_[A,B,C]
// // [] represents and[], {} represents or{}
// impl<'a> SimpleSoundChange {
//     pub fn new(str: &str) -> Option<SimpleSoundChange> {
//         let (str_from, str_to, str_after, str_before) = split_sc(str)?;
//
//         let (from_fet, from_bool) = feature_pick(&str_from)?;
//         let mut from = FeatureBundle::new();
//         from.set(from_fet, from_bool);
//
//         let (to_fet, to_bool) = feature_pick(&str_to)?;
//         let mut to = FeatureBundle::new();
//         to.set(to_fet, to_bool);
//
//         let (before_fet, before_bool) = feature_pick(&str_before)?;
//         let mut before = FeatureBundle::new();
//         before.set(before_fet, before_bool);
//
//         let (after_fet, after_bool) = feature_pick(&str_after)?;
//         let mut after = FeatureBundle::new();
//         after.set(after_fet, after_bool);
//
//         Some(SimpleSoundChange {
//             from,
//             to,
//             after,
//             before,
//         })
//     }
//
//     pub fn to_string(&self) -> String {
//         let arr = &[
//             Feature::list_features(),
//             self.from.to_short_debug_string(),
//             self.to.to_short_debug_string(),
//             self.after.to_short_debug_string(),
//             self.before.to_short_debug_string(),
//         ];
//         concat_by_line(arr, "")
//     }
// }

impl Segment {
    fn apply(&self, fb: &[FeatureBundle]) -> Self {
        let mut seg = self.clone();

        for f in fb {
            seg.apply_simple_mut(f);
        }
        seg
    }

    fn apply_mut(&mut self, fb: &[FeatureBundle]) {
        for f in fb {
            self.apply_simple_mut(f);
        }
        self.tokens = xsampa::xsampa_split(&self.features.to_string());
    }

    fn apply_simple(&self, fb: &FeatureBundle) -> Self {
        let mut features = self.features;

        for f in super::FEATURES {
            match fb.get(f) {
                Some(b) => {
                    features.set(f, b);
                }
                None => {}
            }
        }

        Segment::from_feature_bundle(features)
    }

    fn apply_simple_mut(&mut self, fb: &FeatureBundle) {
        for f in super::FEATURES {
            match fb.get(f) {
                Some(b) => {
                    self.features.set(f, b);
                }
                None => {}
            }
        }

        let tokens = xsampa::xsampa_split_segments(&FeatureBundle::to_string(self.features));
        self.tokens = tokens;
    }
}

impl FeaturePhonObject for super::SegmentString {
    fn apply_seg_prim_mut(&mut self, sc: &FeatureSoundChange) {
        for i in 0..self.segments.len() {
            let o_before = &self.peek(i as isize - 1);
            let o_after = &self.peek(i as isize + 1);

            if self.segments[i].matches_all(&sc.from) {
                match (o_before, o_after) {
                    (Some(before), Some(after)) => {
                        if before.matches_all(&sc.before) && after.matches_all(&sc.after) {
                            self.segments[i].apply_mut(&sc.to)
                        }
                    }
                    (Some(before), None) => {
                        if before.matches_all(&sc.before) && sc.after.is_empty() {
                            self.segments[i].apply_mut(&sc.to)
                        }
                    }
                    (None, Some(after)) => {
                        if sc.before.is_empty() && after.matches_all(&sc.after) {
                            self.segments[i].apply_mut(&sc.to)
                        }
                    }
                    (None, None) => {}
                }
            }
        }
    }

    fn apply_seg_prim(&self, sc: &FeatureSoundChange) -> Self {
        let mut result_word = super::SegmentString::new();
        for i in 0..self.segments.len() {
            let prev_segment = &self.peek(i as isize - 1);
            let next_segment = &self.peek(i as isize + 1);

            let mut seg = self.segments[i].clone();
            if self.segments[i].matches_all(&sc.from) {
                match (prev_segment, next_segment) {
                    (Some(before), Some(after)) => {
                        if before.matches_all(&sc.before) && after.matches_all(&sc.after) { 
                            seg = seg.apply(&sc.to);
                        }
                    }
                    (Some(before), None) => { //Case: end of word
                        if before.matches_all(&sc.before) && sc.after.is_empty() { 
                            seg = seg.apply(&sc.to);
                        }
                    }
                    (None, Some(after)) => { //Case: beginning of word
                        if sc.before.is_empty() && after.matches_all(&sc.after) { 
                            seg = seg.apply(&sc.to);
                        }
                    } 
                    (None, None) => {} //Case: single segment word
                }
            }
            result_word.push(seg);
        }
        result_word
    }
}



// impl SimplePhonObject for super::SegmentString {
//     fn apply_simple_mut(&mut self, sc: &SimpleSoundChange) {
//         for i in 0..self.segments.len() {
//             let o_before = &self.peek(i as isize - 1);
//             let o_after = &self.peek(i as isize + 1);
//
//             if self.segments[i].matches_all(&vec![sc.from]) {
//                 match (o_before, o_after) {
//                     (Some(before), Some(after)) => {
//                         if before.matches(sc.before)
//                             && after.matches(sc.after)
//                         {
//                             self.segments[i].apply_simple_mut(&sc.to)
//                         }
//                     }
//                     (Some(before), None) => {
//                         println!("Hi3");
//                     }
//                     (None, Some(after)) => {}
//                     (None, None) => {}
//                 }
//             }
//         }
//     }
//
//     fn apply_simple(&self, sc: &SimpleSoundChange) -> Self {
//         let mut result_word = super::SegmentString::new();
//         for i in 0..self.segments.len() {
//             let o_before = &self.peek(i as isize - 1);
//             let o_after = &self.peek(i as isize + 1);
//
//             let mut seg = self.segments[i].clone();
//             if self.segments[i].matches(sc.from){
//                 match (o_before, o_after) {
//                     (Some(before), Some(after)) => {
//                         if before.matches(sc.before)
//                             && after.matches(sc.after)
//                         {
//                             seg = seg.apply_simple(&sc.to);
//                         }
//                     }
//                     (Some(before), None) => {
//                         println!("Hi3");
//                     }
//                     (None, Some(after)) => {}
//                     (None, None) => {}
//                 }
//             }
//             result_word.push(seg);
//         }
//         result_word
//     }
// }
//
// pub trait SimplePhonObject {
//     fn apply_simple(&self, sc: &SimpleSoundChange) -> Self;
//     fn apply_simple_mut(&mut self, sc: &SimpleSoundChange);
// }

pub fn feature_pick(str: &str) -> Option<(Feature, bool)> {
    let (str_1, str_2) = str.split_at(1);

    let b = match str_1 {
        "+" => Some(true),
        "-" => Some(false),
        _ => None,
    }?;

    Some((Feature::from_string(&str_2)?, b))
}

pub fn prs_fet(str: &str) -> Option<FeatureBundle> {
    let str_arr = unbracket_arr(str, ('[', ']')).unwrap_or(vec![str.to_string()]);
    let mut fet_bund = FeatureBundle::new();
    
    for s in str_arr.iter().filter(|&x| x!="") {
        let (fet, b) = feature_pick(&s)?;
        fet_bund.set(fet, b);

    }

    Some(fet_bund)
}



pub fn prs(str: &str) -> Option<Vec<FeatureBundle>> {
    if str.is_empty() {return Some(vec![])}
    let head = str.trim().chars().nth(0)?;
    match head {
        '(' => {
            let str_lis = unbracket_arr(str, ('(', ')'))?;
            let mut fet_arr = Vec::new();

            for s in str_lis {
                fet_arr.push(prs_fet(&s)?)
            }

            Some(fet_arr)
        },

        _   => {
                Some(vec![prs_fet(str)?])
        },
    }
}
