use std::{
    fmt::{Debug, Display},
    ops::Deref,
};

use crate::{
    util::{self, split_on::Splittable, utilities::unbracket_arr},
    xsampa,
};

use super::{segment_modifier::SegModifier, segment_sound_change::SCSegment, SegModif};

pub mod feature_table;
// pub mod alt_feature_table;
pub mod feature_shape;

pub trait FeatureIndex: Sized + Copy + Clone + Debug + Eq + PartialEq + Display {
    fn list_features() -> String;

    // fn to_string(&self) -> &'static str;
    fn to_short_string(&self) -> &'static str;

    fn from_string(str: &str) -> Option<Self>;
    fn parse(str: &str) -> Option<Vec<(Self, bool)>>;

    fn from_feature(f: Feature) -> Self;
    fn index<'a>() -> &'a [Self];
}

pub trait FeatureBundle: Sized + Eq + PartialEq + Copy + Debug {
    type Index: FeatureIndex;

    fn new() -> Self;
    
    // Returns Some(true) or Some(false) if the bundle is specified for the input feature, None if unspecified
    fn get(&self, feature: Self::Index) -> Option<bool>;

    // get() the beginning state of a contour segment
    fn get_right(&self, feature: Self::Index) -> Option<bool>;

    // get() the end state of a contour segment
    fn get_left(&self, feature: Self::Index) -> Option<bool>;

    // Modifies self to have feature with value Some(true)
    fn set(&mut self, feature: Self::Index, b: bool);

    // Returns a copy of self where feature has value Some(true)
    // fn set_and_return(self, feature: Self::Index, b: bool) -> Self;
    
    // From a valid token, generate a FeatureBundle
    fn from_string(str: &str) -> Self;

    // From a FeatureBundle, generate a best match for token
    fn print(&self) -> String; 
   
    // All things after this point are to be audited

    // Outputs to newline separated string
    fn to_col_str(&self) -> String;
    // Outputs to 
    fn to_long_col_str(&self) -> String; //To a named column vector

    fn to_debug_string(&self) -> String; //To a row vector
    fn to_long_debug_string(&self) -> String; //To a named row vector

    fn modify(&mut self, modif: &SegModifier<Self::Index>);
}

pub fn feature_bundle_to_string<F: FeatureBundle>(fb: &F) -> String {
    let cons_chart = CONSONENT_CHART.concat();
    let non_pulm_cons_chart = NON_PULMONIC_CONSONANT_CHART.concat();
    let vow_chart = VOWEL_CHART.concat();

    let chart = [cons_chart, non_pulm_cons_chart, vow_chart].concat();

    //let fet_arr = self.to_isize_arr();
    let mut current_closest: usize = usize::MAX;
    let mut str: &str = "";
    //let c_iter: Vec<&&str> = chart.iter().filter(|&x| x!=&"").collect();
    //println!("{:?}", c_iter);
    for s in chart.iter().filter(|&x| x != &"") {
        //let s_arr = FeatureBundle::from_string(s).to_isize_arr();
        let dist = taxi_dist(fb, &F::from_string(s));

        if dist < current_closest {
            current_closest = dist;
            str = s;
        }
    }
    let mut base_fet = F::from_string(str);
    //println!("{}", self.to_debug_string());

    let mut result = str.to_owned();
    for _ in 0..3 {
        let mut temp_mod: SegModifier<F::Index> = SegModifier::default();
        for m in SegModifier::modifiers() {
            let mut temp_base_fet = base_fet.clone();

            temp_base_fet.modify(&m);

            let mod_dist = taxi_dist(&temp_base_fet, fb);

            if mod_dist < current_closest {
                current_closest = mod_dist;
                temp_mod = m.clone();
            }
        }
        base_fet.modify(&temp_mod);
        result += &temp_mod.token;
    }
    result
}

fn taxi_dist<F: FeatureBundle>(x: &F, y: &F) -> usize {
    let x_arr = to_isize_arr(x);
    let y_arr = to_isize_arr(y);
    _taxi_dist(&x_arr, &y_arr) as usize
}

