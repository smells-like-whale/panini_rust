use std::fmt::Display;

use crate::segment::segment_sound_change::SCSegment;
use feature::feature_table::{
    // feature_sound_change::FeatureSoundChange,
    FeatureTable,
};
use feature::{
    Feature, FeatureBundle, FeatureIndex, CONSONENT_CHART, NON_PULMONIC_CONSONANT_CHART,
    VOWEL_CHART,
};
use segment_modifier::SegModifier;
use segment_sound_change::{FetLis, SCSegAn};
// use token::token_sound_change::TokenSoundChange;

use crate::{util::utilities::concat_by_line, xsampa};

pub mod feature;
pub mod segment_modifier;
pub mod segment_sound_change;
pub mod segment_str;
pub mod segment_string;
//#[derive(Clone)]
//struct SegmentModifier {
//    label: String,
//    modifier: fn(FeatureBundle) -> FeatureBundle,
//}
//impl SegmentModifier {
//}

pub type TokenString = Vec<String>;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct SegModif<'a> {
    token: &'a str,
    modif: &'a [(Feature, bool)], //This is modified during initialisation, but could be hardcoded
}

impl<'a> SegModif<'a> {
    // pub fn new(token: &'a str, fet: Feature, b: bool) -> Self {
    //
    //     SegModif {
    //         token,
    //         modif: &[(fet, b)],
    //     }
    // }

    pub fn default() -> Self {
        SegModif {
            token: "",
            modif: &[],
        }
    }

    pub fn from_string(mod_token: &str) -> Option<Self> {
        for m in MODIFIERS {
            if mod_token == m.token {
                return Some(m.clone());
            }
        }
        None
    }
}

const MODIFIERS: &[SegModif] = &[
    SegModif {
        token: "",
        modif: &[],
    },
    SegModif {
        token: "_>",
        modif: &[(Feature::ConstrictedGlottis, true)],
    },
    SegModif {
        token: "_h",
        modif: &[(Feature::SpreadGlottis, true)],
    },
    SegModif {
        token: "_=",
        modif: &[(Feature::Syllabic, true)],
    },
    SegModif {
        token: "_~",
        modif: &[(Feature::Nasal, true)],
    },
    // SegModif {
    //     token: "_a",
    //     modif: FeatureTable::new().set_and_return(Feature::Syllabic, true),
    // },
    // SegModif {
    //     token: "_t",
    //     modif: FeatureTable::new().set_and_return(Feature::Nasal, true),
    // },
    SegModif {
        token: "_d",
        modif: &[(Feature::Distributed, true), (Feature::Anterior, true)],
    },
    SegModif {
        token: "_w",
        modif: &[
            (Feature::Labial, true),
            (Feature::Round, true),
            (Feature::Dorsal, true),
        ],
    },
    SegModif {
        token: "_j",
        modif: &[
            (Feature::Coronal, true),
            (Feature::Dorsal, true),
            (Feature::High, true),
        ],
    },
    SegModif {
        token: "_G",
        modif: &[
            (Feature::Dorsal, true),
            (Feature::High, true),
            (Feature::Back, true),
        ],
    },
    SegModif {
        token: "_?\\",
        modif: &[(Feature::Pharyngeal, true)],
    },
    SegModif {
        token: ":",
        modif: &[(Feature::Length, true)],
    },
];

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SegmentShell<F: FeatureBundle> {
    Seg(ASegment<F>),
    Escape(String),
    WordEdge,
}
impl<F: FeatureBundle> Display for SegmentShell<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SegmentShell::Seg(asegment) => write!(f, "{asegment}"),
            SegmentShell::Escape(str) => write!(f, "{str}"),
            SegmentShell::WordEdge => write!(f, ""),
        }
    }
}
impl<F: FeatureBundle> SegmentShell<F> {
    // fn apply_prim_feature_arr(&self, fb: &[AltFeatureTable]) -> Self {
    //     let mut seg = self.clone();
    //
    //     for f in fb {
    //         seg.apply_prim_feature_mut(f);
    //     }
    //     seg
    // }
    //
    // fn apply_prim_feature_arr_mut(&mut self, fb: &[AltFeatureTable]) {
    //     for f in fb {
    //         self.apply_prim_feature_mut(f);
    //     }
    //     self.refresh_tokens()
    // }

    pub fn apply_prim_feature(&self, fb: &F) -> Self {
        let mut features = self.features();

        for &f in F::Index::index() {
            match fb.get(f) {
                Some(b) => {
                    features.set(f, b);
                }
                None => {}
            }
        }

        SegmentShell::from_feature_bundle(features)
    }

    pub fn apply_feature_list(&self, fs: &[(F::Index, bool)]) -> Self {
        let mut result = self.clone();
        for &(f, b) in fs {
            result.set_feature(f, b);
        }
        result.update_token();
        result
    }

    pub fn apply_feature_list_mut(&mut self, fs: &[(F::Index, bool)]) {
        for &(f, b) in fs {
            self.set_feature(f, b);
        }
        self.update_token();
    }

    pub fn apply_feature_list_mut_and_return(mut self, fs: &[(F::Index, bool)]) -> Self {
        // println!("self == {self:?}");
        for &(f, b) in fs {
            // println!("{f:?} : {b}");
            self.set_feature(f, b);
        }
        // println!("Entered: apply_feature_list_mut_and_return");
        self.update_token();
        // println!("self == {self:?}");
        self
    }

    fn apply_prim_feature_mut(&mut self, fb: &F) {
        for &f in F::Index::index() {
            match fb.get(f) {
                Some(b) => {
                    self.set_feature(f, b);
                }
                None => {}
            }
        }
        self.refresh_tokens();
        // let tokens = xsampa::xsampa_split_segments(&FeatureBundle::to_string(self.features));
        // self.tokens = tokens;
    }
    pub fn new() -> Self {
        let s = ASegment::new();
        SegmentShell::Seg(s)
    }

    pub fn is_initial(&self) -> bool {
        let var_name = match self {
            SegmentShell::Seg(seg) => match seg.tokens[0].chars().nth(0) {
                Some(c) => !['_', ':', '~'].contains(&c),
                None => false,
            },
            SegmentShell::Escape(_) => true,
            SegmentShell::WordEdge => true,
        };
        var_name
    }

    pub fn from_feature_bundle(features: F) -> Self {
        let s = ASegment::from_feature_bundle(&features);
        SegmentShell::Seg(s)
    }

    pub fn add_modif(&mut self, modif: SegModifier<F::Index>) {
        match self {
            SegmentShell::Seg(s) => s.add_modif(modif),
            SegmentShell::Escape(_) => {
                eprintln!(
                    "Error@Segment::ASegment::add_modif(): Cannot modify Escape() enum variant"
                )
            }
            SegmentShell::WordEdge => {
                eprintln!(
                    "Error@Segment::ASegment::add_modif(): Cannot modify WordEdge enum variant"
                )
            }
        }
    }

    pub fn from_string(str: &str) -> Self {
        if str.starts_with('*') {
            return SegmentShell::Escape(str.strip_prefix('*').unwrap().to_string());
        }
        match str {
            "#" => SegmentShell::WordEdge,
            _ => SegmentShell::Seg(ASegment::from_string(str)),
        }
    }

    // pub fn to_string(&self) -> String {
    //     match self {
    //         SegmentShell::Seg(s) => xsampa::xsampa(&s.tokens.concat()),
    //         SegmentShell::Escape(s) => s.to_string(),
    //         SegmentShell::WordEdge => "".to_string(),
    //     }
    // }

    pub fn to_token(&self) -> String {
        match self {
            SegmentShell::Seg(s) => s.tokens.concat(),
            SegmentShell::Escape(s) => s.to_string(),
            SegmentShell::WordEdge => "".to_string(),
        }
    }

