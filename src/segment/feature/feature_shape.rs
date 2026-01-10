use std::fmt::Display;

use crate::{segment::segment_modifier::SegModifier, util, xsampa};

use super::{feature_bundle_to_string, FeatureBundle, FeatureIndex};

// #[derive(Copy, Clone, Debug, Eq, PartialEq)]
//Consider the lack of this to signify sonorants?
// struct Obstruant {
//     occlusive: bool,
//     fricative: bool,
// }
// impl Obstruant { //None signifies a sonorant
//     const fn new() -> Self {
//         Self { //If both are true, affricate! If neither, ?????? New science.
//             occlusive: false,
//             fricative: false,
//         }
//     }
// }

// #[derive(Copy, Clone, Debug, Eq, PartialEq)]
// enum AltLabial {
//     None,
//     Secondary,
//     Primary {
//         protruded: bool,
//     },
// }

use RootFeatures::*;
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum RootFeatures {
    Consonental,
    Continuant,
    Strident,
    // obstruant,
    Sonorant,
    Syllabic,
    Long,

    Laryngeal(LaryngealFeature),

    SupraLaryngeal(Option<SupraLaryngealFeatures>),
}
const BREAK_AT: &[RootFeatures] = &[
    Laryngeal(Voice),
    SupraLaryngeal(Some(Nasal)),
    SupraLaryngeal(Some(Aperture(High))),
    SupraLaryngeal(Some(Place(Labial(None)))),
    SupraLaryngeal(Some(Place(Lingual(Lateral)))),
    SupraLaryngeal(Some(Place(Lingual(Dorsal)))),
];

impl Display for RootFeatures {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Consonental => "Consonental",
            Sonorant => "Sonorant",
            Strident => "Strident",
            Continuant => "Continuant",
            Syllabic => "Syllabic",
            Long => "Long",
            // Lateral => "Lateral",
            Laryngeal(laryngeal_feature) => laryngeal_feature.to_string(),

            SupraLaryngeal(supra_laryngeal_feature) => supra_laryngeal_feature
                .map(|slf| slf.to_string())
                .unwrap_or("SupraLaryngeal"),
        };

        write!(f, "{str}")
    }
}

impl FeatureIndex for RootFeatures {
    fn list_features() -> String {
        let mut result = String::new();
        for feature in Self::index() {
            if BREAK_AT.contains(&feature) {
                result.push('\n');
            }
            result.push_str(&feature.to_string());
            result.push('\n');
        }
        result
    }

    fn to_short_string(&self) -> &'static str {
        match self {
            Consonental => "cons",
            Sonorant => "son",
            Strident => "str",
            Continuant => "cont",
            Syllabic => "syl",
            Long => "len",
            // Lateral => "Lateral",
            Laryngeal(laryngeal_feature) => laryngeal_feature.to_short_string(),

            SupraLaryngeal(supra_laryngeal_feature) => supra_laryngeal_feature
                .map(|slf| slf.to_short_string())
                .unwrap_or("SLar"),
        }
    }

    fn from_string(str: &str) -> Option<Self> {
        match str {
            "consonental" | "cons" => Some(Consonental),
            "continuant" | "cont" => Some(Continuant),
            "strident" | "str" => Some(Strident),
            "sonorant" | "son" => Some(Sonorant),
            "syllabic" | "syl" => Some(Syllabic),
            "length" | "len" | "long" => Some(Long),

            "nasal" | "nas" => Some(SupraLaryngeal(Some(Nasal))),

            "voice" | "voi" => Some(Laryngeal(Voice)),
            "spreadGlottis" | "sprgl" => Some(Laryngeal(Spread)),
            "constrictedGlottis" | "congl" => Some(Laryngeal(Constricted)),

            "high" => Some(SupraLaryngeal(Some(Aperture(High)))),
            "low" => Some(SupraLaryngeal(Some(Aperture(Low)))),
            "advancedTongueRoot" | "atr" => Some(SupraLaryngeal(Some(Aperture(ATR)))),

            "labial" | "lab" => Some(SupraLaryngeal(Some(Place(Labial(None))))),
            "round" | "rnd" => Some(SupraLaryngeal(Some(Place(Labial(Some(Round)))))),

            "lateral" | "lat" => Some(SupraLaryngeal(Some(Place(Lingual(Lateral))))),

            "coronal" | "cor" | "front" => {
                Some(SupraLaryngeal(Some(Place(Lingual(Coronal(None))))))
            }
            "anterior" | "ant" => Some(SupraLaryngeal(Some(Place(Lingual(Coronal(Some(Anterior))))))),
            "distributed" | "dist" => Some(SupraLaryngeal(Some(Place(Lingual(Coronal(Some(Distributed))))))),

            "dorsal" | "dors" | "back" => Some(SupraLaryngeal(Some(Place(Lingual(Dorsal))))),

            // "delayedRelease" | "delrel" => Some(Feature::DelayedRelease),
            _ => None,
        }
    }

    fn parse(str: &str) -> Option<Vec<(Self, bool)>> {
        let mut result = Vec::new();
        let str_arr = util::utilities::unbracket_arr(str, ("[", "]"))?;

        for str in str_arr {
            let (pre, rest) = str.split_at(1);
            match pre {
                "+" => {
                    // println!("attempting to parse {pre}{rest}");
                    let f = Self::from_string(rest)?;
                    // println!("Parsed {pre}{rest} and received ({f:?}, true)");
                    result.push((f, true));
                }
                "-" => {
                    // println!("attempting to parse {pre}{rest}");
                    let f = Self::from_string(rest)?;
                    // println!("Parsed {pre}{rest} and received ({f:?}, false)");
                    result.push((f, false));
                }
                _ => {
                    eprintln!("ERROR@<RootFeatures as FeatureIndex>::parse(): No valid boolian value can be parsed from {pre}{rest}");
                }
            }
        }

        Some(result)
    }

    fn from_feature(f: super::Feature) -> Self {
        match f {
            super::Feature::Consonental => Consonental,
            super::Feature::Continuant => Continuant,
            super::Feature::Strident => Strident,
            super::Feature::Lateral => SupraLaryngeal(Some(Place(Lingual(Lateral)))),
            super::Feature::Sonorant => Sonorant,
            super::Feature::Syllabic => Syllabic,
            super::Feature::Length => Long,

            super::Feature::Voice => Laryngeal(Voice),
            super::Feature::SpreadGlottis => Laryngeal(Spread),
            super::Feature::ConstrictedGlottis => Laryngeal(Constricted),

            super::Feature::Nasal => SupraLaryngeal(Some(Nasal)),

            super::Feature::Labial => SupraLaryngeal(Some(Place(Labial(None)))),
            super::Feature::Round => SupraLaryngeal(Some(Place(Labial(Some(Round))))),

            super::Feature::Coronal => SupraLaryngeal(Some(Place(Lingual(Coronal(None))))),
            super::Feature::Anterior => {
                SupraLaryngeal(Some(Place(Lingual(Coronal(Some(Anterior))))))
            }
            super::Feature::Distributed => {
                SupraLaryngeal(Some(Place(Lingual(Coronal(Some(Distributed))))))
            }

            super::Feature::Dorsal => SupraLaryngeal(Some(Place(Lingual(Dorsal)))),

            super::Feature::High => SupraLaryngeal(Some(Aperture(High))),
            super::Feature::Low => SupraLaryngeal(Some(Aperture(Low))),
            super::Feature::Front => SupraLaryngeal(Some(Place(Lingual(Coronal(None))))),
            super::Feature::Back => SupraLaryngeal(Some(Place(Lingual(Dorsal)))),

            super::Feature::Pharyngeal => Consonental,
            super::Feature::AdvancedTongueRoot => SupraLaryngeal(Some(Aperture(ATR))), //

            super::Feature::DelayedRelease => Consonental,
        }
    }

    fn index<'a>() -> &'a [Self] {
        &[
            //5
            Consonental,
            Continuant,
            Strident,
            // Lateral,
            Sonorant,
            Syllabic,
            Long,
            //3
            Laryngeal(Voice),
            Laryngeal(Spread),
            Laryngeal(Constricted),
            //1
            SupraLaryngeal(Some(Nasal)),
            //3
            SupraLaryngeal(Some(Aperture(High))),
            SupraLaryngeal(Some(Aperture(Low))),
            SupraLaryngeal(Some(Aperture(ATR))),
            //2
            SupraLaryngeal(Some(Place(Labial(None)))),
            SupraLaryngeal(Some(Place(Labial(Some(Round))))),
            //1
            SupraLaryngeal(Some(Place(Lingual(Lateral)))),
            //3
            SupraLaryngeal(Some(Place(Lingual(Coronal(None))))),
            SupraLaryngeal(Some(Place(Lingual(Coronal(Some(Anterior)))))),
            SupraLaryngeal(Some(Place(Lingual(Coronal(Some(Distributed)))))),
            //1
            SupraLaryngeal(Some(Place(Lingual(Dorsal)))),
            //18 total features
        ]
    }
}