fn _taxi_dist(x: &[isize], y: &[isize]) -> isize {
    let n = x.len();
    let mut val = 0;

    for i in 0..n {
        val = val + (x[i] - y[i]).abs();
    }
    val
}

//add weights
fn to_isize_arr<F: FeatureBundle>(fb: &F) -> Vec<isize> {
    let n = F::Index::index().len();
    let mut fet_arr = vec![0; n];
    let cont = F::Index::from_feature(Feature::Continuant);
    let stri = F::Index::from_feature(Feature::Strident);
    let dors = F::Index::from_feature(Feature::Dorsal);

    for i in 0..n {
        let tmp = F::Index::index()[i];
        let weight = if tmp == cont {
            2
        } else if tmp == stri {
            3
        } else {
            1
        };
        let val = match fb.get(F::Index::index()[i]) {
            Some(true) => weight,
            Some(false) => -weight,
            None => -weight,
        };

        fet_arr[i] = val;
    }
    fet_arr
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Feature {
    //Class
    Consonental, //SLATED FOR REMOVAL
    Continuant,  //Can be continuously produced
    Strident, //Stronger airflow? Distinguishes s - like sounds from other coronal fricatives, labiodental from labial, and trills from taps
    Lateral,  //l - like or sounds with airflow thtough both sides of mouth
    Sonorant, //Sounds that cause no turbulant flow or occlusion
    Syllabic, //Anything that serves as the nucleus of a syllable has this feature
    // Obstruant,
    // Fricative,
    // Occlusive,
    Voice,              //Decreases voice onset time
    SpreadGlottis,      //Increses voice onset time
    ConstrictedGlottis, //Closes the glottis during articulation

    Nasal,

    //Place
    Labial, //Involving the lips
    Round,  //Lip rounding during articulation

    Coronal, //Involving the tip or blade of the tongue
    Anterior,
    Distributed,

    Dorsal, //Involving the body or root of the tongue
    High,
    Low,
    Front,
    Back,

    //Laryngeal
    Pharyngeal, //Involving the pharynx, often conflated with the epiglottal POA, maybe consolidate with dorsal?
    AdvancedTongueRoot,

    //Manner
    DelayedRelease, //Denotes affricates SLATED FOR REMOVAL
    Length,
} //25 values

pub const FEATURES: [Feature; 24] = [
    Feature::Consonental,
    Feature::Continuant,
    Feature::Strident,
    Feature::Sonorant,
    Feature::Syllabic,
    // AltFeature::Obstruant,
    // AltFeature::Fricative,
    // AltFeature::Occlusive,
    Feature::Lateral,
    Feature::Voice,
    Feature::SpreadGlottis,
    Feature::ConstrictedGlottis,
    Feature::Nasal,
    Feature::Labial,
    Feature::Round,
    Feature::Coronal,
    Feature::Anterior,
    Feature::Distributed,
    Feature::Dorsal,
    Feature::High,
    Feature::Low,
    Feature::Front,
    Feature::Back,
    Feature::Pharyngeal,
    Feature::AdvancedTongueRoot,
    Feature::DelayedRelease,
    Feature::Length,
]; //25 values

const BREAK_AT: &[Feature] = &[
    Feature::Voice,
    Feature::Nasal,
    Feature::Labial,
    Feature::Coronal,
    Feature::Dorsal,
    Feature::Pharyngeal,
    Feature::DelayedRelease,
];

impl Display for Feature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Feature::Consonental => "Consonental",
            Feature::Sonorant => "Sonorant",
            Feature::Syllabic => "Syllabic",
            // AltFeature::Obstruant => "Obstruant",
            // AltFeature::Fricative => "Fricative",
            // AltFeature::Occlusive => "Occlusive",
            Feature::Labial => "Labial",
            Feature::Round => "Round",

            Feature::Coronal => "Coronal",
            Feature::Anterior => "Anterior",
            Feature::Distributed => "Distributed",

            Feature::Dorsal => "Dorsal",
            Feature::High => "High",
            Feature::Low => "Low",
            Feature::Front => "Front",
            Feature::Back => "Back",

            Feature::Pharyngeal => "Pharyngeal",
            Feature::AdvancedTongueRoot => "AdvancedTongueRoot",

            Feature::Voice => "Voice",
            Feature::SpreadGlottis => "SpreadGlottis",
            Feature::ConstrictedGlottis => "ConstrictedGlottis",

            Feature::Continuant => "Continuant",
            Feature::Strident => "Strident",
            Feature::Lateral => "Lateral",
            Feature::DelayedRelease => "DelayedRelease",
            Feature::Nasal => "Nasal",
            Feature::Length => "Long",
        };
        write!(f, "{str}")
    }
}

