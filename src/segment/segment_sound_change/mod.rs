use std::{fmt::Display, ops::Deref};

// use itertools::Itertools;
use sound_change_utilities::parse_scstring;

use crate::{
    // segment::feature::feature_table::parse_featurebundle_list,
    util::utilities::{print_arr, split_sc},
    xsampa,
};

use super::{
    feature::{
        // feature_table::FeatureTable,
        FeatureBundle,
        FeatureIndex,
    },
    segment_string::SegmentStringShell,
    SegmentShell, TokenString,
};

pub mod sound_change_segment_string;
pub mod sound_change_utilities;
//pub mod sound_change_main;

pub trait PhonObject<F: FeatureBundle> {
    fn apply(&self, sc: &SoundChangeShell<F>) -> Self;
    fn apply_mut(&mut self, sc: &SoundChangeShell<F>);
}

pub type FetLis<F> = Vec<(F, bool)>;
pub fn print_fetlis<F: FeatureIndex>(fs: &FetLis<F>) -> String {
    let mut result = String::new();
    result.push('[');
    for &(f, b) in fs {
        if result.len() != 1 {
            result.push(',');
        }
        result.push(if b { '+' } else { '-' });
        result.push_str(&f.to_short_string());
    }
    result.push(']');
    result
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SCSegAn<F: FeatureBundle> {
    SCSeg(SCSegment<F>),
    Skip(Vec<SCSegment<F>>),
    // Not(Vec<SCSegment<F>>),
    SCMultiSeg(Vec<SCSegment<F>>), //Replace with SCString for more generality
}

impl<F: FeatureBundle> Display for SCSegAn<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SCSegAn::SCSeg(scseg) => write!(f, "{scseg}"),
            SCSegAn::Skip(skip_vec) => write!(f, "{}_s", print_arr(skip_vec, ("(", ")"))),
            SCSegAn::SCMultiSeg(scseg_vec) => write!(f, "{}", print_arr(scseg_vec, ("(", ")"))),
        }
    }
}

impl<F: FeatureBundle> SCSegAn<F> {
    pub fn to_segment(&self) -> Option<SegmentShell<F>> {
        match self {
            SCSegAn::SCSeg(scsegment) => scsegment.to_segment(),
            SCSegAn::Skip(vec) => None,
            SCSegAn::SCMultiSeg(vec) => None,
        }
    }

    pub fn to_scsegment(&self) -> Option<SCSegment<F>> {
        match self {
            SCSegAn::SCSeg(scsegment) => Some(scsegment.clone()),
            SCSegAn::Skip(_skipseg_vec) => None,
            SCSegAn::SCMultiSeg(scseg_vec) => {
                if scseg_vec.len() == 1 {
                    Some(scseg_vec[0].clone())
                } else {
                    None
                }
            }
        }
    }

    // pub fn to_string(&self) -> String {
    //     match self {
    //         SCSegAn::SCSeg(scsegment) => scsegment.to_string(),
    //         SCSegAn::Skip(skip_vec) => format!("{skip_vec:?}"),
    //         SCSegAn::SCMultiSeg(scseg_vec) => format!("{scseg_vec:?}"),
    //     }
    // }

    pub fn is_skip(&self) -> bool {
        match self {
            SCSegAn::Skip(_) => true,
            _ => false,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SCSegment<F: FeatureBundle> {
    TokenSeg(TokenString),

    FeatureSeg(FetLis<F::Index>),

    WordEdge,
}
impl<F: FeatureBundle> Display for SCSegment<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SCSegment::TokenSeg(t) => write!(f, "{}", xsampa::xsampa(&t.concat())),
            // SCSegment::SkipSeg(ts) => {
            //     let mut tmp = String::new();
            //     for t in ts {
            //         tmp.push_str(&xsampa::xsampa(&t.to_string()));
            //     }
            //     "(".to_string()+&tmp+")"
            // },
            SCSegment::FeatureSeg(fb) => write!(f, "{}", print_fetlis(fb)),

            SCSegment::WordEdge => write!(f, "#",),
        }
    }
}

impl<F: FeatureBundle> SCSegment<F> {
    pub fn to_segment(&self) -> Option<SegmentShell<F>> {
        match self {
            SCSegment::TokenSeg(tok) => Some(SegmentShell::from_string(&tok.concat())),

            SCSegment::FeatureSeg(_) => None,

            SCSegment::WordEdge => None,
        }
    }