impl RootFeatures {
    pub fn list() -> &'static [RootFeatures] {
        &[
            //5
            Consonental,
            Continuant,
            Strident,
            // Lateral,
            Sonorant,
            Syllabic,
            //3
            Laryngeal(Voice),
            Laryngeal(Spread),
            Laryngeal(Constricted),
            //1
            SupraLaryngeal(Some(Nasal)),
            //3
            SupraLaryngeal(Some(Aperture(High))),
            SupraLaryngeal(Some(Aperture(Low))),
            SupraLaryngeal(Some(Aperture(ATR))),
            //2
            SupraLaryngeal(Some(Place(Labial(None)))),
            SupraLaryngeal(Some(Place(Labial(Some(Round))))),
            //1
            SupraLaryngeal(Some(Place(Lingual(Lateral)))),
            //3
            SupraLaryngeal(Some(Place(Lingual(Coronal(None))))),
            SupraLaryngeal(Some(Place(Lingual(Coronal(Some(Anterior)))))),
            SupraLaryngeal(Some(Place(Lingual(Coronal(Some(Distributed)))))),
            //1
            SupraLaryngeal(Some(Place(Lingual(Dorsal)))),
            //18 total features
        ]
    }
}

pub enum BasicFeature {
    Consonental,
    Continuant,
    Strident,
    Lateral,
    Sonorant,

    //3 Laryngeal
    Voice,
    Spread,
    Constricted,

    //1 SupraLaryngeal
    Nasal,

    //3 Aperture
    High,
    Low,
    RTR,

    //2
    Labial,
    Round,

    //3
    Coronal, //Also Front
    Anterior,
    Distributed,

    //1
    Dorsal, //Also Back

            //18 total features
}
use LabialFeatures::*;
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum LabialFeatures {
    Round,
}
impl LabialFeatures {
    pub fn to_string(&self) -> &'static str {
        match self {
            Round => "Round",
        }
    }
}
use CoronalFeature::*;
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum CoronalFeature {
    Anterior,
    Distributed,
}
impl CoronalFeature {
    pub fn to_string(&self) -> &'static str {
        match self {
            Anterior => "Anterior",
            Distributed => "Distributed",
        }
    }
    pub fn to_short_string(&self) -> &'static str {
        match self {
            Anterior => "ant",
            Distributed => "dist",
        }
    }
}
use SupraLaryngealFeatures::*;
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum SupraLaryngealFeatures {
    Nasal,
    Aperture(ApertureFeature),
    Place(PlaceFeature),
}
impl SupraLaryngealFeatures {
    pub fn to_string(&self) -> &'static str {
        match self {
            Nasal => "Nasal",
            Aperture(aperture_feature) => aperture_feature.to_string(),
            Place(place_feature) => place_feature.to_string(),
        }
    }
    pub fn to_short_string(&self) -> &'static str {
        match self {
            Nasal => "nas",
            Aperture(aperture_feature) => aperture_feature.to_short_string(),
            Place(place_feature) => place_feature.to_short_string(),
        }
    }
}