    pub fn to_debug_string(&self) -> String {
        match self {
            SegmentShell::Seg(seg) => seg.to_debug_string(),
            SegmentShell::Escape(s) => s.to_string(),
            SegmentShell::WordEdge => "#".to_string(),
        }
    }

    pub fn to_short_debug_string(&self) -> String {
        match self {
            SegmentShell::Seg(seg) => seg.to_short_debug_string(),
            SegmentShell::Escape(s) => s.to_string(),
            SegmentShell::WordEdge => "#".to_string(),
        }
    }

    pub fn matches_unannot(&self, scseg: &SCSegment<F>) -> bool {
        match self {
            SegmentShell::Seg(seg) => seg.matches_unannot(scseg),
            SegmentShell::Escape(s) => false,
            SegmentShell::WordEdge => scseg == &SCSegment::WordEdge,
        }
    }
    pub fn matches(&self, scseg: &SCSegAn<F>) -> bool {
        match scseg {
            SCSegAn::SCSeg(scsegment) => self.matches_unannot(scsegment),
            SCSegAn::Skip(skip_vec) => skip_vec.iter().any(|skip| self.matches_unannot(skip)),
            SCSegAn::SCMultiSeg(scseg_vec) => {
                scseg_vec.iter().any(|scseg| self.matches_unannot(scseg))
            }
        }
    }
    pub fn matches_left_unannot(&self, scseg: &SCSegment<F>) -> bool {
        match self {
            SegmentShell::Seg(seg) => seg.matches_left_unannot(scseg),
            SegmentShell::Escape(s) => false,
            SegmentShell::WordEdge => scseg == &SCSegment::WordEdge,
        }
    }
    pub fn matches_left(&self, scseg: &SCSegAn<F>) -> bool {
        match scseg {
            SCSegAn::SCSeg(scsegment) => self.matches_left_unannot(scsegment),
            SCSegAn::Skip(skip_vec) => skip_vec.iter().any(|skip| self.matches_left_unannot(skip)),
            SCSegAn::SCMultiSeg(scseg_vec) => scseg_vec
                .iter()
                .any(|scseg| self.matches_left_unannot(scseg)),
        }
    }
    pub fn matches_right_unannot(&self, scseg: &SCSegment<F>) -> bool {
        match self {
            SegmentShell::Seg(seg) => seg.matches_right_unannot(scseg),
            SegmentShell::Escape(s) => false,
            SegmentShell::WordEdge => scseg == &SCSegment::WordEdge,
        }
    }
    pub fn matches_right(&self, scseg: &SCSegAn<F>) -> bool {
        match scseg {
            SCSegAn::SCSeg(scsegment) => self.matches_right_unannot(scsegment),
            SCSegAn::Skip(skip_vec) => skip_vec.iter().any(|skip| self.matches_right_unannot(skip)),
            SCSegAn::SCMultiSeg(scseg_vec) => scseg_vec
                .iter()
                .any(|scseg| self.matches_right_unannot(scseg)),
        }
    }

    pub fn tokens(&self) -> TokenString {
        match self {
            SegmentShell::Seg(seg) => seg.tokens.clone(),
            SegmentShell::Escape(s) => vec![s.to_string()],
            SegmentShell::WordEdge => Vec::new(),
        }
    }

    pub fn features(&self) -> F {
        match self {
            SegmentShell::Seg(seg) => seg.features.clone(),
            SegmentShell::Escape(s) => F::new(),
            SegmentShell::WordEdge => F::new(),
        }
    }
    pub fn features_left(&self) -> F {
        match self {
            SegmentShell::Seg(seg) => seg.features.clone(),
            SegmentShell::Escape(s) => F::new(),
            SegmentShell::WordEdge => F::new(),
        }
    }
    pub fn features_right(&self) -> F {
        match self {
            SegmentShell::Seg(seg) => seg.features.clone(),
            SegmentShell::Escape(s) => F::new(),
            SegmentShell::WordEdge => F::new(),
        }
    }

    pub fn refresh_tokens(&mut self) {
        println!("SegmentShell::refresh_tokens line 313");
        match self {
            SegmentShell::Seg(s) => {
                let xsampa_split = xsampa::xsampa_split(&s.features.to_col_str());
                s.tokens = xsampa_split
            },
            SegmentShell::Escape(_) => eprintln!("Error@Segment::Escape()::refresh_tokens(): Escape enum variant does not contain Segment"),
            SegmentShell::WordEdge => eprintln!("Error@Segment::Segment::refresh_tokens(): WordEdge enum variant holds no tokens"),
        }
    }

    pub fn set_feature(&mut self, f: F::Index, b: bool) {
        match self {
            SegmentShell::Seg(s) => {
                // println!("set_feature({f:?},{b})");
                s.features.set(f, b);
                // s.features.set(f, b);
            },
            SegmentShell::Escape(_) => eprintln!("Error@Segment::Escape()::set_features(): Escape enum variant does not contain Segment"),
            SegmentShell::WordEdge => eprintln!("Error@Segment::Segment::set_features(): WordEdge enum variant holds no features"),
        }
    }

    pub fn update_token(&mut self) {
        match self {
            SegmentShell::Seg(seg) => {
                let f = seg.features;
                seg.tokens = xsampa::xsampa_split_segments(&f.print());
                },
            SegmentShell::Escape(_) => eprintln!("Error@Segment::Escape()::update_token(): Escape enum variant does not contain Segment"),
            SegmentShell::WordEdge => eprintln!("ERROR@Segment::WordEdge::update_token(): WordEdge has no token to update"),
        }
    }

    pub fn matches_all(&self, fet_arr: &[&FetLis<F::Index>]) -> bool {
        match self {
            SegmentShell::Seg(s) => s.matches_all(fet_arr),
            SegmentShell::Escape(_) => false,
            SegmentShell::WordEdge => false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ASegment<F: FeatureBundle> {
    pub tokens: TokenString, // A list containing a segment token and all its diacritics
    pub features: F, // The list of features for the segment in question, after diacritics are taken to account
}

impl<F: FeatureBundle> Display for ASegment<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = xsampa::xsampa(&self.tokens.concat());
        write!(f, "{str}")
    }
}

impl<F: FeatureBundle> ASegment<F> {
    //pub const fn new() -> Self {
    //    let token: Vec<String> = Vec::new();
    //    let features = SegmentFeatures::new();
    //    Segment { token, features }
    //}

    pub fn new() -> Self {
        let tokens = TokenString::new();
        let features = F::new();
        ASegment { tokens, features }
    }

    /*
    This, in effect, simply generates a label for the FeatureBundle.
    Maybe make this fail gracefully with result, allowing for best guesses to be
    distinguished from perfect matches.
    Result, or a similar custom type, could even tell which information is
    the original, and which is generated in this translation.
    */
    pub fn from_feature_bundle(f: &F) -> Self {
        let tokens = xsampa::xsampa_split(&f.print());
        let features = f.clone();
        //let features = FeatureBundle::from_string(&features.to_string());
        Self { tokens, features }
    }

    pub fn add_modif(&mut self, modif: SegModifier<F::Index>) {
        self.tokens.push(modif.token.to_string());
        self.features.modify(&modif);
    }

    // pub fn refresh_tokens(&mut self) {
    //         self.tokens = xsampa::xsampa_split_segments(&F::to_string(&self.features));
    // }

    //I really need to make this fail gracefully
    pub fn from_string(str: &str) -> Self {
        // if str == "#" {
        //     return Segment::new();
        // }
        //Consider making this output an Option<Self> type
        let segment_tokens: Vec<String> = xsampa::xsampa_split_segments(str);
        // println!("segment_tokens == {segment_tokens:?}");

        let tokens = xsampa::xsampa_split(&segment_tokens[0]);
        // println!("tokens == {tokens:?}");

        // println!("segment_tokens[0] == {}", &segment_tokens[0]);
        let mut f = F::from_string(&segment_tokens[0]);
        // println!("{f:?}");

        for t in tokens.iter().skip(1) {
            let modif = SegModifier::from_string(t);
            match modif {
                Some(modifier) => f.modify(&modifier),
                None => (),
            }
        }

        ASegment {
            tokens,
            features: f,
        }
    }