    // pub fn to_string(&self) -> String {
    //     match self {
    //         SCSegment::TokenSeg(t) => xsampa::xsampa(&t.concat()),
    //         // SCSegment::SkipSeg(ts) => {
    //         //     let mut tmp = String::new();
    //         //     for t in ts {
    //         //         tmp.push_str(&xsampa::xsampa(&t.to_string()));
    //         //     }
    //         //     "(".to_string()+&tmp+")"
    //         // },
    //
    //         SCSegment::FeatureSeg(fb) => print_fetlis(fb),
    //
    //         SCSegment::WordEdge => "#".to_string(),
    //     }
    // }

    // pub fn is_skip(&self) -> bool {
    //     match self {
    //         SCSegment::SkipSeg(_) => true,
    //         _ => false,
    //     }
    // }
}
#[derive(Debug)]
pub struct SCApl<F: FeatureBundle> {
    pub segments: Vec<SCSegment<F>>,
}

impl<F: FeatureBundle> Deref for SCApl<F> {
    type Target = [SCSegment<F>];

    fn deref(&self) -> &Self::Target {
        &self.segments
    }
}

impl<F: FeatureBundle> SCApl<F> {
    pub fn to_segments(&self) -> Option<SegmentStringShell<F>> {
        let iter = self.segments.iter().map(|scstr| scstr.to_segment());

        let mut result = SegmentStringShell::new();

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

#[derive(Debug)]
pub struct SCString<F: FeatureBundle> {
    pub segments: Vec<SCSegAn<F>>,
    block_context: bool,
}

impl<F: FeatureBundle> Deref for SCString<F> {
    type Target = [SCSegAn<F>];

    fn deref(&self) -> &Self::Target {
        &self.segments
    }
}

impl<F: FeatureBundle> SCString<F> {
    pub fn to_scapl(&self) -> Option<SCApl<F>> {
        let mut segments = Vec::new();

        for seg in &self.segments {
            match seg.to_scsegment() {
                Some(seg) => segments.push(seg),
                None => return None,
            }
        }

        Some(SCApl { segments })
    }

    pub fn to_segments(&self) -> Option<SegmentStringShell<F>> {
        let iter = self.segments.iter().map(|scstr| scstr.to_segment());

        let mut result = SegmentStringShell::new();

        for seg_option in iter {
            match seg_option {
                Some(seg) => result.push(seg),
                None => return None,
            }
        }

        Some(result)
    }

    pub fn to_string(&self) -> String {
        let mut str_result = if self.block_context {
            "*".to_string()
        } else {
            String::new()
        };

        for scs in &self.segments {
            str_result.push_str(&scs.to_string());
        }

        str_result
    }
}

/**
Transforms a subsequence of tokens, of the form [from] to [to]
when the subsequence [before] occurs before the subsequence, and [after], after.
Eg: let sound_change_example = SoundChange::new("k->p/[+Vowel]__t");
"Attested in romanian, Lat:octō -> Rom:opt", with additional terminal vowel deletion.
Note that this struct uses SegmentString instead of Word, the Word struct contains a massive
amount of information that SoundChange and its associated functions will never use.
*/
#[derive(Debug)]
pub struct SoundChangeShell<F: FeatureBundle> {
    //This is the segment, or sequence of segments, that will be modified
    from: SCString<F>,
    /*
    Describes how self.from will be modified
    If segments are to be added or removed then all segment modifiying tokens
    must be of the string type
    */
    to: SCApl<F>,
    //The necessary context preceeding the modified segment
    before: SCString<F>,
    //The necessary context followng the modified segment
    after: SCString<F>,
}

impl<F: FeatureBundle> SoundChangeShell<F> {
    pub fn new(str: &str) -> Option<Self> {
        let (str_from, str_to, str_before, str_after) = split_sc(str)?;

        let from = parse_scstring(&str_from)?;
        // println!("{:?}", from);
        let to = parse_scstring(&str_to)?.to_scapl()?;
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
            // println!("{str}");
            match SoundChangeShell::new(str) {
                Some(sc) => result.push(sc),
                None => return None,
            }
        }

        Some(result)
    }

    pub fn to_string(&self) -> String {
        self.from.to_string()
            + "->"
            + &self.to.to_string()
            + "/"
            + &self.before.to_string()
            + "__"
            + &self.after.to_string()
    }
    pub fn to_short_string(&self) -> String {
        self.from.to_string()
            + "->"
            + &self.to.to_string()
            + "/"
            + &self.before.to_string()
            + "__"
            + &self.after.to_string()
    }
}