use ApertureFeature::*;
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum ApertureFeature {
    High,
    Low,
    ATR,
}
impl ApertureFeature {
    pub fn to_string(&self) -> &'static str {
        match self {
            High => "High",
            Low => "Low",
            ATR => "ATR",
        }
    }
    pub fn to_short_string(&self) -> &'static str {
        match self {
            High => "hi",
            Low => "lo",
            ATR => "atr",
        }
    }
}
use LaryngealFeature::*;
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum LaryngealFeature {
    Voice,
    Spread,
    Constricted,
}
impl LaryngealFeature {
    pub fn to_string(&self) -> &'static str {
        match self {
            Voice => "Voice",
            Spread => "Spread",
            Constricted => "Constricted",
        }
    }
    pub fn to_short_string(&self) -> &'static str {
        match self {
            Voice => "voi",
            Spread => "spr",
            Constricted => "cnstr",
        }
    }
}
// use LingualFeatures::*;
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum LingualFeature {
    Lateral,
    Coronal(Option<CoronalFeature>),
    Dorsal,
}
use LingualFeature::*;
impl LingualFeature {
    pub fn to_string(&self) -> &'static str {
        match self {
            LingualFeature::Lateral => "Lateral",
            LingualFeature::Coronal(coronal_feature_opt) => match coronal_feature_opt {
                Some(coronal_feature) => coronal_feature.to_string(),
                None => "Coronal",
            },
            LingualFeature::Dorsal => "Dorsal",
        }
    }
    pub fn to_short_string(&self) -> &'static str {
        match self {
            LingualFeature::Lateral => "lat",
            LingualFeature::Coronal(coronal_feature_opt) => match coronal_feature_opt {
                Some(coronal_feature) => coronal_feature.to_short_string(),
                None => "cor",
            },
            LingualFeature::Dorsal => "dors",
        }
    }
}