impl FeatureIndex for Feature {
    fn list_features() -> String {
        let mut result = String::new();
        for feature in FEATURES {
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
            Feature::Consonental => "Cons",
            Feature::Sonorant => "Son",
            Feature::Syllabic => "Syl",
            // AltFeature::Obstruant => "Obstruant",
            // AltFeature::Fricative => "Fricative",
            // AltFeature::Occlusive => "Occlusive",
            Feature::Labial => "Lab",
            Feature::Round => "Rnd",

            Feature::Coronal => "Cor",
            Feature::Anterior => "Ant",
            Feature::Distributed => "Dist",

            Feature::Dorsal => "Dors",
            Feature::High => "High",
            Feature::Low => "Low",
            Feature::Front => "Front",
            Feature::Back => "Back",

            Feature::Pharyngeal => "Phar",
            Feature::AdvancedTongueRoot => "ATR",

            Feature::Voice => "Voi",
            Feature::SpreadGlottis => "Spread",
            Feature::ConstrictedGlottis => "Constricted",

            Feature::Continuant => "Cont",
            Feature::Strident => "Str",
            Feature::Lateral => "Lat",
            Feature::DelayedRelease => "DelRel",
            Feature::Nasal => "Nas",
            Feature::Length => "Len",
        }
    }

    fn from_string(str: &str) -> Option<Self> {
        match str {
            "consonental" | "cons" => Some(Feature::Consonental),
            "sonorant" | "son" => Some(Feature::Sonorant),
            "syllabic" | "syl" => Some(Feature::Syllabic),

            "labial" | "lab" => Some(Feature::Labial),
            "round" | "rnd" => Some(Feature::Round),

            "coronal" | "cor" => Some(Feature::Coronal),
            "anterior" | "ant" => Some(Feature::Anterior),
            "distributed" | "dist" => Some(Feature::Distributed),

            "dorsal" | "dors" => Some(Feature::Dorsal),
            "high" => Some(Feature::High),
            "low" => Some(Feature::Low),
            "front" => Some(Feature::Front),
            "back" => Some(Feature::Back),

            "Pharyngeal" | "phar" => Some(Feature::Pharyngeal),
            "advancedTongueRoot" | "atr" => Some(Feature::AdvancedTongueRoot),

            "voice" | "voi" => Some(Feature::Voice),
            "spreadGlottis" | "sprgl" => Some(Feature::SpreadGlottis),
            "constrictedGlottis" | "congl" => Some(Feature::ConstrictedGlottis),

            "continuant" | "cont" => Some(Feature::Continuant),
            "strident" | "str" => Some(Feature::Strident),
            "lateral" | "lat" => Some(Feature::Lateral),
            "delayedRelease" | "delrel" => Some(Feature::DelayedRelease),
            "nasal" | "nas" => Some(Feature::Nasal),
            "length" | "len" | "long" => Some(Feature::Length),
            _ => None,
        }
    }
    fn parse(str: &str) -> Option<Vec<(Feature, bool)>> {
        let mut result = Vec::new();
        let str_arr = unbracket_arr(str, ("[", "]"))?;

        for str in str_arr {
            let (pre, rest) = str.split_at(1);
            match pre {
                "+" => {
                    // println!("attempting to parse {pre}{rest}");
                    let f = Feature::from_string(rest)?;
                    // println!("Parsed {pre}{rest} and received ({f:?}, true)");
                    result.push((f, true));
                }
                "-" => {
                    // println!("attempting to parse {pre}{rest}");
                    let f = Feature::from_string(rest)?;
                    // println!("Parsed {pre}{rest} and received ({f:?}, false)");
                    result.push((f, false));
                }
                _ => {
                    eprintln!("ERROR@<Feature as FeatureIndex>::parse(): No valid boolian value can be parsed from {pre}{rest}");
                }
            }
        }

        Some(result)
    }

