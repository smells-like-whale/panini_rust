use std::ops::Deref;

use std::fmt::Debug;

use crate::segment::segment_modifier::SegModifier;
use crate::{segment::SegModif, xsampa};

use super::{
    Feature, FeatureBundle, FeatureIndex, CONSONENT_CHART, NON_PULMONIC_CONSONANT_CHART,
    VOWEL_CHART,
};

const BREAK_AT: &[Feature] = &[
    Feature::Voice,
    Feature::Nasal,
    Feature::Labial,
    Feature::Coronal,
    Feature::Dorsal,
    Feature::Pharyngeal,
    Feature::DelayedRelease,
];

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Labial {
    round: bool,
}
impl Labial {
    const fn new() -> Self {
        Self { round: false }
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Pharyngeal {
    atr: bool,
}
impl Pharyngeal {
    const fn new() -> Self {
        Self { atr: false }
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct FeatureTable {
    //there are 21 values
    //CLASS FEATURES
    consonental: bool,
    sonorant: bool,
    syllabic: bool,
    // obstruant: Option<Obstruant>,
    //approximant: bool,

    //PLACE FEATURES
    labial: Option<Labial>,

    coronal: Option<Coronal>,

    dorsal: Option<Dorsal>,

    pharyngeal: Option<Pharyngeal>,

    //LARYNGEAL FEATURES
    voice: bool,
    spread_glottis: bool,
    constricted_glottis: bool,

    continuant: bool,
    nasal: bool,
    strident: bool,
    lateral: bool,
    delayed_release: bool,
    long: bool,
    //modifiers: Vec<String>,
}

impl FeatureBundle for FeatureTable {
    type Index = Feature;

    fn new() -> Self {
        Self {
            consonental: false,
            sonorant: false,
            syllabic: false,
            // obstruant: None,
            labial: None,
            coronal: None,
            dorsal: None,
            pharyngeal: None,

            voice: false,
            spread_glottis: false,
            constricted_glottis: false,

            continuant: false,
            nasal: false,
            strident: false,
            lateral: false,
            delayed_release: false,
            long: false,
        }
    }

    //     fn get(&self, fet: Feature) -> bool {
    //         match self.get_opt(fet) {
    //             Some(b) => b,
    //             None => false,
    //         }
    //      }

    fn get(&self, fet: Feature) -> Option<bool> {
        match fet {
            Feature::Consonental => Some(self.consonental),
            Feature::Sonorant => Some(self.sonorant),
            Feature::Syllabic => Some(self.syllabic),
            // AltFeature::Obstruant => Some(self.obstruant.is_some()),
            // AltFeature::Fricative => {
            //         match self.obstruant {
            //             Some(val) => Some(val.fricative),
            //             None => None,
            //     }
            // }
            // AltFeature::Occlusive => {
            //         match self.obstruant {
            //             Some(val) => Some(val.occlusive),
            //             None => None,
            //     }
            // }
            Feature::Labial => Some(self.labial.is_some()),
            Feature::Round => match self.labial {
                Some(val) => Some(val.round),
                None => None,
            },
            Feature::Coronal => Some(self.coronal.is_some()),
            Feature::Anterior => match self.coronal {
                Some(val) => Some(val.anterior),
                None => None,
            },
            Feature::Distributed => match self.coronal {
                Some(val) => Some(val.distributed),
                None => None,
            },
            Feature::Dorsal => Some(self.dorsal.is_some()),
            Feature::Front => match self.dorsal {
                Some(val) => Some(val.front),
                None => None,
            },
            Feature::Back => match self.dorsal {
                Some(val) => Some(val.back),
                None => None,
            },
            Feature::High => match self.dorsal {
                Some(val) => Some(val.high),
                None => None,
            },
            Feature::Low => match self.dorsal {
                Some(val) => Some(val.low),
                None => None,
            },
            Feature::Pharyngeal => Some(self.pharyngeal.is_some()),

            Feature::AdvancedTongueRoot => {
                let this = self.pharyngeal;
                match this {
                    Some(val) => Some(val.atr),
                    None => Some(false),
                }
            }

            Feature::Voice => Some(self.voice),
            Feature::SpreadGlottis => Some(self.spread_glottis),
            Feature::ConstrictedGlottis => Some(self.constricted_glottis),

            Feature::Continuant => Some(self.continuant),
            Feature::Nasal => Some(self.nasal),
            Feature::Strident => Some(self.strident),
            Feature::Lateral => Some(self.lateral),
            Feature::DelayedRelease => Some(self.delayed_release),
            Feature::Length => Some(self.long),
        }
    }

    fn get_right(&self, feature: Self::Index) -> Option<bool> {
        self.get(feature)
    }

    fn get_left(&self, feature: Self::Index) -> Option<bool> {
        self.get(feature)
    }

    fn set(&mut self, fet: Feature, b: bool) {
        match fet {
            Feature::Consonental => {
                self.consonental = b;
            }
            Feature::Sonorant => {
                self.sonorant = b;
            }
            Feature::Syllabic => {
                self.syllabic = b;
            }
            // AltFeature::Obstruant => {
            //     if b {
            //         self.obstruant = match self.obstruant {
            //             Some(x) => Some(x),
            //             None => Some(Obstruant::new()),
            //         }
            //     } else {
            //         self.obstruant = None;
            //     }
            // }
            // AltFeature::Fricative => match self.obstruant {
            //     Some(mut x) => x.fricative = b,
            //     None => {
            //         self.obstruant = match b {
            //             true => {
            //                 Some(Obstruant {
            //                     occlusive: false,
            //                     fricative: true })
            //             }
            //             false => None,
            //         }
            //     }
            // },
            // AltFeature::Occlusive => match self.obstruant {
            //     Some(mut x) => x.occlusive = b,
            //     None => {
            //         self.obstruant = match b {
            //             true => {
            //                 Some(Obstruant {
            //                     occlusive: true,
            //                     fricative: false })
            //             }
            //             false => None,
            //         }
            //     }
            // },
            Feature::Labial => {
                if b {
                    self.labial = match self.labial {
                        Some(x) => Some(x),
                        None => Some(Labial::new()),
                    }
                } else {
                    self.labial = None;
                }
            }
            Feature::Round => match self.labial {
                Some(mut x) => {
                    x.round = b;
                    self.labial = Some(x);
                }
                None => {
                    self.labial = if b {
                        Some(Labial { round: true })
                    } else {
                        None
                    }
                }
            },
            Feature::Coronal => {
                if b {
                    self.coronal = match self.coronal {
                        Some(x) => Some(x),
                        None => Some(Coronal::new()),
                    }
                } else {
                    self.coronal = None;
                }
            }
            Feature::Anterior => match self.coronal {
                Some(mut x) => {
                    x.anterior = b;
                    self.coronal = Some(x);
                }
                None => {
                    self.coronal = if b {
                        Some(Coronal {
                            anterior: true,
                            distributed: false,
                        })
                    } else {
                        None
                    }
                }
            },
            Feature::Distributed => match self.coronal {
                Some(mut x) => {
                    x.distributed = b;
                    self.coronal = Some(x);
                }
                None => {
                    self.coronal = if b {
                        Some(Coronal {
                            anterior: false,
                            distributed: true,
                        })
                    } else {
                        None
                    }
                }
            },
            Feature::Dorsal => {
                if b {
                    self.dorsal = match self.dorsal {
                        Some(x) => Some(x),
                        None => Some(Dorsal::new()),
                    }
                } else {
                    self.dorsal = None;
                }
            }
            Feature::Front => match self.dorsal {
                Some(mut x) => {
                    x.front = b;
                    self.dorsal = Some(x);
                    // println!("I am now fronter!!! {}", x.front);
                }
                None => {
                    self.dorsal = if b {
                        Some(Dorsal {
                            front: true,
                            high: false,
                            back: false,
                            low: false,
                        })
                    } else {
                        None
                    }
                }
            },
            Feature::Back => match self.dorsal {
                Some(mut x) => {
                    x.back = b;
                    self.dorsal = Some(x);
                }
                None => {
                    self.dorsal = if b {
                        Some(Dorsal {
                            front: false,
                            back: true,
                            high: false,
                            low: false,
                        })
                    } else {
                        None
                    }
                }
            },
            Feature::High => match self.dorsal {
                Some(mut x) => {
                    x.high = b;
                    self.dorsal = Some(x);
                    // println!("I am now higher {}", x.high);
                }
                None => {
                    self.dorsal = if b {
                        Some(Dorsal {
                            front: false,
                            back: false,
                            high: true,
                            low: false,
                        })
                    } else {
                        None
                    }
                }
            },
            Feature::Low => match self.dorsal {
                Some(mut x) => {
                    x.low = b;
                    self.dorsal = Some(x);
                }
                None => {
                    self.dorsal = if b {
                        Some(Dorsal {
                            front: false,
                            high: false,
                            back: false,
                            low: true,
                        })
                    } else {
                        None
                    }
                }
            },
            Feature::Pharyngeal => {
                if b {
                    self.pharyngeal = match self.pharyngeal {
                        Some(x) => Some(x),
                        None => Some(Pharyngeal::new()),
                    }
                } else {
                    self.pharyngeal = None;
                }
            }
            Feature::AdvancedTongueRoot => match self.pharyngeal {
                Some(mut x) => {
                    x.atr = b;
                    self.pharyngeal = Some(x);
                }
                None => {
                    self.pharyngeal = if b {
                        Some(Pharyngeal { atr: true })
                    } else {
                        None
                    }
                }
            },
            Feature::Voice => {
                self.voice = b;
            }
            Feature::SpreadGlottis => {
                self.spread_glottis = b;
            }
            Feature::ConstrictedGlottis => {
                self.constricted_glottis = b;
            }

            Feature::Continuant => {
                self.continuant = b;
            }
            Feature::Nasal => {
                self.nasal = b;
            }
            Feature::Strident => {
                self.strident = b;
            }
            Feature::Lateral => {
                self.lateral = b;
            }
            Feature::DelayedRelease => {
                self.delayed_release = b;
            }
            Feature::Length => {
                self.long = b;
            }
        }
    }

    fn from_string(str: &str) -> Self {
        let str_tokens = xsampa::xsampa_split(str);

        let (c, m, p) = super::string_to_intermediate_repr(&str_tokens[0]).unwrap();
        let mut result = from_token_intermediate(c, m, p);

        for t in 1..str_tokens.len() {
            match SegModifier::<Feature>::from_string(&str_tokens[t]) {
                Some(modif) => {
                    result.modify(&modif);
                }
                None => {}
            }
        }

        result
    }

    fn print(&self) -> String {
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
            let dist = FeatureTable::taxi_dist(&self, &FeatureTable::from_string(s));

            if dist < current_closest {
                current_closest = dist;
                str = s;
            }
        }
        let mut base_fet = FeatureTable::from_string(str);
        //println!("{}", self.to_debug_string());

        let mut result = str.to_owned();
        for _ in 0..3 {
            let mut temp_mod: SegModifier<Feature> = SegModifier::default();
            for m in SegModifier::modifiers() {
                let mut temp_base_fet = base_fet.clone();

                temp_base_fet.modify(&m);

                let mod_dist = FeatureTable::taxi_dist(&temp_base_fet, &self);

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

    fn modify(&mut self, modifier: &SegModifier<Feature>) {
        //self.modify_feature(modifier.modif[0].0, modifier.modif[0].1);
        for &(f, b) in &modifier.modif {
            self.set(f, b);
        }
    }
}

impl FeatureTable {
    pub const fn new() -> Self {
        Self {
            consonental: false,
            continuant: false,
            sonorant: false,
            syllabic: false,
            strident: false,
            lateral: false,
            // obstruant: None,
            voice: false,
            spread_glottis: false,
            constricted_glottis: false,

            nasal: false,

            labial: None,
            coronal: None,
            dorsal: None,
            pharyngeal: None,

            delayed_release: false,
            long: false,
        }
    }
    pub const fn set_and_return(mut self, fet: Feature, b: bool) -> Self {
        match fet {
            Feature::Consonental => {
                self.consonental = b;
            }
            Feature::Sonorant => {
                self.sonorant = b;
            }
            Feature::Syllabic => {
                self.syllabic = b;
            }
            // AltFeature::Obstruant => {
            //     if b {
            //         self.obstruant = match self.obstruant {
            //             Some(x) => Some(x),
            //             None => Some(Obstruant::new()),
            //         }
            //     } else {
            //         self.obstruant = None;
            //     }
            // }
            // AltFeature::Fricative => match self.obstruant {
            //     Some(mut x) => x.fricative = b,
            //     None => {
            //         self.obstruant = match b {
            //             true => {
            //                 Some(Obstruant {
            //                     occlusive: false,
            //                     fricative: true })
            //             }
            //             false => None,
            //         }
            //     }
            // },
            // AltFeature::Occlusive => match self.obstruant {
            //     Some(mut x) => x.occlusive = b,
            //     None => {
            //         self.obstruant = match b {
            //             true => {
            //                 Some(Obstruant {
            //                     occlusive: true,
            //                     fricative: false })
            //             }
            //             false => None,
            //         }
            //     }
            // },
            Feature::Labial => {
                if b {
                    self.labial = match self.labial {
                        Some(x) => Some(x),
                        None => Some(Labial::new()),
                    }
                } else {
                    self.labial = None;
                }
            }
            Feature::Round => match self.labial {
                Some(mut x) => x.round = b,
                None => {
                    self.labial = if b {
                        Some(Labial { round: true })
                    } else {
                        None
                    }
                }
            },
            Feature::Coronal => {
                if b {
                    self.coronal = match self.coronal {
                        Some(x) => Some(x),
                        None => Some(Coronal::new()),
                    }
                } else {
                    self.coronal = None;
                }
            }
            Feature::Anterior => match self.coronal {
                Some(mut x) => x.anterior = b,
                None => {
                    self.coronal = if b {
                        Some(Coronal {
                            anterior: true,
                            distributed: false,
                        })
                    } else {
                        None
                    }
                }
            },
            Feature::Distributed => match self.coronal {
                Some(mut x) => x.distributed = b,
                None => {
                    self.coronal = if b {
                        Some(Coronal {
                            anterior: false,
                            distributed: true,
                        })
                    } else {
                        None
                    }
                }
            },
            Feature::Dorsal => {
                if b {
                    self.dorsal = match self.dorsal {
                        Some(x) => Some(x),
                        None => Some(Dorsal::new()),
                    }
                } else {
                    self.dorsal = None;
                }
            }
            Feature::Front => match self.dorsal {
                Some(mut x) => x.front = b,
                None => {
                    self.dorsal = if b {
                        Some(Dorsal {
                            front: true,
                            high: false,
                            back: false,
                            low: false,
                        })
                    } else {
                        None
                    }
                }
            },
            Feature::Back => match self.dorsal {
                Some(mut x) => x.back = b,
                None => {
                    self.dorsal = if b {
                        Some(Dorsal {
                            front: false,
                            back: true,
                            high: false,
                            low: false,
                        })
                    } else {
                        None
                    }
                }
            },
            Feature::High => match self.dorsal {
                Some(mut x) => x.high = b,
                None => {
                    self.dorsal = if b {
                        Some(Dorsal {
                            front: false,
                            back: false,
                            high: true,
                            low: false,
                        })
                    } else {
                        None
                    }
                }
            },
            Feature::Low => match self.dorsal {
                Some(mut x) => x.low = b,
                None => {
                    self.dorsal = if b {
                        Some(Dorsal {
                            front: false,
                            high: false,
                            back: false,
                            low: true,
                        })
                    } else {
                        None
                    }
                }
            },
            Feature::Pharyngeal => {
                if b {
                    self.pharyngeal = match self.pharyngeal {
                        Some(x) => Some(x),
                        None => Some(Pharyngeal::new()),
                    }
                } else {
                    self.pharyngeal = None;
                }
            }
            Feature::AdvancedTongueRoot => match self.pharyngeal {
                Some(mut x) => x.atr = b,
                None => {
                    self.pharyngeal = if b {
                        Some(Pharyngeal { atr: true })
                    } else {
                        None
                    }
                }
            },
            Feature::Voice => {
                self.voice = b;
            }
            Feature::SpreadGlottis => {
                self.spread_glottis = b;
            }
            Feature::ConstrictedGlottis => {
                self.constricted_glottis = b;
            }

            Feature::Continuant => {
                self.continuant = b;
            }
            Feature::Nasal => {
                self.nasal = b;
            }
            Feature::Strident => {
                self.strident = b;
            }
            Feature::Lateral => {
                self.lateral = b;
            }
            Feature::DelayedRelease => {
                self.delayed_release = b;
            }
            Feature::Length => {
                self.long = b;
            }
        }
        self
    }
    fn taxi_dist(x: &Self, y: &Self) -> usize {
        let x_arr = x.to_isize_arr();
        let y_arr = y.to_isize_arr();
        FeatureTable::_taxi_dist(x_arr, y_arr) as usize
    }

    fn _taxi_dist(x: [isize; 24], y: [isize; 24]) -> isize {
        let mut val = 0;

        for i in 0..24 {
            val = val + (x[i] - y[i]).abs();
        }
        val
    }

    //add weights
    fn to_isize_arr(self) -> [isize; 24] {
        let mut fet_arr: [isize; 24] = [0; 24];

        for i in 0..24 {
            let weight = match <FeatureTable as FeatureBundle>::Index::index()[i] {
                Feature::Continuant => 2,
                Feature::Strident => 3,
                _ => 1,
            };
            let val = match self.get(<FeatureTable as FeatureBundle>::Index::index()[i]) {
                Some(true) => weight,
                Some(false) => -weight,
                None => 0,
            };

            fet_arr[i] = val;
        }
        fet_arr
    }

    fn from_token_old(_s: &str) -> FeatureTable {
        let tokens = xsampa::xsampa_split(_s);
        let mut token_iterator = tokens.iter().map(|x| xsampa::token_type(x));
        //Check if initial is a valid segmental token
        if token_iterator.next() != Some((true, false)) {
            eprintln!("Error: Not a valid initial\n");
            println!("{:?}\n", tokens);
        }
        //Check if the trailing tokens are valid non-segmental tokens
        for temp in token_iterator.skip(1) {
            if temp != (true, true) {
                eprintln!("Error: Invalid token found\n");
                println!("{:?}\n", tokens);
            }
        }

        //let modifs: Vec<String> = tokens.iter().skip(1).map(|x| x.to_string()).collect();
        //Is the initial a consonent?
        let init = &tokens[0];
        let mut manner_number: u8 = 0;
        let mut place_number: u8 = 0;
        let mut chart_number: u8 = 0;

        let mut fet = FeatureTable::new();

        for list in CONSONENT_CHART {
            for token in list {
                if init == token {
                    fet = from_token_intermediate(chart_number, manner_number, place_number);
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
                    fet = from_token_intermediate(chart_number, manner_number, place_number);
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
                    fet = from_token_intermediate(chart_number, manner_number, place_number);
                }
                place_number += 1;
            }
            place_number = 0;
            manner_number += 1;
        }
        if fet == FeatureTable::new() {
            eprintln!("ERROR: no valid token found");
        }

        //for t in modifs {
        //    let modif = SegModif::from_string(&t);
        //    match modif {
        //        Some(modifier) => fet.modify(&modifier),
        //        None =>(),
        //    }
        //}
        fet
    }
}

fn from_token_intermediate(chart: u8, manner: u8, place: u8) -> FeatureTable {
    let mut temp: FeatureTable = FeatureTable::new();
    match chart {
        0 => {
            // temp.set(AltFeature::Consonental, true);
            //temp.set(AltFeature::Syllabic, false);
            //temp.set(AltFeature::SpreadGlottis, false);
            //temp.set(AltFeature::ConstrictedGlottis, false);
            temp.set(Feature::Voice, place % 2 != 0);
            match manner {
                0 => {
                    //plosive
                    temp.set(Feature::Consonental, true);
                    //temp.set(AltFeature::Sonorant, false);
                    //temp.set(AltFeature::Nasal, false);
                    //temp.set(AltFeature::Strident, false);
                    //temp.set(AltFeature::Lateral, false);
                    //temp.set(AltFeature::DelayedRelease, false);
                //Implosive
                }
                1 => {
                    //fricative
                    temp.set(Feature::Consonental, true);
                    //temp.set(AltFeature::Sonorant, false);
                    temp.set(Feature::Continuant, true);
                    //temp.set(AltFeature::Nasal, false);
                    //temp.set(AltFeature::Strident, false);
                    //temp.set(AltFeature::Lateral, false);
                    //temp.set(AltFeature::DelayedRelease, false);
                }
                2 => {
                    //Strident Fricative
                    temp.set(Feature::Consonental, true);
                    //temp.set(AltFeature::Sonorant, false);
                    temp.set(Feature::Continuant, true);
                    //temp.set(AltFeature::Nasal, false);
                    temp.set(Feature::Strident, true);
                    //temp.set(AltFeature::Lateral, false);
                    //temp.set(AltFeature::DelayedRelease, false);
                }
                4 => {
                    //Lateral fricatives
                    temp.set(Feature::Consonental, true);
                    temp.set(Feature::Continuant, true);
                    //temp.set(AltFeature::Sonorant, true);
                    //temp.set(AltFeature::Nasal, true);
                    //temp.set(AltFeature::Strident, false);
                    temp.set(Feature::Lateral, true);
                    //temp.set(AltFeature::DelayedRelease, false);
                }
                3 => {
                    //nasal
                    temp.set(Feature::Consonental, true);
                    temp.set(Feature::Sonorant, true);
                    //temp.set(AltFeature::Continuant, false);
                    temp.set(Feature::Nasal, true);
                    //temp.set(AltFeature::Strident, false);
                    //temp.set(AltFeature::Lateral, false);
                    //temp.set(AltFeature::DelayedRelease, false);
                }
                8 => {
                    //Lateral
                    //temp.set(AltFeature::Approximant, true);
                    temp.set(Feature::Consonental, true);
                    temp.set(Feature::Continuant, true);
                    temp.set(Feature::Sonorant, true);
                    // temp.set(Feature::Nasal, false);
                    // temp.set(Feature::Strident, false);
                    temp.set(Feature::Lateral, true);
                    // temp.set(Feature::DelayedRelease, false);
                }
                7 => {
                    //taps
                    //temp.set(AltFeature::Approximant, false);
                    temp.set(Feature::Sonorant, true);
                    //temp.set(AltFeature::Continuant, true);
                    //temp.set(AltFeature::Nasal, false);
                    //temp.set(AltFeature::Strident, false);
                    //temp.set(AltFeature::Lateral, false);
                    //temp.set(AltFeature::DelayedRelease, false);
                }
                5 => {
                    //trills
                    temp.set(Feature::Sonorant, true);
                    temp.set(Feature::Continuant, true);
                    temp.set(Feature::Consonental, true);
                    //temp.set(AltFeature::Nasal, true);
                    //temp.set(AltFeature::Strident, false);
                    //temp.set(AltFeature::Lateral, false);
                    //temp.set(AltFeature::DelayedRelease, false);
                }
                6 => {
                    //approximates
                    //temp.set(AltFeature::Approximant, false);
                    temp.set(Feature::Sonorant, true);
                    temp.set(Feature::Continuant, true);
                    // temp.set(AltFeature::Nasal, true);
                    // temp.set(AltFeature::Strident, false);
                    // temp.set(AltFeature::Lateral, false);
                    // temp.set(AltFeature::DelayedRelease, false);
                }
                _ => {}
            }

            match place / 2 {
                0 => {
                    //biLabial
                    temp.set(Feature::Labial, true);
                    temp.set(Feature::Coronal, false);
                    //temp.set(AltFeature::Anterior   , false);
                    //temp.set(AltFeature::Distributed, false);
                    temp.set(Feature::Dorsal, false);
                    temp.set(Feature::Pharyngeal, false);
                }
                1 => {
                    //labiodental
                    temp.set(Feature::Labial, true);
                    temp.set(Feature::Coronal, false);
                    //temp.set(AltFeature::Anterior   , false);
                    //temp.set(AltFeature::Distributed, false);
                    temp.set(Feature::Dorsal, false);
                    temp.set(Feature::Pharyngeal, false);
                }
                2 => {
                    //dental
                    temp.set(Feature::Labial, false);
                    temp.set(Feature::Coronal, true);
                    temp.set(Feature::Anterior, true);
                    temp.set(Feature::Distributed, true);
                    temp.set(Feature::Dorsal, false);
                    temp.set(Feature::Pharyngeal, false);
                }
                3 => {
                    //alveolar
                    temp.set(Feature::Labial, false);
                    temp.set(Feature::Coronal, true);
                    temp.set(Feature::Anterior, true);
                    temp.set(Feature::Distributed, false);
                    temp.set(Feature::Dorsal, false);
                    temp.set(Feature::Pharyngeal, false);
                }
                4 => {
                    //postalveolar
                    temp.set(Feature::Labial, false);
                    temp.set(Feature::Coronal, true);
                    temp.set(Feature::Anterior, false);
                    temp.set(Feature::Distributed, true);
                    temp.set(Feature::Dorsal, false);
                    temp.set(Feature::Pharyngeal, false);
                }
                5 => {
                    //retroflex
                    temp.set(Feature::Labial, false);
                    temp.set(Feature::Coronal, true);
                    temp.set(Feature::Anterior, false);
                    temp.set(Feature::Distributed, false);
                    temp.set(Feature::Dorsal, false);
                    temp.set(Feature::Pharyngeal, false);
                }
                6 => {
                    //palatal
                    temp.set(Feature::Labial, false);
                    temp.set(Feature::Coronal, false);
                    temp.set(Feature::Anterior, false);
                    temp.set(Feature::Distributed, true);
                    temp.set(Feature::Dorsal, true);
                    temp.set(Feature::High, true);
                    temp.set(Feature::Low, false);
                    temp.set(Feature::Front, true);
                    temp.set(Feature::Back, false);
                    temp.set(Feature::Pharyngeal, false);
                }
                7 => {
                    //velar
                    temp.set(Feature::Labial, false);
                    temp.set(Feature::Coronal, false);
                    //temp.set(AltFeature::Anterior   , false);
                    //temp.set(AltFeature::Distributed, false);
                    temp.set(Feature::Dorsal, true);
                    temp.set(Feature::High, true);
                    temp.set(Feature::Low, false);
                    //temp.set(AltFeature::Front      , false);
                    temp.set(Feature::Back, true);
                    temp.set(Feature::Pharyngeal, false);
                }
                8 => {
                    //uvular
                    temp.set(Feature::Labial, false);
                    temp.set(Feature::Coronal, false);
                    //temp.set(AltFeature::Anterior   , false);
                    //temp.set(AltFeature::Distributed, false);
                    temp.set(Feature::Dorsal, true);
                    temp.set(Feature::High, false);
                    temp.set(Feature::Low, true);
                    //temp.set(AltFeature::Front      , false);
                    temp.set(Feature::Back, true);
                    temp.set(Feature::Pharyngeal, false);
                }
                9 => {
                    //Pharyngeal
                    temp.set(Feature::Labial, false);
                    temp.set(Feature::Coronal, false);
                    //temp.set(AltFeature::Anterior   , false);
                    //temp.set(AltFeature::Distributed, false);
                    temp.set(Feature::Dorsal, false);
                    temp.set(Feature::Pharyngeal, true);
                }
                10 => {
                    //glottal
                    temp.set(Feature::Labial, false);
                    temp.set(Feature::Coronal, false);
                    //temp.set(AltFeature::Anterior   , false);
                    //temp.set(AltFeature::Distributed, false);
                    temp.set(Feature::Dorsal, false);
                    temp.set(Feature::Pharyngeal, false);
                }
                _ => {}
            }
        }

        1 => {
            temp.set(Feature::Syllabic, false);
            // temp.set(AltFeature::Consonental, true);
            temp.set(Feature::Sonorant, false);
            match manner {
                0 => {
                    //Ejectives
                    //temp.set(AltFeature::Approximant, false);
                    temp.set(Feature::Continuant, false);
                    temp.set(Feature::Nasal, false);
                    temp.set(Feature::Strident, false);
                    temp.set(Feature::Lateral, false);
                    temp.set(Feature::DelayedRelease, false);
                    temp.set(Feature::Voice, true);
                    temp.set(Feature::ConstrictedGlottis, true);
                }
                1 => {
                    temp = FeatureTable::new();
                    eprint!("ERROR: invalid or undefined token {temp:?}");
                    ////temp.set(AltFeature::Approximant, false);
                    //temp.set(AltFeature::Sonorant, false);
                    //temp.set(AltFeature::Continuant, false);
                    //temp.set(AltFeature::Nasal, false);
                    //temp.set(AltFeature::Strident =Some(false);
                    //temp.set(AltFeature::Lateral, false);
                    //temp.set(AltFeature::DelayedRelease, false);
                }
                _ => {}
            }

            match place {
                0 => {
                    temp.set(Feature::Labial, true);
                    temp.set(Feature::Coronal, false);
                    temp.set(Feature::Dorsal, false);
                    temp.set(Feature::Pharyngeal, false);
                }
                1 => {
                    temp.set(Feature::Labial, false);
                    temp.set(Feature::Coronal, true);
                    temp.set(Feature::Anterior, true);
                    temp.set(Feature::Distributed, false);
                    temp.set(Feature::Dorsal, false);
                    temp.set(Feature::Pharyngeal, false);
                }
                2 => {
                    temp.set(Feature::Labial, false);
                    temp.set(Feature::Coronal, false);
                    temp.set(Feature::Dorsal, true);
                    temp.set(Feature::High, true);
                    temp.set(Feature::Low, false);
                    temp.set(Feature::Front, false);
                    temp.set(Feature::Back, false);
                    temp.set(Feature::Pharyngeal, false);
                }
                3 => {
                    temp.set(Feature::Labial, false);
                    temp.set(Feature::Coronal, false);
                    temp.set(Feature::Dorsal, true);
                    temp.set(Feature::High, true);
                    temp.set(Feature::Low, false);
                    temp.set(Feature::Front, false);
                    temp.set(Feature::Back, true);
                    temp.set(Feature::Pharyngeal, false);
                }
                4 => {
                    temp.set(Feature::Labial, false);
                    temp.set(Feature::Coronal, false);
                    temp.set(Feature::Dorsal, true);
                    temp.set(Feature::High, false);
                    temp.set(Feature::Low, true);
                    temp.set(Feature::Front, false);
                    temp.set(Feature::Back, true);
                    temp.set(Feature::Pharyngeal, false);
                }
                _ => {}
            }
        }
        2 => {
            // VOWELS
            //temp.set(AltFeature::Sonorant, true);
            // temp.set(AltFeature::Consonental, false);
            temp.set(Feature::Sonorant, true);
            temp.set(Feature::Syllabic, true);
            ////temp.set(AltFeature::Approximant, true); //Can probably be removed entirely

            temp.set(Feature::Labial, true);
            // temp.set(Feature::Round, false);

            // temp.set(Feature::Coronal, false);

            temp.set(Feature::Dorsal, true);

            temp.set(Feature::Voice, true);
            // temp.set(Feature::SpreadGlottis, false);
            temp.set(Feature::Continuant, true);
            // temp.set(Feature::Nasal, false);
            // temp.set(Feature::Strident, false);
            // temp.set(Feature::Lateral, false);
            // temp.set(Feature::ConstrictedGlottis, false);
            temp.set(Feature::AdvancedTongueRoot, true);
            // temp.set(Feature::DelayedRelease, false);
            match manner {
                //Height
                0 => {
                    //Semivowel
                    temp.set(Feature::Syllabic, false);
                    temp.set(Feature::High, true);
                    temp.set(Feature::Low, false);
                    //temp.set(AltFeature::Tense, false);
                }
                1 => {
                    //close/High Tense vowel
                    temp.set(Feature::High, true);
                    temp.set(Feature::Low, false);
                    //temp.set(AltFeature::Tense, true);
                }
                2 => {
                    //near close/lax High vowel
                    temp.set(Feature::High, true);
                    temp.set(Feature::Low, false);
                    temp.set(Feature::AdvancedTongueRoot, false);
                }
                3 => {
                    //mid close/mid Tense vowel
                    temp.set(Feature::High, false);
                    temp.set(Feature::Low, false);
                    //temp.set(AltFeature::Tense, true);
                }
                4 => {
                    //mid vowel/central
                    temp.set(Feature::High, false);
                    temp.set(Feature::Low, false);
                    //temp.set(AltFeature::Tense, false);
                }
                5 => {
                    //mid open vowel/mid lax
                    temp.set(Feature::High, false);
                    temp.set(Feature::Low, false);
                    temp.set(Feature::AdvancedTongueRoot, false);
                    //temp.set(AltFeature::Tense, false);
                }
                6 => {
                    //near open vowel/Low lax
                    temp.set(Feature::High, false);
                    temp.set(Feature::Low, true);
                    temp.set(Feature::AdvancedTongueRoot, false);
                    //temp.set(AltFeature::Tense, false);
                }
                7 => {
                    //open vowwel/Low Tense
                    temp.set(Feature::High, false);
                    temp.set(Feature::Low, true);
                    //temp.set(AltFeature::Tense, true);
                }
                _ => {}
            }

            temp.set(Feature::Round, place % 2 == 1);

            match place / 2 {
                0 => {
                    temp.set(Feature::Front, true);
                    temp.set(Feature::Back, false);
                }
                1 => {
                    temp.set(Feature::Front, false);
                    temp.set(Feature::Back, false);
                }
                2 => {
                    temp.set(Feature::Front, false);
                    temp.set(Feature::Back, true);
                }
                _ => {}
            }
        }
        _ => {}
    }
    temp //.clone()
}