use PlaceFeature::*;
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum PlaceFeature {
    Labial(Option<LabialFeatures>),
    Lingual(LingualFeature),
}
impl PlaceFeature {
    pub fn to_string(&self) -> &'static str {
        match self {
            PlaceFeature::Labial(labial_feature) => match labial_feature {
                Some(Round) => "Round",
                None => "Labial",
            },
            PlaceFeature::Lingual(ling_f) => ling_f.to_string(),
        }
    }
    pub fn to_short_string(&self) -> &'static str {
        match self {
            PlaceFeature::Labial(labial_feature) => match labial_feature {
                Some(Round) => "rnd",
                None => "lab",
            },
            PlaceFeature::Lingual(ling_f) => ling_f.to_short_string(),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Labial {
    round: bool,
}
impl Labial {
    const fn new() -> Self {
        Self { round: false }
    }

    fn get(&self, _f: LabialFeatures) -> bool {
        self.round
    }

    fn set(&mut self, _f: LabialFeatures, b: bool) {
        self.round = b;
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Lingual {
    lateral: bool,
    coronal: Option<Coronal>,
    dorsal: bool,
}
impl Lingual {
    const fn new() -> Self {
        Self {
            lateral: false,
            coronal: None,
            dorsal: false,
        }
    }
    fn get(&self, f: LingualFeature) -> Option<bool> {
        match f {
            LingualFeature::Lateral => Some(self.lateral),
            LingualFeature::Coronal(coronal_feature) => match coronal_feature {
                Some(cor_f) => self.coronal.map(|cor| cor.get(cor_f)),
                None => Some(self.coronal.is_some()),
            },
            LingualFeature::Dorsal => Some(self.dorsal),
        }
    }

    fn set(&mut self, f: LingualFeature, b: bool) {
        match f {
            LingualFeature::Lateral => self.lateral = b,
            LingualFeature::Coronal(cor_f_opt) => match cor_f_opt {
                Some(cor_f) => match &mut self.coronal {
                    Some(c) => c.set(cor_f, b),
                    None => {
                        if b {
                            let mut tmp = Coronal::new();
                            tmp.set(cor_f, b);
                            self.coronal = Some(tmp);
                        }
                    }
                },
                None => {
                    if b {
                        match &mut self.coronal {
                            Some(c) => {}
                            None => {
                                if b {
                                    self.coronal = Some(Coronal::new());
                                }
                            }
                        }
                    } else {
                        self.coronal = None;
                    }
                }
            },
            LingualFeature::Dorsal => self.dorsal = b,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Coronal {
    anterior: bool,
    distributed: bool,
}
impl Coronal {
    const fn new() -> Self {
        Self {
            anterior: false,
            distributed: false,
        }
    }
    fn get(&self, f: CoronalFeature) -> bool {
        match f {
            Anterior => self.anterior,
            Distributed => self.distributed,
        }
    }

    fn set(&mut self, f: CoronalFeature, b: bool) {
        match f {
            Anterior => self.anterior = b,
            Distributed => self.distributed = b,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Dorsal {
    high: bool,
    low: bool,
    front: bool,
    back: bool,
}
impl Dorsal {
    const fn new() -> Self {
        Self {
            high: false,
            low: false,
            front: false,
            back: false,
        }
    }
}

// #[derive(Copy, Clone, Debug, Eq, PartialEq)]
// pub struct RootNode {
//     consonental: bool,
//     continuant: bool,
//     strident: bool,
//     // lateral: bool,
//     sonorant: bool,
//     syllabic: bool,
//     laryngeal: Laryngeal,
//     supralaryngeal: Option<SupraLaryngealNodes>,
// }
// impl RootNode {
//     fn get(&self, f: RootFeatures) -> Option<bool> {
//         match f {
//             Consonental => Some(self.consonental),
//             Continuant => Some(self.continuant),
//             Strident => Some(self.strident),
//             // Lateral => Some(self.lateral),
//             Sonorant => Some(self.sonorant),
//             Syllabic => Some(self.syllabic),
//
//             Laryngeal(lf) => Some(self.laryngeal.get(lf)),
//
//             SupraLaryngeal(slf_opt) => {
//                 match self.supralaryngeal {
//                     Some(supra) => {
//                         match slf_opt {
//                             Some(slf) => {
//                                 supra.get(slf)
//                             },
//                             None => Some(self.supralaryngeal.is_some()),
//                         }
//                     },
//                     None => None,
//                 }
//             },
//         }
//     }
//
//     fn set(&mut self, f: RootFeatures, b: bool) {
//         match f {
//             Consonental => self.consonental = b,
//             Continuant => self.continuant = b,
//             Strident => self.strident = b,
//             // Lateral => self.lateral = b,
//             Sonorant => self.sonorant = b,
//             Syllabic => self.syllabic = b,
//
//             Laryngeal(lf) => {
//                 self.laryngeal.set(lf,b);
//             },
//
//             SupraLaryngeal(slf_opt) => {
//                 match self.supralaryngeal {
//                     Some(mut supra) => {
//                         match slf_opt {
//                             Some(slf) => {
//                                 supra.set(slf, b);
//                             },
//                             None => {
//                                 if b == false {
//                                     self.supralaryngeal = None;
//                                 }
//                             },
//                         }
//                     },
//                     None => {
//                         if b {
//                             self.supralaryngeal = Some(SupraLaryngealNodes::new());
//                         }
//                     },
//                 }
//             },
//         }
//     }
// }
//

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct SupraLaryngealNodes {
    nasal: bool,
    aperture: Aperture, //Make optional?
    place: Place,//Move labial features out?
}
impl SupraLaryngealNodes {
    fn new() -> Self {
        Self {
            nasal: false,
            aperture: Aperture::new(),
            place: Place::new(),
        }
    }

    fn get(&self, f: SupraLaryngealFeatures) -> Option<bool> {
        match f {
            Nasal => Some(self.nasal),
            Aperture(af) => Some(self.aperture.get(af)),
            Place(pf) => self.place.get(pf),
        }
    }

    fn set(&mut self, f: SupraLaryngealFeatures, b: bool) {
        match f {
            Nasal => {
                self.nasal = b;
                // println!("SupraLaryngeal_Set_Nasal as {b}");
            }
            Aperture(af) => {
                self.aperture.set(af, b);
                // println!("SupraLaryngeal_Set_Aperature as {af:?}");
            }
            Place(pf) => {
                self.place.set(pf, b);
            }
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Aperture {
    high: bool,
    low: bool,
    atr: bool,
}
impl Aperture {
    fn new() -> Self {
        Self {
            high: false,
            low: false,
            atr: false,
        }
    }

    fn get(&self, f: ApertureFeature) -> bool {
        match f {
            High => self.high,
            Low => self.low,
            ATR => self.atr,
        }
    }

    fn set(&mut self, f: ApertureFeature, b: bool) {
        // println!("Set {f:?} to {b}");
        match f {
            High => self.high = b,
            Low => self.low = b,
            ATR => self.atr = b,
        }
        // println!("{f:?} was set to {}", self.get(f));
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Laryngeal {
    voice: bool,
    spread: bool,
    constricted: bool,
}
impl Laryngeal {
    fn new() -> Self {
        Laryngeal {
            voice: false,
            spread: false,
            constricted: false,
        }
    }

    fn get(&self, f: LaryngealFeature) -> bool {
        match f {
            Voice => self.voice,
            Spread => self.spread,
            Constricted => self.constricted,
        }
    }

    fn set(&mut self, f: LaryngealFeature, b: bool) {
        match f {
            Voice => self.voice = b,
            Spread => self.spread = b,
            Constricted => self.constricted = b,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Place {
    labial: Option<Labial>,
    lingual: Option<Lingual>,
}
impl Place {
    fn new() -> Self {
        Place {
            labial: None,
            lingual: None,
        }
    }

    fn get(&self, f: PlaceFeature) -> Option<bool> {
        match f {
            Labial(lab_f_opt) => {
                match lab_f_opt {
                    Some(lab_f) => self.labial.map(|l| l.get(lab_f)),
                    None => {
                        // println!("Labial is {}", self.labial.is_some());
                        Some(self.labial.is_some())
                    }
                }
            }
            Lingual(ling_f) => self.lingual.map(|l| l.get(ling_f))?,
        }
    }

    fn set(&mut self, f: PlaceFeature, b: bool) {
        match f {
            Labial(lf_opt) => {
                // println!("SupraLaryngeal_Set_Labial");
                match lf_opt {
                    Some(lf) => match &mut self.labial {
                        Some(l) => {
                            l.set(lf, b);
                        }
                        None => {
                            if b {
                                let mut tmp = Labial::new();
                                tmp.set(lf, b);
                                self.labial = Some(tmp);
                            } else {
                                self.labial = None;
                            }
                        }
                    },
                    None => {
                        // println!("SupraLaryngeal_Set_Labial as {b}");
                        if b {
                            match &mut self.labial {
                                Some(_) => {}
                                None => {
                                    // println!("SupraLaryngeal_Set_Labial as {b}");
                                    self.labial = Some(Labial::new());
                                    // println!("{:?}", self.get((Labial(None))));
                                }
                            }
                        } else {
                            self.labial = None;
                        }
                    }
                }
            }
            Lingual(ling_f) => {
                // println!("SupraLaryngeal_Set_Lingual");
                match &mut self.lingual {
                    Some(l) => l.set(ling_f, b),
                    None => {
                        let mut tmp = Lingual::new();
                        tmp.set(ling_f, b);
                        self.lingual = Some(tmp);
                    }
                }
            }
        }
    }
}

pub static TEST_FET_GEOM_BUND: FeatureGeometryBundle = FeatureGeometryBundle {
    //L\
    consonental: true,
    sonorant: true,
    continuant: true,
    strident: false,
    syllabic: false,
    long: false,

    laryngeal: Laryngeal {
        voice: true,
        spread: false,
        constricted: false,
    },
    supra_laryngeal: Some(SupraLaryngealNodes {
        nasal: false,
        aperture: Aperture {
            high: true,
            low: false,
            atr: false,
        },
        place: Place {
            labial: None,
            lingual: Some(Lingual {
                lateral: true,
                coronal: None,
                dorsal: true,
            }),
        },
    }),
};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct FeatureGeometryBundle {
    consonental: bool,
    sonorant: bool,
    continuant: bool,
    strident: bool,
    // lateral: bool,
    syllabic: bool,
    long: bool,

    laryngeal: Laryngeal,

    supra_laryngeal: Option<SupraLaryngealNodes>,
}

impl FeatureBundle for FeatureGeometryBundle {
    type Index = RootFeatures;

    fn new() -> Self {
        FeatureGeometryBundle {
            consonental: false,
            sonorant: false,
            continuant: false,
            strident: false,
            syllabic: false,
            long: false,
            // lateral: false,
            laryngeal: Laryngeal::new(),
            supra_laryngeal: None,
        }
    }

    fn get(&self, f: RootFeatures) -> Option<bool> {
        match f {
            Consonental => Some(self.consonental),
            Continuant => Some(self.continuant),
            Strident => Some(self.strident),
            // Lateral => Some(self.lateral),
            Sonorant => Some(self.sonorant),
            Syllabic => Some(self.syllabic),
            Long => Some(self.long),

            Laryngeal(lf) => Some(self.laryngeal.get(lf)),

            SupraLaryngeal(slf_opt) => match self.supra_laryngeal {
                Some(supra) => match slf_opt {
                    Some(slf) => supra.get(slf),
                    None => Some(self.supra_laryngeal.is_some()),
                },
                None => None,
            },
        }
    }

    fn get_right(&self, feature: Self::Index) -> Option<bool> {
        self.get(feature)
    }

    fn get_left(&self, feature: Self::Index) -> Option<bool> {
        self.get(feature)
    }

    fn set(&mut self, f: RootFeatures, b: bool) {
        match f {
            Consonental => self.consonental = b,
            Continuant => self.continuant = b,
            Strident => self.strident = b,
            // Lateral => self.lateral = b,
            Sonorant => self.sonorant = b,
            Syllabic => self.syllabic = b,
            Long => self.long = b,

            Laryngeal(lf) => self.laryngeal.set(lf, b),

            SupraLaryngeal(slf_opt) => {
                match self.supra_laryngeal {
                    Some(mut supra) => {
                        match slf_opt {
                            Some(slf) => {
                                supra.set(slf, b);
                            }
                            None => {
                                if b == false {
                                    self.supra_laryngeal = None;
                                    // println!("SupraLaryngeal_Delete");
                                }
                            }
                        }
                        self.supra_laryngeal = Some(supra);
                    }
                    None => {
                        if b {
                            match slf_opt {
                                Some(slf) => {
                                    let mut sup_node = SupraLaryngealNodes::new();
                                    sup_node.set(slf, b);
                                    self.supra_laryngeal = Some(sup_node);
                                }
                                None => {
                                    self.supra_laryngeal = Some(SupraLaryngealNodes::new());
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn from_string(str: &str) -> Self {
        // println!("str == {str}");
        let str_tokens = xsampa::xsampa_split(str);

        // println!("str_tokens == {str_tokens:?}");

        let (c, m, p) = super::string_to_intermediate_repr(&str_tokens[0]).unwrap();
        let mut result = from_token_intermediate(c, m, p);

        for t in 1..str_tokens.len() {
            match SegModifier::<RootFeatures>::from_string(&str_tokens[t]) {
                Some(modif) => {
                    result.modify(&modif);
                }
                None => {}
            }
        }

        result
    }

    fn print(&self) -> String {
        feature_bundle_to_string(self)
    }

    fn to_col_str(&self) -> String {
        let mut result = String::new();

        for &f in Self::Index::index() {
            if BREAK_AT.contains(&f) {
                result.push('\n');
            }
            let b = self.get(f);

            match b {
                Some(true) => {
                    result.push('+');
                    // str.push_str(&fet.to_string());
                    result.push('\n');
                }
                Some(false) => {
                    result.push('-');
                    // str.push_str(&fet.to_string());
                    result.push('\n');
                }
                None => {
                    result.push('∅');
                    result.push('\n');
                }
            }
        }
        result
    }

    fn to_long_col_str(&self) -> String {
        let mut result = String::new();
        result.push('[');

        for &f in Self::Index::index() {
            match self.get(f) {
                Some(b) => {
                    if b {
                        result.push('+');
                        result.push_str(f.to_short_string());
                        result.push(',');
                    }
                }
                None => {}
            }
        }
        result.push(']');
        result
    }

    fn to_debug_string(&self) -> String {
        let mut output = String::new();

        for &f in Self::Index::index() {
            if BREAK_AT.contains(&f) {
                output.push('\n');
            }
            match self.get(f) {
                Some(true) => output.push_str("+\n"),
                Some(false) => output.push_str("-\n"),
                None => output.push_str("∅\n"),
            }
        }
        output
    }

    fn to_long_debug_string(&self) -> String {
        let mut str = String::new();

        for &f in Self::Index::index() {
            if BREAK_AT.contains(&f) {
                str.push('\n');
            }
            match self.get(f) {
                Some(true) => {
                    str.push_str("+");
                    str.push_str(&f.to_string());
                }
                Some(false) => {
                    str.push_str("-");
                    str.push_str(&f.to_string());
                }
                None => str.push_str("∅"),
            }
            str.push('\n')
        }

        str
    }

    fn modify(&mut self, modifier: &SegModifier<Self::Index>) {
        // //self.modify_feature(modifier.modif[0].0, modifier.modif[0].1);
        // for &(f,b) in modifier.modif {
        //         self.set(f, b);
        // }
        for &(f, b) in &modifier.modif {
            self.set(f, b);
        }
    }
}

impl FeatureGeometryBundle {
    fn print(&self) -> String {
        todo!()
    }
}

fn from_token_intermediate(chart: u8, manner: u8, place: u8) -> FeatureGeometryBundle {
    let mut temp: FeatureGeometryBundle = FeatureGeometryBundle::new();
    match chart {
        0 => {
            // temp.set(Consonental, true);
            //temp.set(Syllabic, false);
            //temp.set(SpreadGlottis, false);
            //temp.set(ConstrictedGlottis, false);
            temp.set(Laryngeal(Voice), place % 2 != 0);
            match manner {
                0 => {
                    //plosive
                    temp.set(Consonental, true);
                    // temp.set(Sonorant, false);
                    //temp.set(Nasal, false);
                    //temp.set(Strident, false);
                    //temp.set(Lateral, false);
                    //temp.set(DelayedRelease, false);
                }
                1 => {
                    //implosive
                    temp.set(Consonental, true);
                    temp.set(Sonorant, true);
                    //temp.set(Nasal, false);
                    //temp.set(Strident, false);
                    //temp.set(Lateral, false);
                    //temp.set(DelayedRelease, false);
                }
                //Implosive
                2 => {
                    //fricative
                    temp.set(Consonental, true);
                    //temp.set(Sonorant, false);
                    temp.set(Continuant, true);
                    //temp.set(Nasal, false);
                    //temp.set(Strident, false);
                    //temp.set(Lateral, false);
                    //temp.set(DelayedRelease, false);
                }
                3 => {
                    //Strident Fricative
                    temp.set(Consonental, true);
                    //temp.set(Sonorant, false);
                    temp.set(Continuant, true);
                    //temp.set(Nasal, false);
                    temp.set(Strident, true);
                    //temp.set(Lateral, false);
                    //temp.set(DelayedRelease, false);
                }
                4 => {
                    //Lateral fricatives
                    temp.set(Consonental, true);
                    temp.set(Continuant, true);
                    //temp.set(Sonorant, true);
                    //temp.set(Nasal, true);
                    //temp.set(Strident, false);
                    temp.set(SupraLaryngeal(Some(Place(Lingual(Lateral)))), true);
                    //temp.set(DelayedRelease, false);
                }
                5 => {
                    //nasal
                    temp.set(Consonental, true);
                    temp.set(Sonorant, true);
                    //temp.set(Continuant, false);
                    temp.set(SupraLaryngeal(Some(Nasal)), true);
                    //temp.set(Strident, false);
                    //temp.set(Lateral, false);
                    //temp.set(DelayedRelease, false);
                }
                6 => {
                    //Lateral
                    //temp.set(Approximant, true);
                    temp.set(Consonental, true);
                    temp.set(Sonorant, true);
                    temp.set(Continuant, true);
                    // temp.set(SupraLaryngeal(Some(Nasal)), false);
                    // temp.set(Strident, false);
                    temp.set(SupraLaryngeal(Some(Place(Lingual(Lateral)))), true);
                    // temp.set(DelayedRelease, false);
                }
                7 => {
                    //taps
                    temp.set(Consonental, true);
                    temp.set(Sonorant, true);
                    //temp.set(Continuant, true);
                    //temp.set(Nasal, false);
                    //temp.set(Strident, false);
                    //temp.set(Lateral, false);
                    //temp.set(DelayedRelease, false);
                }
                8 => {
                    //trills
                    temp.set(Consonental, true);
                    temp.set(Continuant, true);
                    temp.set(Sonorant, true);
                    //temp.set(Nasal, true);
                    //temp.set(Strident, false);
                    //temp.set(Lateral, false);
                    //temp.set(DelayedRelease, false);
                }
                9 => {
                    //approximates
                    temp.set(Sonorant, true);
                    temp.set(Continuant, true);
                    // temp.set(Nasal, true);
                    // temp.set(Strident, false);
                    // temp.set(Lateral, false);
                    // temp.set(DelayedRelease, false);
                }
                _ => {}
            }

            match place / 2 {
                0 => {
                    //biLabial
                    temp.set(SupraLaryngeal(Some(Place(Labial(None)))), true);
                    // temp.set(SupraLaryngeal(Place(Lingual(Coronal(None)))), false);
                    //temp.set(Anterior   , false);
                    //temp.set(Distributed, false);
                    // temp.set(SupraLaryngeal(Place(Lingual(Dorsal))), false);
                    // temp.set(Pharyngeal, false);
                }
                1 => {
                    //labiodental
                    temp.set(SupraLaryngeal(Some(Place(Labial(None)))), true);
                    // temp.set(SupraLaryngeal(Place(Lingual(Coronal(None)))), false);
                    //temp.set(Anterior   , false);
                    //temp.set(Distributed, false);
                    // temp.set(SupraLaryngeal(Place(Lingual(Dorsal))), false);
                    // temp.set(Pharyngeal, false);
                }
                2 => {
                    //dental
                    // temp.set(SupraLaryngeal(Place(Labial(None))), false);
                    temp.set(SupraLaryngeal(Some(Place(Lingual(Coronal(None))))), true);
                    temp.set(
                        SupraLaryngeal(Some(Place(Lingual(Coronal(Some(Anterior)))))),
                        true,
                    );
                    temp.set(
                        SupraLaryngeal(Some(Place(Lingual(Coronal(Some(Distributed)))))),
                        true,
                    );
                    // temp.set(SupraLaryngeal(Place(Lingual(Dorsal))), false);
                    // temp.set(Pharyngeal, false);
                }
                3 => {
                    //alveolar
                    // temp.set(SupraLaryngeal(Place(Labial(None))), false);
                    temp.set(SupraLaryngeal(Some(Place(Lingual(Coronal(None))))), true);
                    temp.set(
                        SupraLaryngeal(Some(Place(Lingual(Coronal(Some(Anterior)))))),
                        true,
                    );
                    // temp.set(SupraLaryngeal(Some(Place(Lingual(Coronal(Some(Distributed)))))), false);
                    // temp.set(SupraLaryngeal(Place(Lingual(Dorsal))), false);
                    // temp.set(Pharyngeal, false);
                }
                4 => {
                    //postalveolar
                    // temp.set(SupraLaryngeal(Place(Labial(None))), false);
                    temp.set(SupraLaryngeal(Some(Place(Lingual(Coronal(None))))), true);
                    temp.set(
                        SupraLaryngeal(Some(Place(Lingual(Coronal(Some(Anterior)))))),
                        true,
                    );
                    // temp.set(SupraLaryngeal(Some(Place(Lingual(Coronal(Some(Distributed)))))), true);
                    temp.set(SupraLaryngeal(Some(Aperture(High))), true);
                    // temp.set(SupraLaryngeal(Place(Lingual(Dorsal))), false);
                    // temp.set(Pharyngeal, false);
                }
                5 => {
                    //retroflex
                    // temp.set(SupraLaryngeal(Place(Labial(None))), false);
                    temp.set(SupraLaryngeal(Some(Place(Lingual(Coronal(None))))), true);
                    // temp.set(SupraLaryngeal(Place(Lingual(Coronal(Some(Anterior))))), false);
                    // temp.set(SupraLaryngeal(Place(Lingual(Coronal(Some(Distributed))))), false);
                    // temp.set(SupraLaryngeal(Place(Lingual(Dorsal))), false);
                    // temp.set(Pharyngeal, false);
                }
                6 => {
                    //palatal
                    // temp.set(SupraLaryngeal(Place(Labial(None))), false);
                    // temp.set(SupraLaryngeal(Place(Lingual(Coronal(None)))), false);
                    // temp.set(SupraLaryngeal(Some(Place(Lingual(Coronal(None))))), true);
                    //temp.set(SupraLaryngeal(Place(SupraLaryngeal(Place(Coronal(None)))(Some(Distributed))), false);
                    // temp.set(SupraLaryngeal(Some(Place(Lingual(Dorsal)))), true);
                    temp.set(SupraLaryngeal(Some(Aperture(High))), true);
                    // temp.set(SupraLaryngeal(Aperture(Low)), false);
                    // temp.set(SupraLaryngeal(Some(Place(Lingual(Coronal(None))))), true);
                    temp.set(
                        SupraLaryngeal(Some(Place(Lingual(Coronal(Some(Distributed)))))),
                        true,
                    );
                    // temp.set(SupraLaryngeal(Some(Place(Lingual(Dorsal)))), true);
                    // temp.set(Pharyngeal, false);
                }
                7 => {
                    //velar
                    // temp.set(SupraLaryngeal(Place(Labial(None))), false);
                    // temp.set(SupraLaryngeal(Place(Lingual(Coronal(None)))), false);
                    //temp.set(SupraLaryngeal(Place(SupraLaryngeal(Place(Coronal(None)))(Some(Anterior)))   , false);
                    //temp.set(SupraLaryngeal(Place(SupraLaryngeal(Place(Coronal(None)))(Some(Distributed))), false);
                    temp.set(SupraLaryngeal(Some(Place(Lingual(Dorsal)))), true);
                    temp.set(SupraLaryngeal(Some(Aperture(High))), true);
                    // temp.set(SupraLaryngeal(Aperture(Low)), false);
                    //temp.set(SupraLaryngeal(Place(Coronal(None)))      , false);
                    // temp.set(Pharyngeal, false);
                }
                8 => {
                    //uvular
                    // temp.set(SupraLaryngeal(Place(Labial(None))), false);
                    // temp.set(SupraLaryngeal(Place(Lingual(Coronal(None)))), false);
                    //temp.set(SupraLaryngeal(Place(SupraLaryngeal(Place(Coronal(None)))(Some(Anterior)))   , false);
                    //temp.set(SupraLaryngeal(Place(SupraLaryngeal(Place(Coronal(None)))(Some(Distributed))), false);
                    temp.set(SupraLaryngeal(Some(Place(Lingual(Dorsal)))), true);
                    // temp.set(SupraLaryngeal(Aperture(High)), false);
                    temp.set(SupraLaryngeal(Some(Aperture(Low))), true);
                    //temp.set(SupraLaryngeal(Place(Coronal(None)))      , false);
                    // temp.set(SupraLaryngeal(Place(Lingual(Dorsal))), true);
                    // temp.set(Pharyngeal, false);
                }
                9 => {
                    //Pharyngeal
                    // temp.set(SupraLaryngeal(Place(Labial(None))), false);
                    // temp.set(SupraLaryngeal(Place(Lingual(Coronal(None)))), false);
                    //temp.set(SupraLaryngeal(Place(SupraLaryngeal(Place(Coronal(None)))(Some(Anterior)))   , false);
                    //temp.set(SupraLaryngeal(Place(SupraLaryngeal(Place(Coronal(None)))(Some(Distributed))), false);
                    // temp.set(SupraLaryngeal(Place(Lingual(Dorsal))), false);
                    // temp.set(Pharyngeal, true);
                }
                10 => {
                    //glottal
                    // temp.set(SupraLaryngeal(Place(Labial(None))), false);
                    // temp.set(SupraLaryngeal(Place(Lingual(Coronal(None)))), false);
                    //temp.set(SupraLaryngeal(Place(SupraLaryngeal(Place(Coronal(None)))(Some(Anterior)))   , false);
                    //temp.set(SupraLaryngeal(Place(SupraLaryngeal(Place(Coronal(None)))(Some(Distributed))), false);
                    // temp.set(SupraLaryngeal(Place(Lingual(Dorsal))), false);
                    // temp.set(Pharyngeal, false);
                }
                _ => {}
            }
        }

        1 => {
            // temp.set(Syllabic, false);
            // temp.set(Consonental, true);
            // temp.set(Sonorant, false);
            match manner {
                0 => {
                    //Ejectives
                    //temp.set(Approximant, false);
                    // temp.set(Continuant, false);
                    // temp.set(SupraLaryngeal(Some(Nasal)), false);
                    // temp.set(Strident, false);
                    // temp.set(SupraLaryngeal(Some(Place(Lingual(Lateral)))), false);
                    // temp.set(DelayedRelease, false);
                    temp.set(Laryngeal(Voice), true);
                    temp.set(Laryngeal(Constricted), true);
                }
                1 => {
                    temp = FeatureGeometryBundle::new();
                    eprint!("ERROR: invalid or undefined token {temp:?}");
                    ////temp.set(Approximant, false);
                    //temp.set(Sonorant, false);
                    //temp.set(Continuant, false);
                    //temp.set(Nasal, false);
                    //temp.set(Strident =Some(false);
                    //temp.set(Lateral, false);
                    //temp.set(DelayedRelease, false);
                }
                _ => {}
            }

            match place {
                0 => {
                    temp.set(SupraLaryngeal(Some(Place(Labial(None)))), true);
                    // temp.set(SupraLaryngeal(Some(Place(Lingual(Coronal(None))))), false);
                    // temp.set(SupraLaryngeal(Some(Place(Lingual(Dorsal)))), false);
                    // temp.set(Pharyngeal, false);
                }
                1 => {
                    // temp.set(SupraLaryngeal(Some(Place(Labial(None)))), false);
                    temp.set(SupraLaryngeal(Some(Place(Lingual(Coronal(None))))), true);
                    temp.set(
                        SupraLaryngeal(Some(Place(Lingual(Coronal(Some(Anterior)))))),
                        true,
                    );
                    // temp.set(SupraLaryngeal(Some(Place(Lingual(Coronal(Some(Distributed)))))), false);
                    // temp.set(SupraLaryngeal(Some(Place(Lingual(Dorsal)))), false);
                    // temp.set(Pharyngeal, false);
                }
                2 => {
                    // temp.set(SupraLaryngeal(Some(Place(Labial(None)))), false);
                    // temp.set(SupraLaryngeal(Some(Place(Lingual(Coronal(None))))), false);
                    temp.set(SupraLaryngeal(Some(Place(Lingual(Dorsal)))), true);
                    temp.set(SupraLaryngeal(Some(Aperture(High))), true);
                    // temp.set(SupraLaryngeal(Some(Aperture(Low))), false);
                    // temp.set(SupraLaryngeal(Some(Place(Lingual(Coronal(None))))), false);
                    // temp.set(SupraLaryngeal(Some(Place(Lingual(Dorsal)))), false);
                    // temp.set(Pharyngeal, false);
                }
                3 => {
                    // temp.set(SupraLaryngeal(Some(Place(Labial(None)))), false);
                    // temp.set(SupraLaryngeal(Some(Place(Lingual(Coronal(None))))), false);
                    temp.set(SupraLaryngeal(Some(Place(Lingual(Dorsal)))), true);
                    temp.set(SupraLaryngeal(Some(Aperture(High))), true);
                    // temp.set(SupraLaryngeal(Some(Aperture(Low))), false);
                    // temp.set(SupraLaryngeal(Some(Place(Lingual(Coronal(None))))), false);
                    // temp.set(SupraLaryngeal(Some(Place(Lingual(Dorsal)))), true);
                    // temp.set(Pharyngeal, false);
                }
                4 => {
                    // temp.set(SupraLaryngeal(Some(Place(Labial(None)))), false);
                    // temp.set(SupraLaryngeal(Some(Place(Lingual(Coronal(None))))), false);
                    temp.set(SupraLaryngeal(Some(Place(Lingual(Dorsal)))), true);
                    // temp.set(SupraLaryngeal(Some(Aperture(High))), false);
                    temp.set(SupraLaryngeal(Some(Aperture(Low))), true);
                    // temp.set(SupraLaryngeal(Some(Place(Lingual(Coronal(None))))), false);
                    temp.set(SupraLaryngeal(Some(Place(Lingual(Dorsal)))), true);
                    // temp.set(Pharyngeal, false);
                }
                _ => {}
            }
        }
        2 => {
            // VOWELS
            // temp.set(Consonental, false);
            temp.set(Continuant, true);
            temp.set(Sonorant, true);
            temp.set(Syllabic, true);

            temp.set(SupraLaryngeal(Some(Place(Labial(None)))), true);
            // temp.set(SupraLaryngeal(Some(Place(Labial(Some(Round))))), false);

            // temp.set(SupraLaryngeal(Some(Place(Lingual(Coronal(None))))), false);

            // temp.set(SupraLaryngeal(Some(Place(Lingual(Dorsal)))), true);

            temp.set(Laryngeal(Voice), true);
            // temp.set(Laryngeal(Spread), false);
            // temp.set(SupraLaryngeal(Some(Nasal)), false);
            // temp.set(Strident, false);
            // temp.set(SupraLaryngeal(Some(Place(Lingual(Lateral)))), false);
            // temp.set(Laryngeal(Constricted), false);
            // temp.set(DelayedRelease, false);
            match manner {
                //Height
                0 => {
                    //Semivowel
                    temp.set(Syllabic, false);
                    temp.set(SupraLaryngeal(Some(Aperture(High))), true);
                    // temp.set(SupraLaryngeal(Aperture(Low)), false);
                    temp.set(SupraLaryngeal(Some(Aperture(ATR))), true);
                    //temp.set(Tense, false);
                }
                1 => {
                    //close/High Tense vowel
                    temp.set(SupraLaryngeal(Some(Aperture(High))), true);
                    // temp.set(SupraLaryngeal(Aperture(Low)), false);
                    temp.set(SupraLaryngeal(Some(Aperture(ATR))), true);
                    //temp.set(Tense, true);
                }
                2 => {
                    //near close/lax High vowel
                    temp.set(SupraLaryngeal(Some(Aperture(High))), true);
                    // temp.set(SupraLaryngeal(Aperture(Low)), false);
                    // temp.set(SupraLaryngeal(Some(Aperture(ATR))), true);
                    //temp.set(Tense, false);
                }
                3 => {
                    //mid close/mid Tense vowel
                    // temp.set(SupraLaryngeal(Some(Aperture(High))), false);
                    // temp.set(SupraLaryngeal(Some(Aperture(Low))), false);
                    temp.set(SupraLaryngeal(Some(Aperture(ATR))), true);
                    //temp.set(Tense, true);
                }
                4 => {
                    //mid vowel/central
                    // temp.set(SupraLaryngeal(Some(Aperture(High))), false);
                    // temp.set(SupraLaryngeal(Some(Aperture(Low))), false);
                    // temp.set(SupraLaryngeal(Some(Aperture(RTR))), false);
                    //temp.set(Tense, false);
                }
                5 => {
                    //mid open vowel/mid lax
                    // temp.set(SupraLaryngeal(Some(Aperture(High))), false);
                    // temp.set(SupraLaryngeal(Some(Aperture(Low))), false);
                    // temp.set(SupraLaryngeal(Some(Aperture(ATR))), true);
                    //temp.set(Tense, false);
                }
                6 => {
                    //near open vowel/SupraLaryngeal(Aperture(Low)) lax
                    // temp.set(SupraLaryngeal(Some(Aperture(High))), false);
                    temp.set(SupraLaryngeal(Some(Aperture(Low))), true);
                    // temp.set(SupraLaryngeal(Some(Aperture(RTR))), false);
                    //temp.set(Tense, false);
                }
                7 => {
                    //open vowwel/SupraLaryngeal(Aperture(Low)) Tense
                    // temp.set(SupraLaryngeal(Some(Aperture(High))), false);
                    temp.set(SupraLaryngeal(Some(Aperture(Low))), true);
                    temp.set(SupraLaryngeal(Some(Aperture(ATR))), true);
                    //temp.set(Tense, true);
                }
                _ => {}
            }

            temp.set(
                SupraLaryngeal(Some(Place(Labial(Some(Round))))),
                place % 2 == 1,
            );

            match place / 2 {
                0 => {
                    //Front
                    temp.set(
                        SupraLaryngeal(Some(Place(Lingual(Coronal(Some(Distributed)))))),
                        true,
                    );
                    // temp.set(SupraLaryngeal(Place(Dorsal)), false);
                }
                1 => { //Central
                     // temp.set(SupraLaryngeal(Some(Place(Lingual(Coronal(None))))), false);
                     // temp.set(SupraLaryngeal(Some(Place(Lingual(Dorsal)))), false);
                }
                2 => {
                    //Back
                    // temp.set(SupraLaryngeal(Place(Coronal(None))), false);
                    temp.set(SupraLaryngeal(Some(Place(Lingual(Dorsal)))), true);
                }
                _ => {}
            }
        }
        _ => {}
    }
    temp //.clone()
}