    pub fn to_debug_string(&self) -> String {
        let mut output = xsampa::xsampa(&self.tokens.concat());
        output.push_str(&format!("\n{}\n", self.features.to_col_str()));
        output.push_str(&format!("\n Base:      {}", &self.tokens[0]));
        output.push_str(&format!("\n Modifiers: {:?}", &self.tokens[1..]));
        output
    }

    pub fn to_short_debug_string(&self) -> String {
        //let mut str = self.tokens.concat();
        let mut str = xsampa::xsampa(&self.tokens.concat());
        // str.push_str("\n");
        str.push_str(&format!("\n{}", self.features.to_debug_string()));

        str
    }

    fn matches_any(&self, fet_arr: &[&FetLis<F::Index>]) -> bool {
        fet_arr.iter().any(|&fet| self.matches_feature(&fet))
    }
    fn matches_all(&self, fet_arr: &[&FetLis<F::Index>]) -> bool {
        fet_arr.iter().all(|&fet| self.matches_feature(&fet))
    }

    fn matches_token(&self, tok: &TokenString) -> bool {
        &self.tokens == tok
    }

    //This is just returning true :(
    //This returns true if all nonempty values match
    // fn matches_feature(&self, fs: &FetLis<>) -> bool {
    //     for &f in <F as FeatureBundle>::Index::index() {
    //         match fet_bundle.get(f) { // Ever get??
    //             None => {}
    //             Some(b) => {
    //                 if Some(b) != self.features.get(f) {
    //                     return false;
    //                 }
    //             }
    //         }
    //     }
    //     true
    // }

    fn matches_feature(&self, fs: &FetLis<F::Index>) -> bool {
        // let seg = self.tokens.concat();
        // println!("{fs:?}");
        for &(f, b) in fs {
            let fl = self.features.get_left(f);
            let fr = self.features.get_right(f);
            if Some(b) != fl || Some(b) != fr {
                // println!("Some({b}) doesn't match {fet:?} for feature {f:?} in segment {seg}");
                return false;
            }
        }
        // println!("I matched");
        true
    }

    fn matches_feature_left(&self, fs: &FetLis<F::Index>) -> bool {
        // let seg = self.tokens.concat();
        // println!("{fs:?}");
        for &(f, b) in fs {
            let fet = self.features.get_left(f);
            if Some(b) != fet {
                // println!("Some({b}) doesn't match {fet:?} for feature {f:?} in segment {seg}");
                return false;
            }
        }
        // println!("I matched");
        true
    }
    fn matches_feature_right(&self, fs: &FetLis<F::Index>) -> bool {
        // let seg = self.tokens.concat();
        // println!("{fs:?}");
        for &(f, b) in fs {
            let fet = self.features.get_right(f);
            if Some(b) != fet {
                // println!("Some({b}) doesn't match {fet:?} for feature {f:?} in segment {seg}");
                return false;
            }
        }
        // println!("I matched");
        true
    }

    pub fn matches_unannot(&self, sc_seg: &SCSegment<F>) -> bool {
        match sc_seg {
            SCSegment::TokenSeg(tok) => self.matches_token(tok),

            SCSegment::FeatureSeg(fet) => self.matches_feature(fet),

            SCSegment::WordEdge => false,
        }
    }