    fn from_feature(f: Feature) -> Self {
        f
    }

    fn index<'a>() -> &'a [Self] {
        &FEATURES
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum FeatureShell<I: FeatureIndex> {
    F(I),
    Affricate,
}

impl<I: FeatureIndex> Display for FeatureShell<I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FeatureShell::Affricate => write!(f, "Affricate"),
            FeatureShell::F(fet) => write!(f, "{fet}"),
        }
    }
}

impl<I: FeatureIndex> FeatureIndex for FeatureShell<I> {
    fn list_features() -> String {
        todo!()
    }

    fn to_short_string(&self) -> &'static str {
        match self {
            FeatureShell::F(f) => f.to_short_string(),
            FeatureShell::Affricate => "affr",
        }
    }

    fn from_string(str: &str) -> Option<Self> {
        match str {
            "Affricate" | "Affr" | "affr" | "affricate" => Some(FeatureShell::Affricate),
            str => I::from_string(str).map(|f| FeatureShell::F(f)),
        }
    }

    fn parse(str: &str) -> Option<Vec<(Self, bool)>> {
        let mut result = Vec::new();
        let str_arr = unbracket_arr(str, ("[", "]"))?;

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
                    eprintln!("ERROR@<Feature as FeatureIndex>::parse(): No valid boolian value can be parsed from {pre}{rest}");
                }
            }
        }

        Some(result)
    }

    fn from_feature(f: Feature) -> Self {
        todo!()
    }

    fn index<'a>() -> &'a [Self] {
        todo!()
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct ComplexFeatureBundleShell<F: FeatureBundle> {
    left: F,
    right: F,
}

impl<F: FeatureBundle> FeatureBundle for ComplexFeatureBundleShell<F> {
    type Index = F::Index;

    fn new() -> Self {
        ComplexFeatureBundleShell {
            left: F::new(),
            right: F::new(),
        }
    }

    fn get(&self, feature: Self::Index) -> Option<bool> {
        if self.left == self.right {
            self.left.get(feature)
        } else {
            None
        }
    }

    fn get_right(&self, feature: Self::Index) -> Option<bool> {
        self.right.get(feature)
    }

    fn get_left(&self, feature: Self::Index) -> Option<bool> {
        self.left.get(feature)
    }

    fn set(&mut self, feature: Self::Index, b: bool) {
        self.left.set(feature, b);
        self.right.set(feature, b);
    }

    fn from_string(str: &str) -> Self {
        // println!("Seg: {str}");
        let tokens = xsampa::xsampa_split(str);
        let binding = ["-/".to_string()];
        let split = tokens.split_on_first(&binding);

        // let split = str.split_once("-/");
        // println!("{split:?}");
        match split {
            Some((init, release)) => {
                // println!("Complex Segment");
                let left = F::from_string(&init.concat());
                let right = F::from_string(&release.concat());
                ComplexFeatureBundleShell { left, right }
            }
            None => {
                let binding2 = ["-".to_string()];
                let split2 = tokens.split_on_first(&binding2);
                // println!("{split:?}");
                match split2 {
                    Some((init, release)) => {
                        // println!("Complex Segment");
                        let left = F::from_string(&init.concat());
                        let right = F::from_string(&release.concat());
                        ComplexFeatureBundleShell { left, right }
                    }
                    None => {
                        let fb = F::from_string(str);
                        ComplexFeatureBundleShell {
                            left: fb,
                            right: fb,
                        }
                    }
                }
            }
        }
    }

    fn print(&self) -> String {
        if self.left == self.right {
            self.left.print()
        } else {
            let str1 = self.left.print();
            let str2 = self.right.print();

            str1 + "-/" + &str2
        }
    }

    fn to_col_str(&self) -> String {
        if self.left == self.right {
            self.left.to_col_str()
        } else {
            let str1 = self.left.to_col_str();
            let str2 = self.right.to_col_str();

            util::utilities::concat_by_line(&[str1, str2], "")
        }
    }

    fn to_long_col_str(&self) -> String {
        todo!()
    }

    fn to_debug_string(&self) -> String {
        self.to_col_str()
    }

    fn to_long_debug_string(&self) -> String {
        todo!()
    }

    fn modify(&mut self, modif: &SegModifier<Self::Index>) {
        self.left.modify(modif);
        self.right.modify(modif);
    }
}

pub fn string_to_intermediate_repr(_s: &str) -> Option<(u8, u8, u8)> {
    let tokens = xsampa::xsampa_split(_s);
    // let mut token_iterator = tokens.iter().map(|x| xsampa::token_type(x));

    //Check if initial is a valid segmental token, this needs to be reworked
    // if token_iterator.next() != Some((true, false)) {
    //     eprintln!("Error: Not a valid initial\n");
    //     println!("{:?}\n", tokens);
    // }
    //Check if the trailing tokens are valid non-segmental tokens
    // for temp in token_iterator.skip(1) {
    //     if temp != (true, true) {
    //         eprintln!("Error: Invalid token found\n");
    //         println!("{:?}\n", tokens);
    //     }
    // }

    //let modifs: Vec<String> = tokens.iter().skip(1).map(|x| x.to_string()).collect();
    //Is the initial a consonent?
    let init = &tokens[0];
    let mut manner_number: u8 = 0;
    let mut place_number: u8 = 0;
    let mut chart_number: u8 = 0;

    for list in CONSONENT_CHART {
        for token in list {
            if init == token {
                return Some((chart_number, manner_number, place_number));
            }
            place_number += 1;
        }
        place_number = 0;
        manner_number += 1;
    }
    chart_number += 1;
    manner_number = 0;
    place_number = 0;

    for list in NON_PULMONIC_CONSONANT_CHART {
        for token in list {
            if init == token {
                return Some((chart_number, manner_number, place_number));
            }
            place_number += 1;
        }
    }
    chart_number += 1;
    manner_number = 0;
    place_number = 0;

    for list in VOWEL_CHART {
        for token in list {
            if init == token {
                return Some((chart_number, manner_number, place_number));
            }
            place_number += 1;
        }
        place_number = 0;
        manner_number += 1;
    }
    // if fet == AltFeatureBundle::new() {
    //     eprintln!("ERROR: no valid token found");
    // }

    //for t in modifs {
    //    let modif = SegModif::from_string(&t);
    //    match modif {
    //        Some(modifier) => fet.modify(&modifier),
    //        None =>(),
    //    }
    //}
    Some((chart_number, manner_number, place_number))
}

pub static CONSONENT_CHART: [[&str; 22]; 10] = [
    [
        "p", "b", "", "", "", "", "t", "d", "", "", "t`", "d`", "c", "J\\", "k", "g", "q", "G\\",
        "", "", "?", "",
    ],
    [
        "", "b_<", "", "", "", "", "", "d_<", "", "", "", "D_<", "", "f_<", "", "g_<", "", "G\\_<",
        "", "", "", "",
    ],
    [
        "p\\", "B", "", "", "T", "D", "", "", "", "", "", "", "C", "j\\", "x", "G", "X", "R",
        "X\\", "?\\", "h", "h\\",
    ],
    [
        "f", "v", "", "", "", "", "s", "z", "S", "Z", "s`", "z`", "s\\", "z\\", "", "", "", "", "",
        "", "", "",
    ],
    [
        "", "", "", "", "", "", "K", "K\\", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
    ],
    [
        "", "m", "", "F", "", "", "", "n", "", "", "", "n`", "", "J", "", "N", "", "N\\", "", "",
        "", "",
    ],
    [
        "", "", "", "", "", "", "", "l", "", "", "", "l`", "", "L", "", "L\\", "", "", "", "", "",
        "",
    ],
    [
        "", "", "", "", "", "", "", "4", "", "", "", "r`", "", "", "", "", "", "", "", "", "", "",
    ],
    [
        "", "B\\", "", "", "", "", "", "r", "", "", "", "", "", "", "", "", "", "R\\", "", "", "", "",
    ],
    [
        "", "", "", "v\\", "", "", "", "r\\", "", "", "", "r\\`", "", "", "", "", "", "", "", "",
        "", "",
    ],
];

pub static AFFRICATE_CONSONANT_CHART: &[&[&str]] = &[
    //exludes ejectives for reasons
    &[
        "p_f", "b_v", "t_T", "d_D", "t_s", "d_z", "t_S", "d_Z", "t_s`", "d_z`", "t_s\\", "d_z\\",
        "k_x", "g_G", "q_X", "G\\_R",
    ],
];

pub static NON_PULMONIC_CONSONANT_CHART: [[&str; 5]; 2] = [
    //exludes ejectives for reasons
    ["b_<", "d_<", "f_<", "g_<", "G_<"],
    ["O\\", "|\\", "!\\", "=\\", "|\\|\\"], // |\\|\\ is not being parsed properly
];
//Front     central     "back"
//needs 3 features to describe height???
// pub static VOWEL_CHART: [[&str; 6]; 8] = [
//     ["j", "H", "", "", "M\\", "w"],
//     ["i", "y", "1", "}", "M", "u"], //+high +tense -mid -low OR +raised -retracted
//     ["I", "Y", "", "", "", "U"],    //                       OR +raised +retracted
//     ["e", "2", "@\\", "8", "7", "o"], //+high +tense +mid -low OR -raised -retracted
//     ["", "", "@", "", "", ""],      //-high -tense +mid -low OR -raised
//     ["E", "9", "3", "3\\", "V", "O"], //-high -tense +mid +low OR -raised +retracted
//     ["{", "", "6", "", "", ""],     //-high -tense -mid +low OR -raised +retracted
//     ["a", "&", "a_\"", "", "A", "Q"], //-high +tense -mid +low OR -raised +retracted
// ]; //front                      back
pub static VOWEL_CHART: [[&str; 6]; 8] = [
    ["j", "H", "", "", "M\\", "w"],
    ["i", "y", "1", "}", "M", "u"], //+high +tense -mid -low OR +raised -retracted
    ["I", "Y", "", "", "", "U"],    //                       OR +raised +retracted
    ["e", "2", "@\\", "8", "7", "o"], //+high +tense +mid -low OR -raised -retracted
    ["", "", "@", "", "", ""],      //-high -tense +mid -low OR -raised
    ["E", "9", "3", "3\\", "V", "O"], //-high -tense +mid +low OR -raised +retracted
    ["", "", "6", "", "", ""],      //-high -tense -mid +low OR -raised +retracted
    ["{", "", "a", "&", "A", "Q"],  //-high +tense -mid +low OR -raised +retracted
]; //front                      back

//const MASTER_LIST: &[&str] = concat!(
//    CONSONENT_CHART.concat().as_slice(),
//    NON_PULMONIC_CONSONANT_CHART.concat().as_slice(),
//    VOWEL_CHART.concat().as_slice(),
//);

//static ALT_VOWEL_CHART: = [
//    [],
//]
//static SEGMENT_MODIFIERS: =[];