    pub fn matches(&self, sc_seg: &SCSegAn<F>) -> bool {
        match sc_seg {
            SCSegAn::SCSeg(scsegment) => self.matches_unannot(scsegment),
            SCSegAn::Skip(vec) => todo!(),
            SCSegAn::SCMultiSeg(scseg_vec) => {
                for scseg in scseg_vec {
                    let tmp = self.matches_unannot(scseg);
                    if tmp {
                        return true;
                    }
                }
                false
            }
        }
    }
    pub fn matches_left_unannot(&self, sc_seg: &SCSegment<F>) -> bool {
        match sc_seg {
            SCSegment::TokenSeg(tok) => self.matches_token(tok),

            SCSegment::FeatureSeg(fet) => self.matches_feature_left(fet),

            SCSegment::WordEdge => false,
        }
    }
    pub fn matches_left(&self, sc_seg: &SCSegAn<F>) -> bool {
        match sc_seg {
            SCSegAn::SCSeg(scsegment) => self.matches_left_unannot(scsegment),
            SCSegAn::Skip(vec) => todo!(),
            SCSegAn::SCMultiSeg(scseg_vec) => {
                for scseg in scseg_vec {
                    let tmp = self.matches_left_unannot(scseg);
                    if tmp {
                        return true;
                    }
                }
                false
            }
        }
    }
    pub fn matches_right_unannot(&self, sc_seg: &SCSegment<F>) -> bool {
        match sc_seg {
            SCSegment::TokenSeg(tok) => self.matches_token(tok),

            SCSegment::FeatureSeg(fet) => self.matches_feature_right(fet),

            SCSegment::WordEdge => false,
        }
    }
    pub fn matches_right(&self, sc_seg: &SCSegAn<F>) -> bool {
        match sc_seg {
            SCSegAn::SCSeg(scsegment) => self.matches_right_unannot(scsegment),
            SCSegAn::Skip(vec) => todo!(),
            SCSegAn::SCMultiSeg(scseg_vec) => {
                for scseg in scseg_vec {
                    let tmp = self.matches_right_unannot(scseg);
                    if tmp {
                        return true;
                    }
                }
                false
            }
        }
    }
}

pub fn metches<F: FeatureBundle>(seg: &SegmentShell<F>, sc_seg: &SCSegAn<F>) -> bool {
    match sc_seg {
        SCSegAn::SCSeg(scsegment) => matches_unannot(seg, scsegment),
        SCSegAn::Skip(vec) => todo!(),
        SCSegAn::SCMultiSeg(scseg_vec) => scseg_vec.iter().any(|scseg| seg.matches_unannot(scseg)),
        // SCSegAn::SCMultiSeg(scseg_vec) => {
        //     for scseg in scseg_vec {
        //         let tmp = seg.matches_unannot(scseg);
        //         if tmp { return true; }
        //     }
        //     false
        // },
    }
}

pub fn matches_unannot<F: FeatureBundle>(seg: &SegmentShell<F>, sc_seg: &SCSegment<F>) -> bool {
    match seg {
        SegmentShell::Seg(s) => s.matches_unannot(sc_seg),
        SegmentShell::Escape(_) => false,
        SegmentShell::WordEdge => false,
    }
}

impl<F: FeatureBundle> Default for ASegment<F> {
    fn default() -> Self {
        Self::new()
    }
}

// #[derive(Copy, Clone, Debug, Eq, PartialEq)]
// pub struct FeatureTable_depre {
//     //there are 21 values
//     consonental: Option<bool>,
//     sonorant: Option<bool>,
//     syllabic: Option<bool>,
//     approximant: Option<bool>,
//
//     labial: Option<bool>,
//     round: Option<bool>,
//
//     coronal: Option<bool>,
//     anterior: Option<bool>,
//     distributed: Option<bool>,
//
//     dorsal: Option<bool>,
//     high: Option<bool>,
//     low: Option<bool>,
//     front: Option<bool>,
//     back: Option<bool>,
//     tense: Option<bool>,
//
//     pharyngeal: Option<bool>,
//     //advanced_tongue_root : Option<bool>,
//     voice: Option<bool>,
//     spread_glottis: Option<bool>,
//     constricted_glottis: Option<bool>,
//
//     continuant: Option<bool>,
//     strident: Option<bool>,
//     lateral: Option<bool>,
//     delayed_release: Option<bool>,
//     nasal: Option<bool>,
//     length: Option<bool>, //modifiers: Vec<String>,
//                           // struct Cor {anterior: bool,distributed: bool,},
// }
//
// impl FeatureTable_depre {
//     pub const fn new() -> Self {
//         FeatureTable_depre {
//             consonental: Some(false),
//             sonorant: Some(false),
//             syllabic: Some(false),
//
//             approximant: Some(false),
//
//             labial: Some(false),
//             round: None,
//             coronal: Some(false),
//             anterior: None,
//             distributed: None,
//             dorsal: Some(false),
//             high: None,
//             low: None,
//             front: None,
//             back: None,
//             tense: None,
//             pharyngeal: Some(false),
//
//             voice: Some(false),
//             spread_glottis: Some(false),
//             constricted_glottis: Some(false),
//
//             continuant: Some(false),
//             strident: Some(false),
//             lateral: Some(false),
//             delayed_release: Some(false),
//             nasal: Some(false),
//             length: Some(false),
//         }
//     }
//     pub fn modify(&mut self, modifier: &SegModif) {
//         //self.modify_feature(modifier.modif[0].0, modifier.modif[0].1);
//         for &f in <FeatureTable_depre as FeatureBundle>::Index::index() {
//             match modifier.modif.get(f) {
//                 Some(b) => self.set(f, b),
//                 None => (),
//             }
//         }
//     }
//
//     const fn set_and_return(mut self, fet: Feature, b: bool) -> Self {
//         match fet {
//             Feature::Consonental => {
//                 self.consonental = Some(b);
//             }
//             Feature::Sonorant => {
//                 self.sonorant = Some(b);
//             }
//             Feature::Syllabic => {
//                 self.syllabic = Some(b);
//             }
//             // Feature::Approximant => {
//             //     self.approximant = Some(b);
//             // }
//             Feature::Labial => {
//                 self.labial = Some(b);
//             }
//             Feature::Round => {
//                 self.round = Some(b);
//             }
//             Feature::Coronal => {
//                 self.coronal = Some(b);
//             }
//             Feature::Anterior => {
//                 self.anterior = Some(b);
//             }
//             Feature::Distributed => {
//                 self.distributed = Some(b);
//             }
//             Feature::Dorsal => {
//                 self.dorsal = Some(b);
//             }
//             Feature::Front => {
//                 self.front = Some(b);
//             }
//             Feature::Back => {
//                 self.back = Some(b);
//             }
//             Feature::High => {
//                 self.high = Some(b);
//             }
//             Feature::Low => {
//                 self.low = Some(b);
//             }
//             Feature::AdvancedTongueRoot => {
//                 self.atr = Some(b);
//             }
//             Feature::Pharyngeal => {
//                 self.pharyngeal = Some(b);
//             }
//             Feature::Voice => {
//                 self.voice = Some(b);
//             }
//             Feature::SpreadGlottis => {
//                 self.spread_glottis = Some(b);
//             }
//             Feature::ConstrictedGlottis => {
//                 self.constricted_glottis = Some(b);
//             }
//
//             Feature::Continuant => {
//                 self.continuant = Some(b);
//             }
//             Feature::Strident => {
//                 self.strident = Some(b);
//             }
//             Feature::Lateral => {
//                 self.lateral = Some(b);
//             }
//             Feature::DelayedRelease => {
//                 self.delayed_release = Some(b);
//             }
//             Feature::Nasal => {
//                 self.nasal = Some(b);
//             }
//             Feature::Length => self.length = Some(b),
//         }
//         self
//     }
//     fn taxi_dist(x: &Self, y: &Self) -> usize {
//         let x_arr = x.to_isize_arr();
//         let y_arr = y.to_isize_arr();
//         FeatureTable_depre::_taxi_dist(x_arr, y_arr) as usize
//     }
//
//     fn _taxi_dist(x: [isize; 24], y: [isize; 24]) -> isize {
//         let mut val = 0;
//
//         for i in 0..24 {
//             val = val + (x[i] - y[i]).abs();
//         }
//         val
//     }
//
//     //add weights
//     fn to_isize_arr(self) -> [isize; 24] {
//         let mut fet_arr: [isize; 24] = [0; 24];
//
//         for i in 0..24 {
//             let weight = match FEATURES[i] {
//                 Feature::Continuant => 2,
//                 Feature::Strident => 3,
//                 _ => 1,
//             };
//             let val = match self.get(FEATURES[i]) {
//                 Some(true) => weight,
//                 Some(false) => -weight,
//                 None => 0,
//             };
//
//             fet_arr[i] = val;
//         }
//         fet_arr
//     }
// }
//
// impl FeatureBundle for FeatureTable_depre {
//
//     type Index = Feature;
//
//
//     fn new() -> Self {
//         todo!()
//     }
//
//     //const fn normalize(mut self) {
//     //    match self.labial {
//     //        Some(true) => {
//     //            if self.round.is_none() {
//     //                self.round = Some(false);
//     //            }
//     //        }
//     //        Some(false) => {
//     //            self.round = None;
//     //        },
//     //        None => {
//     //            self.labial = Some(false);
//     //            self.round = None;
//     //        },
//     //    }
//     //
//     //    match self.coronal {
//     //        Some(true) => {
//     //            for f in [Feature::Anterior, Feature::Distributed,] {
//     //                if self.f.is_none() {
//     //                    self.f = Some(false);
//     //                }
//     //            }
//     //        }
//     //        Some(false) => {
//     //            self.round = None;
//     //        },
//     //        None => {
//     //            self.round = None;
//     //        },
//     //    }
//     //    match self.labial {
//     //        Some(true) => {
//     //            if self.round.is_none() {
//     //                self.round = Some(false);
//     //            }
//     //        }
//     //        Some(false) => {
//     //            self.round = None;
//     //        },
//     //        None => {
//     //            self.round = None;
//     //        },
//     //    }
//     //    match self.labial {
//     //        Some(true) => {
//     //            if self.round.is_none() {
//     //                self.round = Some(false);
//     //            }
//     //        }
//     //        Some(false) => {
//     //            self.round = None;
//     //        },
//     //        None => {
//     //            self.round = None;
//     //        },
//     //    }
//     //}
//
//
//     fn get(&self, fet: Feature) -> Option<bool> {
//         match fet {
//             Feature::Consonental => self.consonental,
//             Feature::Sonorant => self.sonorant,
//             Feature::Syllabic => self.syllabic,
//
//             Feature::Approximant => self.approximant,
//
//             Feature::Labial => self.labial,
//             Feature::Round => self.round,
//             Feature::Coronal => self.coronal,
//             Feature::Anterior => self.anterior,
//             Feature::Distributed => self.distributed,
//             Feature::Dorsal => self.dorsal,
//             Feature::Front => self.front,
//             Feature::Back => self.back,
//             Feature::High => self.high,
//             Feature::Low => self.low,
//             Feature::Tense => self.tense,
//
//             Feature::Pharyngeal => self.pharyngeal,
//
//             Feature::Voice => self.voice,
//             Feature::SpreadGlottis => self.spread_glottis,
//             Feature::ConstrictedGlottis => self.constricted_glottis,
//
//             Feature::Continuant => self.continuant,
//             Feature::Strident => self.strident,
//             Feature::Lateral => self.lateral,
//             Feature::DelayedRelease => self.delayed_release,
//             Feature::Nasal => self.nasal,
//             Feature::Length => self.length,
//         }
//     }
//
//     //    pub const fn from_bool_list(bool_list: [bool; 24]) -> FeatureBundle {
//     //        FeatureBundle {
//     //            consonental: Some(bool_list[0]),
//     //            sonorant: Some(bool_list[1]),
//     //            syllabic: Some(bool_list[2]),
//     //
//     //            approximant: Some(bool_list[3]),
//     //
//     //            labial: Some(bool_list[4]),
//     //            round: Some(bool_list[5]),
//     //            coronal: Some(bool_list[6]),
//     //            anterior: Some(bool_list[7]),
//     //            distributed: Some(bool_list[8]),
//     //            dorsal: Some(bool_list[9]),
//     //            front: Some(bool_list[10]),
//     //            back: Some(bool_list[11]),
//     //            high: Some(bool_list[12]),
//     //            low: Some(bool_list[13]),
//     //            tense: Some(bool_list[14]),
//     //            pharyngeal: Some(bool_list[15]),
//     //
//     //            voice: Some(bool_list[16]),
//     //            spread_glottis: Some(bool_list[17]),
//     //            constricted_glottis: Some(bool_list[18]),
//     //
//     //            continuant: Some(bool_list[19]),
//     //            nasal: Some(bool_list[20]),
//     //            strident: Some(bool_list[21]),
//     //            lateral: Some(bool_list[22]),
//     //            delayed_release: Some(bool_list[23]),
//     //            //modifiers: vec![],
//     //        }
//     //    }
//
//     fn set(&mut self, fet: Feature, b: bool) {
//         match fet {
//             Feature::Consonental => {
//                 self.consonental = Some(b);
//             }
//             Feature::Sonorant => {
//                 self.sonorant = Some(b);
//             }
//             Feature::Syllabic => {
//                 self.syllabic = Some(b);
//             }
//             Feature::Approximant => {
//                 self.approximant = Some(b);
//             }
//             Feature::Labial => {
//                 self.labial = Some(b);
//             }
//             Feature::Round => {
//                 self.round = Some(b);
//             }
//             Feature::Coronal => {
//                 self.coronal = Some(b);
//             }
//             Feature::Anterior => {
//                 self.anterior = Some(b);
//             }
//             Feature::Distributed => {
//                 self.distributed = Some(b);
//             }
//             Feature::Dorsal => {
//                 self.dorsal = Some(b);
//             }
//             Feature::Front => {
//                 self.front = Some(b);
//             }
//             Feature::Back => {
//                 self.back = Some(b);
//             }
//             Feature::High => {
//                 self.high = Some(b);
//             }
//             Feature::Low => {
//                 self.low = Some(b);
//             }
//             Feature::Tense => {
//                 self.tense = Some(b);
//             }
//             Feature::Pharyngeal => {
//                 self.pharyngeal = Some(b);
//             }
//             Feature::Voice => {
//                 self.voice = Some(b);
//             }
//             Feature::SpreadGlottis => {
//                 self.spread_glottis = Some(b);
//             }
//             Feature::ConstrictedGlottis => {
//                 self.constricted_glottis = Some(b);
//             }
//             Feature::Continuant => {
//                 self.continuant = Some(b);
//             }
//             Feature::Strident => {
//                 self.strident = Some(b);
//             }
//             Feature::Lateral => {
//                 self.lateral = Some(b);
//             }
//             Feature::DelayedRelease => {
//                 self.delayed_release = Some(b);
//             }
//             Feature::Nasal => {
//                 self.nasal = Some(b);
//             }
//             Feature::Length => {
//                 self.length = Some(b);
//             }
//         }
//     }
//
//
//
//     fn to_string(&self) -> String {
//         let cons_chart = CONSONENT_CHART.concat();
//         let non_pulm_cons_chart = NON_PULMONIC_CONSONANT_CHART.concat();
//         let vow_chart = VOWEL_CHART.concat();
//
//         let chart = [cons_chart, non_pulm_cons_chart, vow_chart].concat();
//
//         //let fet_arr = self.to_isize_arr();
//         let mut current_closest: usize = usize::MAX;
//         let mut str: &str = "";
//         //let c_iter: Vec<&&str> = chart.iter().filter(|&x| x!=&"").collect();
//         //println!("{:?}", c_iter);
//         for s in chart.iter().filter(|&x| x != &"") {
//             //let s_arr = FeatureBundle::from_string(s).to_isize_arr();
//             let dist = FeatureTable_depre::taxi_dist(&self, &FeatureTable_depre::from_string(s));
//
//             if dist < current_closest {
//                 current_closest = dist;
//                 str = s;
//             }
//         }
//         let mut base_fet = FeatureTable_depre::from_string(str);
//         //println!("{}", self.to_debug_string());
//
//         let mut result = str.to_owned();
//         for _ in 0..3 {
//             let mut temp_mod: SegModif = SegModif::default();
//             for m in MODIFIERS {
//                 let mut temp_base_fet = base_fet.clone();
//
//                 temp_base_fet.modify(&m);
//
//                 let mod_dist = FeatureTable_depre::taxi_dist(&temp_base_fet, &self);
//
//                 if mod_dist < current_closest {
//                     current_closest = mod_dist;
//                     temp_mod = m.clone();
//                 }
//             }
//             base_fet.modify(&temp_mod);
//             result += temp_mod.token;
//         }
//         result
//     }
//
//     //fn to_char(&self) -> Option<char> {
//     //    eprintln!("No implementation yet!");
//     //    None
//     //}
//
//     //fn is_vowel(&self) -> bool {
//     //    self.syllabic == Some(true)
//     //        && self.consonental == Some(false)
//     //        && self.sonorant == Some(true)
//     //        && self.approximant == Some(true)
//     //        && self.continuant == Some(true)
//     //}
//
//     fn from_string(_s: &str) -> FeatureTable_depre {
//         let tokens = xsampa::xsampa_split(_s);
//         let mut token_iterator = tokens.iter().map(|x| xsampa::token_type(x));
//         //Check if initial is a valid segmental token
//         // if token_iterator.next() != Some((true, false)) {
//         //     eprintln!("Error: Not a valid initial\n");
//         //     println!("{:?}\n", tokens);
//         // }
//         //Check if the trailing tokens are valid non-segmental tokens
//         // for temp in token_iterator {
//         //     if temp != (true, true) {
//         //         eprintln!("Error: Invalid token found\n");
//         //         println!("{:?}\n", tokens);
//         //     }
//         // }
//
//         //let modifs: Vec<String> = tokens.iter().skip(1).map(|x| x.to_string()).collect();
//         //Is the initial a consonent?
//         let init = &tokens[0];
//         let mut m: u8 = 0;
//         let mut p: u8 = 0;
//         let mut c: u8 = 0;
//
//         let mut fet = FeatureTable_depre::new();
//
//         for list in CONSONENT_CHART {
//             for token in list {
//                 if init == token {
//                     fet = _f(c, m, p);
//                 }
//                 p += 1;
//             }
//             p = 0;
//             m += 1;
//         }
//         c += 1;
//         m = 0;
//         p = 0;
//
//         for list in NON_PULMONIC_CONSONANT_CHART {
//             for token in list {
//                 if init == token {
//                     fet = _f(c, m, p);
//                 }
//                 p += 1;
//             }
//         }
//         c += 1;
//         m = 0;
//         p = 0;
//
//         for list in VOWEL_CHART {
//             for token in list {
//                 if init == token {
//                     fet = _f(c, m, p);
//                 }
//                 p += 1;
//             }
//             p = 0;
//             m += 1;
//         }
//         if fet == FeatureTable_depre::new() {
//             eprintln!("ERROR: no valid token found");
//         }
//
//         //for t in modifs {
//         //    let modif = SegModif::from_string(&t);
//         //    match modif {
//         //        Some(modifier) => fet.modify(&modifier),
//         //        None =>(),
//         //    }
//         //}
//         fet
//     }
//
//     fn print(&self) -> String {
//         let mut result = String::new();
//         result.push('[');
//
//         for &fet in FEATURES {
//             match self.get(fet) {
//                 Some(b) => if b {
//                     result.push('+');
//                     result.push_str(fet.to_short_string());
//                     result.push(',');
//                 },
//                 None => {},
//             }
//         }
//         result.push(']');
//         result
//     }
//
//     fn to_debug_string(&self) -> String {
//         let mut str = String::new();
//
//         for &f in FEATURES {
//             if [Feature::Labial, Feature::Coronal, Feature::Dorsal, Feature::Pharyngeal, Feature::Voice].contains(&f) {
//                 str.push('\n');
//             }
//             match self.get(f) {
//                 Some(true) => {
//                     str.push_str("+");
//                     str.push_str(f.to_string());
//                 }
//                 Some(false) => {
//                     str.push_str("-");
//                     str.push_str(f.to_string());
//                 }
//                 None => str.push_str("∅"),
//             }
//             str.push('\n')
//         }
//
//         str
//     }
//
//     fn to_short_debug_string(&self) -> String {
//         let mut output = String::new();
//
//         for &f in FEATURES {
//             if [Feature::Labial, Feature::Coronal, Feature::Dorsal, Feature::Pharyngeal, Feature::Voice].contains(&f) {
//                 output.push('\n');
//             }
//             match self.get(f) {
//                 Some(true) => output.push_str("+\n"),
//                 Some(false) => output.push_str("-\n"),
//                 None => output.push_str("∅\n"),
//             }
//         }
//         output
//     }
//
//     fn modify(&mut self, modifier: &SegModif) {
//         //self.modify_feature(modifier.modif[0].0, modifier.modif[0].1);
//         for &f in FEATURES {
//             match modifier.modif.get(f) {
//                 Some(b) => self.set(f, b),
//                 None => (),
//             }
//         }
//     }
// }
//
// fn _f(c: u8, m: u8, p: u8) -> FeatureTable_depre {
//     let mut temp: FeatureTable_depre = FeatureTable_depre::new();
//     temp.length = Some(false);
//     match c {
//         0 => {
//             temp.syllabic = Some(false);
//             temp.consonental = Some(true);
//             temp.spread_glottis = Some(false);
//             temp.constricted_glottis = Some(false);
//             temp.voice = Some(p % 2 != 0);
//             temp.tense = Some(true);
//
//             match m {
//                 0 => {
//                     //plosive
//                     temp.approximant = Some(false);
//                     temp.sonorant = Some(false);
//                     temp.continuant = Some(false);
//                     temp.nasal = Some(false);
//                     temp.strident = Some(false);
//                     temp.lateral = Some(false);
//                     temp.delayed_release = Some(false);
//                 }
//                 1 => {
//                     //Strident Affricates
//                     temp.approximant = Some(false);
//                     temp.sonorant = Some(false);
//                     temp.continuant = Some(true);
//                     temp.nasal = Some(false);
//                     temp.strident = Some(true);
//                     temp.lateral = Some(false);
//                     temp.delayed_release = Some(true);
//                 }
//                 2 => {
//                     //fricative
//                     temp.approximant = Some(false);
//                     temp.sonorant = Some(false);
//                     temp.continuant = Some(true);
//                     temp.nasal = Some(false);
//                     temp.strident = Some(false);
//                     temp.lateral = Some(false);
//                     temp.delayed_release = Some(false);
//                 }
//                 3 => {
//                     //Strident Fricative
//                     temp.approximant = Some(false);
//                     temp.sonorant = Some(false);
//                     temp.continuant = Some(true);
//                     temp.nasal = Some(false);
//                     temp.strident = Some(true);
//                     temp.lateral = Some(false);
//                     temp.delayed_release = Some(false);
//                 }
//                 4 => {
//                     //nasal
//                     temp.approximant = Some(false);
//                     temp.sonorant = Some(true);
//                     temp.continuant = Some(false);
//                     temp.nasal = Some(true);
//                     temp.strident = Some(false);
//                     temp.lateral = Some(false);
//                     temp.delayed_release = Some(false);
//                 }
//                 5 => {
//                     //Lateral fricatives
//                     temp.approximant = Some(false);
//                     temp.sonorant = Some(true);
//                     temp.continuant = Some(true);
//                     temp.nasal = Some(true);
//                     temp.strident = Some(false);
//                     temp.lateral = Some(true);
//                     temp.delayed_release = Some(false);
//                 }
//                 6 => {
//                     //trills
//                     temp.approximant = Some(false);
//                     temp.sonorant = Some(true);
//                     temp.continuant = Some(true);
//                     temp.nasal = Some(false);
//                     temp.strident = Some(true); //Strident is used to distinguish trills from taps
//                     temp.lateral = Some(false);
//                     temp.delayed_release = Some(false);
//                 }
//                 7 => {
//                     //approximates
//                     temp.approximant = Some(true);
//                     temp.sonorant = Some(true);
//                     temp.continuant = Some(true);
//                     temp.nasal = Some(false);
//                     temp.strident = Some(false);
//                     temp.lateral = Some(false);
//                     temp.delayed_release = Some(false);
//                 }
//                 8 => {
//                     //taps
//                     temp.approximant = Some(false);
//                     temp.sonorant = Some(true);
//                     temp.continuant = Some(false);
//                     temp.nasal = Some(false);
//                     temp.strident = Some(false);
//                     temp.lateral = Some(false);
//                     temp.delayed_release = Some(false);
//                 }
//                 9 => {
//                     //Lateral
//                     temp.approximant = Some(true);
//                     temp.sonorant = Some(true);
//                     temp.continuant = Some(true);
//                     temp.nasal = Some(false);
//                     temp.strident = Some(false);
//                     temp.lateral = Some(true);
//                     temp.delayed_release = Some(false);
//                 }
//                 _ => {}
//             }
//
//             match p / 2 {
//                 0 => {
//                     //bilabial
//                     temp.labial = Some(true);
//                     temp.coronal = Some(false);
//                     //temp.anterior    = Some(false);
//                     //temp.distributed = Some(false);
//                     temp.dorsal = Some(false);
//                     temp.pharyngeal = Some(false);
//                 }
//                 1 => {
//                     //labiodental
//                     temp.labial = Some(true);
//                     temp.coronal = Some(false);
//                     temp.strident = Some(true);
//                     //temp.anterior    = Some(false);
//                     //temp.distributed = Some(false);
//                     temp.dorsal = Some(false);
//                     temp.pharyngeal = Some(false);
//                 }
//                 2 => {
//                     //dental
//                     temp.labial = Some(false);
//                     temp.coronal = Some(true);
//                     temp.anterior = Some(true);
//                     temp.distributed = Some(true);
//                     temp.dorsal = Some(false);
//                     temp.pharyngeal = Some(false);
//                 }
//                 3 => {
//                     //alveolar
//                     temp.labial = Some(false);
//                     temp.coronal = Some(true);
//                     temp.anterior = Some(true);
//                     temp.distributed = Some(false);
//                     temp.dorsal = Some(false);
//                     temp.pharyngeal = Some(false);
//                 }
//                 4 => {
//                     //postalveolar
//                     temp.labial = Some(false);
//                     temp.coronal = Some(true);
//                     temp.anterior = Some(false);
//                     temp.distributed = Some(true);
//                     temp.dorsal = Some(false);
//                     temp.pharyngeal = Some(false);
//                 }
//                 5 => {
//                     //retroflex
//                     temp.labial = Some(false);
//                     temp.coronal = Some(true);
//                     temp.anterior = Some(false);
//                     temp.distributed = Some(false);
//                     temp.dorsal = Some(false);
//                     temp.pharyngeal = Some(false);
//                 }
//                 6 => {
//                     //palatal
//                     temp.labial = Some(false);
//                     temp.coronal = Some(true);
//                     temp.anterior    = Some(false);
//                     temp.distributed = Some(false);
//                     temp.dorsal = Some(true);
//                     temp.high = Some(true);
//                     temp.low = Some(false);
//                     temp.front = Some(false);
//                     temp.back = Some(false);
//                     temp.pharyngeal = Some(false);
//                 }
//                 7 => {
//                     //velar
//                     temp.labial = Some(false);
//                     temp.coronal = Some(false);
//                     //temp.anterior    = Some(false);
//                     //temp.distributed = Some(false);
//                     temp.dorsal = Some(true);
//                     temp.high = Some(false);
//                     temp.low = Some(false);
//                     temp.front = Some(false);
//                     temp.back = Some(false);
//                     temp.tense = Some(true);
//                     temp.pharyngeal = Some(false);
//                 }
//                 8 => {
//                     //uvular
//                     temp.labial = Some(false);
//                     temp.coronal = Some(false);
//                     //temp.anterior    = Some(false);
//                     //temp.distributed = Some(false);
//                     temp.dorsal = Some(true);
//                     temp.high = Some(false);
//                     temp.low = Some(false);
//                     temp.front = Some(false);
//                     temp.back = Some(true);
//                     temp.pharyngeal = Some(false);
//                 }
//                 9 => {
//                     //pharyngeal
//                     temp.labial = Some(false);
//                     temp.coronal = Some(false);
//                     //temp.anterior    = Some(false);
//                     //temp.distributed = Some(false);
//                     temp.dorsal = Some(false);
//                     temp.pharyngeal = Some(true);
//                 }
//                 10 => {
//                     //glottal
//                     temp.labial = Some(false);
//                     temp.coronal = Some(false);
//                     //temp.anterior    = Some(false);
//                     //temp.distributed = Some(false);
//                     temp.dorsal = Some(false);
//                     temp.pharyngeal = Some(false);
//                 }
//                 _ => {}
//             }
//         }
//
//         1 => {
//             temp.syllabic = Some(false);
//             temp.consonental = Some(true);
//             temp.sonorant = Some(false);
//             match m {
//                 0 => {
//                     temp.approximant = Some(false);
//                     temp.continuant = Some(false);
//                     temp.nasal = Some(false);
//                     temp.strident = Some(false);
//                     temp.lateral = Some(false);
//                     temp.delayed_release = Some(false);
//                     temp.voice = Some(true);
//                     temp.constricted_glottis = Some(true);
//                 }
//                 1 => {
//                     temp = FeatureTable_depre::new();
//                     eprint!("ERROR: invalid or undefined token");
//                     //temp.approximant = Some(false);
//                     //temp.sonorant = Some(false);
//                     //temp.continuant = Some(false);
//                     //temp.nasal = Some(false);
//                     //temp.strident =Some(false);
//                     //temp.lateral = Some(false);
//                     //temp.delayed_release = Some(false);
//                 }
//                 _ => {}
//             }
//
//             match p {
//                 0 => {
//                     temp.labial = Some(true);
//                     temp.coronal = Some(false);
//                     temp.dorsal = Some(false);
//                     temp.pharyngeal = Some(false);
//                 }
//                 1 => {
//                     temp.labial = Some(false);
//                     temp.coronal = Some(true);
//                     temp.anterior = Some(true);
//                     temp.distributed = Some(false);
//                     temp.dorsal = Some(false);
//                     temp.pharyngeal = Some(false);
//                 }
//                 2 => {
//                     temp.labial = Some(false);
//                     temp.coronal = Some(false);
//                     temp.dorsal = Some(true);
//                     temp.high = Some(true);
//                     temp.low = Some(false);
//                     temp.front = Some(false);
//                     temp.back = Some(false);
//                     temp.pharyngeal = Some(false);
//                 }
//                 3 => {
//                     temp.labial = Some(false);
//                     temp.coronal = Some(false);
//                     temp.dorsal = Some(true);
//                     temp.high = Some(true);
//                     temp.low = Some(false);
//                     temp.front = Some(false);
//                     temp.back = Some(true);
//                     temp.pharyngeal = Some(false);
//                 }
//                 4 => {
//                     temp.labial = Some(false);
//                     temp.coronal = Some(false);
//                     temp.dorsal = Some(true);
//                     temp.high = Some(false);
//                     temp.low = Some(true);
//                     temp.front = Some(false);
//                     temp.back = Some(true);
//                     temp.pharyngeal = Some(false);
//                 }
//                 _ => {}
//             }
//         }
//         2 => {
//             // VOWELS
//             //temp.sonorant = Some(true);
//             temp.consonental = Some(false);
//             temp.sonorant = Some(true);
//             temp.syllabic = Some(true);
//             temp.approximant = Some(true); //Can probably be removed entirely
//
//             temp.labial = Some(true);
//             temp.round = Some(false);
//
//             temp.coronal = Some(false);
//
//             temp.dorsal = Some(true);
//
//             temp.voice = Some(true);
//             temp.spread_glottis = Some(false);
//             temp.continuant = Some(true);
//             temp.nasal = Some(false);
//             temp.strident = Some(false);
//             temp.lateral = Some(false);
//             temp.constricted_glottis = Some(false);
//             temp.delayed_release = Some(false);
//             match m {
//                 //Height
//                 0 => {
//                     //Semivowel
//                     temp.syllabic = Some(false);
//                     temp.high = Some(true);
//                     temp.low = Some(false);
//                     temp.tense = Some(true);
//                 }
//                 1 => {
//                     //close/high tense vowel
//                     temp.high = Some(true);
//                     temp.low = Some(false);
//                     temp.tense = Some(true);
//                 }
//                 2 => {
//                     //near close/lax high vowel
//                     temp.high = Some(true);
//                     temp.low = Some(false);
//                     temp.tense = Some(false);
//                 }
//                 3 => {
//                     //mid close/mid tense vowel
//                     temp.high = Some(false);
//                     temp.low = Some(false);
//                     temp.tense = Some(true);
//                 }
//                 4 => {
//                     //mid vowel/central
//                     temp.high = Some(false);
//                     temp.low = Some(false);
//                     temp.tense = Some(false);
//                 }
//                 5 => {
//                     //mid open vowel/mid lax
//                     temp.high = Some(false);
//                     temp.low = Some(false);
//                     temp.tense = Some(false);
//                 }
//                 6 => {
//                     //near open vowel/low lax
//                     temp.high = Some(false);
//                     temp.low = Some(true);
//                     temp.tense = Some(false);
//                 }
//                 7 => {
//                     //open vowwel/low tense
//                     temp.high = Some(false);
//                     temp.low = Some(true);
//                     temp.tense = Some(true);
//                 }
//                 _ => {}
//             }
//
//             temp.round = Some(p % 2 == 1);
//
//             match p / 2 {
//                 0 => {
//                     temp.front = Some(true);
//                     temp.back = Some(false);
//                 }
//                 1 => {
//                     temp.front = Some(false);
//                     temp.back = Some(false);
//                 }
//                 2 => {
//                     temp.front = Some(false);
//                     temp.back = Some(true);
//                 }
//                 _ => {}
//             }
//         }
//         _ => {}
//     }
//     temp //.clone()
// }

//static BLANK_SEGMENT: Segment =
//[
//    (false, Syllabic),
//    (false, Consonental),
//    (false, Approximate),
//    (false, Sonorant),
//
//    (false, Voice),
//    (false, SpreadGlottis),
//    (false, ConstrictedGlottis),
//
//    (false, Continuant),
//    (false, Nasal),
//    (false, Strident),
//    (false, Lateral),
//    (false, DelayedRelease),
//
//    (false, Labial),
//    (false, Coronal),
//    (false, Dorsal),
//    (false, Radical),
//    (false, Glottal),
//
//    (false, Front),
//    (false, Raised),
//    (false, Round),
//    (false, Retracted)
//];

//type Segment = [(bool,Feature); 21];
//type Change = (Segment,Segment,Segment,Segment);

//fn compose_two<A,B,C,F,G>(_f: F,_g: G) -> impl Fn(A) -> C
//    where
//        F: Fn(A) -> B,
//        G: Fn(B) -> C,
//{
//    move |_x| _g(_f(_x))
//}
//
//macro_rules! compose {
//    ( $last:expr ) => { $last };
//    ( $head:expr, $($tail:expr), +) => {
//        compose_two($head, compose!($($tail),+))
//    };
//}

pub fn print_xsampa_chart(b: bool) -> String {
    let f = if b {
        |x: &str| x.to_string()
    } else {
        |x: &str| xsampa::xsampa(x)
    };

    let cons_chart = &print_grid(&CONSONENT_CHART, &f);
    let non_pul_chart = &print_grid(&NON_PULMONIC_CONSONANT_CHART, &f);
    let vowel_chart = &print_grid(&VOWEL_CHART, &f);
    let sep = "-------------------------------------------\n";
    let fet = "Class:
     Consonental
     Sonorant
     Syllabic
     Approximant
Place:
     Labial
        Round
     Coronal
        Anterior
        Distributed
     Dorsal
        High
        Low
        Front
        Back
        Tense
Laryngeal:
     Pharyngeal
     Voice
     SpreadGlottis
     ConstrictedGlottis
Manner:
     Continuant
     Nasal
     Strident
     Lateral
     DelayedRelease";

    String::new() + sep + cons_chart + sep + non_pul_chart + sep + vowel_chart + sep + fet
}

pub fn print_ipa_chart() -> String {
    let f = &xsampa::xsampa;
    let cons_chart = &print_grid(&CONSONENT_CHART, f);
    let non_pul_chart = &print_grid(&NON_PULMONIC_CONSONANT_CHART, f);
    let vowel_chart = &print_grid(&VOWEL_CHART, f);
    let sep = "-------------------------------------------\n";
    let fet = "Class:
     Consonental
     Sonorant
     Syllabic
     Approximant
Place:
     Labial
        Round
     Coronal
        Anterior
        Distributed
     Dorsal
        High
        Low
        Front
        Back
        Tense
Laryngeal:
     Pharyngeal
     Voice
     SpreadGlottis
     ConstrictedGlottis
Manner:
     Continuant
     Nasal
     Strident
     Lateral
     DelayedRelease";

    String::new() + sep + cons_chart + sep + non_pul_chart + sep + vowel_chart + sep + fet
}

pub fn print_grid<const M: usize, const N: usize>(
    grid: &[[&str; M]; N],
    f: &dyn Fn(&str) -> String,
) -> String {
    let mut grid_str = String::new();
    for j in 0..M {
        let col = print_ipa_col(grid, j, f);
        grid_str = concat_by_line(&[grid_str, col], "");
    }
    grid_str
}

pub fn print_col<const M: usize, const N: usize>(grid: &[[&str; M]; N], x: usize) -> String {
    let mut col = String::new();

    for i in 0..N {
        let tmp = grid[i][x].to_string();
        col.push_str(&format!("{} \n", tmp));
    }

    col
}

pub fn print_ipa_col<const M: usize, const N: usize>(
    grid: &[[&str; M]; N],
    x: usize,
    f: &dyn Fn(&str) -> String,
) -> String {
    let mut col = String::new();

    for i in 0..N {
        let tmp = match grid[i][x] {
            "" => " ".to_string(),
            _ => f(&grid[i][x]),
        };
        //col.push_str(&format!("{}\n", CONSONENT_CHART[i][x]));
        col.push_str(&format!("{} \n", tmp));
        //println!("{}", col);
    }

    col
}

// static CONSONENT_CHART: [[&str; 22]; 10] = [
//     [
//         "p", "b", "", "", "", "", "t", "d", "", "", "t`", "d`", "c", "J\\", "k", "g", "q", "G\\",
//         "", "", "?", "",
//     ],
//     [
//         "", "", "", "", "", "", "t_s", "d_z", "t_S", "d_Z", "t_s`", "d_z`", "t_s\\", "d_z\\", "",
//         "", "", "", "", "", "", "",
//     ],
//     [
//         "p\\", "B", "f", "v", "T", "D", "", "", "", "", "", "", "C", "j\\", "x", "G", "X", "R",
//         "X\\", "?\\", "h", "h\\",
//     ],
//     [
//         "", "", "", "", "", "", "s", "z", "S", "Z", "s`", "z`", "s\\", "z\\", "", "", "", "", "",
//         "", "", "",
//     ],
//     [
//         "", "m", "", "F", "", "", "", "n", "", "", "", "n`", "", "J", "", "N", "", "N\\", "", "",
//         "", "",
//     ],
//     [
//         "", "", "", "", "", "", "K", "K\\", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
//     ],
//     [
//         "", "B\\", "", "", "", "", "", "r", "", "", "", "", "", "", "", "", "", "R\\", "", "", "", "",
//     ],
//     [
//         "", "", "", "v\\", "", "", "", "r\\", "", "", "", "r\\`", "", "", "", "", "", "", "", "",
//         "", "",
//     ],
//     [
//         "", "", "", "", "", "", "", "4", "", "", "", "r`", "", "", "", "", "", "", "", "", "", "",
//     ],
//     [
//         "", "", "", "", "", "", "", "l", "", "", "", "l`", "", "L", "", "L\\", "", "", "", "", "",
//         "",
//     ],
// ];
//
// static NON_PULMONIC_CONSONANT_CHART: [[&str; 5]; 2] = [
//     //exludes ejectives for reasons
//     ["b_<", "d_<", "f_<", "g_<", "G_<"],
//     ["O\\", "|\\", "!\\", "=\\", "|\\|"], // |\\|\\ is not being parsed properly
// ];
// //Front     central     "back"
// //needs 3 features to describe height???
// pub static VOWEL_CHART: [[&str; 6]; 8] = [
//     ["j", "H",    "",    "", "M\\", "w"],
//     ["i", "y",   "1",   "}",   "M", "u"], //+high +tense -mid -low OR +raised -retracted
//     ["I", "Y",    "",    "",    "", "U"],    //                       OR +raised +retracted
//     ["e", "2", "@\\",   "8",   "7", "o"], //+high +tense +mid -low OR -raised -retracted
//     [ "",  "",   "@",    "",    "", ""],      //-high -tense +mid -low OR -raised
//     ["E", "9",   "3", "3\\",   "V", "O"], //-high -tense +mid +low OR -raised +retracted
//     [ "",  "",   "6",    "",    "", ""],     //-high -tense -mid +low OR -raised +retracted
//     ["{",  "",   "a",   "&",   "A", "Q"], //-high +tense -mid +low OR -raised +retracted
// ]; //front                      back

// pub static VOWEL_CHART: [[&str; 6]; 8] = [
//     ["j", "H",    "",    "", "M\\", "w"],
//     ["i", "y",   "1",   "}",   "M", "u"], //+high +tense -mid -low OR +raised -retracted
//     ["I", "Y",    "",    "",    "", "U"],    //                       OR +raised +retracted
//     ["e", "2", "@\\",   "8",   "7", "o"], //+high +tense +mid -low OR -raised -retracted
//     [ "",  "",   "@",    "",    "", ""],      //-high -tense +mid -low OR -raised
//     ["E", "9",   "3", "3\\",   "V", "O"], //-high -tense +mid +low OR -raised +retracted
//     ["{",  "",   "6",    "",    "", ""],     //-high -tense -mid +low OR -raised +retracted
//     ["a", "&","a_\"",    "",   "A", "Q"], //-high +tense -mid +low OR -raised +retracted
// ]; //front                      back

//const MASTER_LIST: &[&str] = concat!(
//    CONSONENT_CHART.concat().as_slice(),
//    NON_PULMONIC_CONSONANT_CHART.concat().as_slice(),
//    VOWEL_CHART.concat().as_slice(),
//);

//static ALT_VOWEL_CHART: = [
//    [],
//]
//static SEGMENT_MODIFIERS: =[];
