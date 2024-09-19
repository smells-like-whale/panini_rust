use super::Feature;
use crate::{util, xsampa};

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum AltFeature {
    //Class
    Consonental,
    Sonorant,
    Syllabic,
    //Place
    Labial,
    Round,

    Coronal,
    Anterior,
    Distributed,

    Dorsal,
    High,
    Low,
    Front,
    Back,
    //Laryngeal
    Pharyngeal,
    AdvancedTongueRoot,
    Voice,
    SpreadGlottis,
    ConstrictedGlottis,
    //Manner
    Continuant,
    Nasal,
    Strident,
    Lateral,
    DelayedRelease,
    Long,
} //25 values

const ALT_FEATURES: [AltFeature; 24] = [
    AltFeature::Consonental,
    AltFeature::Sonorant,
    AltFeature::Syllabic,
    AltFeature::Labial,
    AltFeature::Round,
    AltFeature::Coronal,
    AltFeature::Anterior,
    AltFeature::Distributed,
    AltFeature::Dorsal,
    AltFeature::High,
    AltFeature::Low,
    AltFeature::Front,
    AltFeature::Back,
    AltFeature::Pharyngeal,
    AltFeature::AdvancedTongueRoot,
    AltFeature::Voice,
    AltFeature::SpreadGlottis,
    AltFeature::ConstrictedGlottis,
    AltFeature::Continuant,
    AltFeature::Strident,
    AltFeature::Lateral,
    AltFeature::DelayedRelease,
    AltFeature::Nasal,
    AltFeature::Long,
]; //25 values

impl AltFeature {
    pub fn to_string(self) -> &'static str {
        match self {
            AltFeature::Consonental => "Consonental",
            AltFeature::Sonorant => "Sonorant",
            AltFeature::Syllabic => "Syllabic",

            AltFeature::Labial => "Labial",
            AltFeature::Round => "Round",

            AltFeature::Coronal => "Coronal",
            AltFeature::Anterior => "Anterior",
            AltFeature::Distributed => "Distributed",

            AltFeature::Dorsal => "Dorsal",
            AltFeature::High => "High",
            AltFeature::Low => "Low",
            AltFeature::Front => "Front",
            AltFeature::Back => "Back",

            AltFeature::Pharyngeal => "Pharyngeal",
            AltFeature::AdvancedTongueRoot => "AdvancedTongueRoot",

            AltFeature::Voice => "Voice",
            AltFeature::SpreadGlottis => "SpreadGlottis",
            AltFeature::ConstrictedGlottis => "ConstrictedGlottis",

            AltFeature::Continuant => "Continuant",
            AltFeature::Strident => "Strident",
            AltFeature::Lateral => "Lateral",
            AltFeature::DelayedRelease => "DelayedRelease",
            AltFeature::Nasal => "Nasal",
            AltFeature::Long => "Long",
        }
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Labial {
    round: bool,
}
impl Labial {
    pub const fn new() -> Self {
        Self { round: false }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Coronal {
    anterior: bool,
    distributed: bool,
}
impl Coronal {
    pub const fn new() -> Self {
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
    pub const fn new() -> Self {
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
    pub const fn new() -> Self {
        Self { atr: false }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct AltFeatureBundle {
    //there are 21 values
    //CLASS FEATURES
    consonental: bool,
    sonorant: bool,
    syllabic: bool,
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

impl AltFeatureBundle {
    pub const fn new() -> Self {
        Self {
            consonental: false,
            sonorant: false,
            syllabic: false,

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

    pub fn to_string(&self) -> String {
        let mut str = String::new();

        for fet in ALT_FEATURES {
            let b = self.get_opt(fet);

            match b {
                Some(true) => {
                    str.push('+');
                    str.push_str(&fet.to_string());
                    str.push('\n');
                }
                Some(false) => {
                    str.push('-');
                    str.push_str(&fet.to_string());
                    str.push('\n');
                        }
                None => {
                    str.push('∅');
                    str.push('\n');

                },
            }
        }
        str
    }
   
    pub const fn get(&self, fet: AltFeature) -> bool {
        match self.get_opt(fet) {
            Some(b) => b,
            None => false,
        }
}

    const fn get_opt(&self, fet: AltFeature) -> Option<bool> {
        match fet {
            AltFeature::Consonental => Some(self.consonental),
            AltFeature::Sonorant => Some(self.sonorant),
            AltFeature::Syllabic => Some(self.syllabic),

            AltFeature::Labial => Some(self.labial.is_some()),
            AltFeature::Round => {
                    match self.labial {
                        Some(val) => Some(val.round),
                        None => None,
                }
            }
            AltFeature::Coronal => Some(self.coronal.is_some()),
            AltFeature::Anterior => {
                    match self.coronal {
                        Some(val) => Some(val.anterior),
                        None => None,
                    }
            }
            AltFeature::Distributed => {
                    match self.coronal {
                        Some(val) => Some(val.distributed),
                        None => None,
                    }
            }
            AltFeature::Dorsal => Some(self.dorsal.is_some()),
            AltFeature::Front => {
                    match self.dorsal {
                        Some(val) => Some(val.front),
                        None => None,
                    }
            }
            AltFeature::Back => {
                    match self.dorsal {
                        Some(val) => Some(val.back),
                        None => None,
                    }
            }
            AltFeature::High => {
                    match self.dorsal {
                        Some(val) => Some(val.high),
                        None => None,
                    }
            }
            AltFeature::Low => {
                    match self.dorsal {
                        Some(val) => Some(val.low),
                        None => None,
                    }
            }
            AltFeature::Pharyngeal => Some(self.pharyngeal.is_some()),

            AltFeature::AdvancedTongueRoot => {
                    let this = self.pharyngeal;
                    match this {
                        Some(val) => Some(val.atr),
                        None => Some(false),
                    }
            }

            AltFeature::Voice => Some(self.voice),
            AltFeature::SpreadGlottis => Some(self.spread_glottis),
            AltFeature::ConstrictedGlottis => Some(self.constricted_glottis),

            AltFeature::Continuant => Some(self.continuant),
            AltFeature::Nasal => Some(self.nasal),
            AltFeature::Strident => Some(self.strident),
            AltFeature::Lateral => Some(self.lateral),
            AltFeature::DelayedRelease => Some(self.delayed_release),
            AltFeature::Long => Some(self.long),
        }
    }


    pub fn set(&mut self, fet: AltFeature, b: bool) -> Self {
        match fet {
            AltFeature::Consonental => {
                self.consonental = b;
            }
            AltFeature::Sonorant => {
                self.sonorant = b;
            }
            AltFeature::Syllabic => {
                self.syllabic = b;
            }
            AltFeature::Labial => {
                if b {
                    self.labial = match self.labial {
                        Some(x) => Some(x),
                        None => Some(Labial::new()),
                    }
                } else {
                    self.labial = None;
                }
            }
            AltFeature::Round => match self.labial {
                Some(mut x) => x.round = b,
                None => {
                    self.labial = if b {
                        Some(Labial { round: true })
                    } else {
                        None
                    }
                }
            },
            AltFeature::Coronal => {
                if b {
                    self.coronal = match self.coronal {
                        Some(x) => Some(x),
                        None => Some(Coronal::new()),
                    }
                } else {
                    self.coronal = None;
                }
            }
            AltFeature::Anterior => match self.coronal {
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
            AltFeature::Distributed => match self.coronal {
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
            AltFeature::Dorsal => {
                if b {
                    self.dorsal = match self.dorsal {
                        Some(x) => Some(x),
                        None => Some(Dorsal::new()),
                    }
                } else {
                    self.dorsal = None;
                }
            }
            AltFeature::Front => match self.dorsal {
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
            AltFeature::Back => match self.dorsal {
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
            AltFeature::High => match self.dorsal {
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
            AltFeature::Low => match self.dorsal {
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
            AltFeature::Pharyngeal => {
                if b {
                    self.pharyngeal = match self.pharyngeal {
                        Some(x) => Some(x),
                        None => Some(Pharyngeal::new()),
                    }
                } else {
                    self.pharyngeal = None;
                }
            }
            AltFeature::AdvancedTongueRoot => match self.pharyngeal {
                Some(mut x) => x.atr = b,
                None => {
                    self.pharyngeal = if b {
                        Some(Pharyngeal { atr: true })
                    } else {
                        None
                    }
                }
            },
            AltFeature::Voice => {
                self.voice = b;
            }
            AltFeature::SpreadGlottis => {
                self.spread_glottis = b;
            }
            AltFeature::ConstrictedGlottis => {
                self.constricted_glottis = b;
            }

            AltFeature::Continuant => {
                self.continuant = b;
            }
            AltFeature::Nasal => {
                self.nasal = b;
            }
            AltFeature::Strident => {
                self.strident = b;
            }
            AltFeature::Lateral => {
                self.lateral = b;
            }
            AltFeature::DelayedRelease => {
                self.delayed_release = b;
            }
            AltFeature::Long => {
                self.long = b;
            }
        }
        *self
    }


    pub fn from_token(_s: &str) -> AltFeatureBundle {
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
        let mut m: u8 = 0;
        let mut p: u8 = 0;
        let mut c: u8 = 0;
        

        let mut fet = AltFeatureBundle::new();

        for list in CONSONENT_CHART {
            for token in list {
                if init == token {
                    fet = from_token_intermediate(c, m, p);
                }
                p += 1;
            }
            p = 0;
            m += 1;
        }
        c += 1;
        m = 0;
        p = 0;

        for list in NON_PULMONIC_CONSONANT_CHART {
            for token in list {
                if init == token {
                    fet = from_token_intermediate(c, m, p);
                }
                p += 1;
            }
        }
        c += 1;
        m = 0;
        p = 0;

        for list in VOWEL_CHART {
            for token in list {
                if init == token {
                    fet = from_token_intermediate(c, m, p);
                }
                p += 1;
            }
            p = 0;
            m += 1;
        }
        if fet == AltFeatureBundle::new() {
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

fn from_token_intermediate(c: u8, m: u8, p: u8) -> AltFeatureBundle {
    let mut temp: AltFeatureBundle = AltFeatureBundle::new();
    match c {
        0 =>{  
            temp.set(AltFeature::Consonental, true);
            //temp.set(AltFeature::Syllabic, false);
            //temp.set(AltFeature::SpreadGlottis, false);
            //temp.set(AltFeature::ConstrictedGlottis, false);
            temp.set(AltFeature::Voice, p % 2 != 0);
            match m {
                0 => {
                    //plosive
                    //temp.set(AltFeature::Approximant, false);
                    //temp.set(AltFeature::Sonorant, false);
                    //temp.set(AltFeature::Continuant, false);
                    //temp.set(AltFeature::Nasal, false);
                    //temp.set(AltFeature::Strident, false);
                    //temp.set(AltFeature::Lateral, false);
                    //temp.set(AltFeature::DelayedRelease, false);
                }
                1 => {
                    //fricative
                    //temp.set(AltFeature::Approximant, false);
                    //temp.set(AltFeature::Sonorant, false);
                    temp.set(AltFeature::Continuant, true);
                    //temp.set(AltFeature::Nasal, false);
                    //temp.set(AltFeature::Strident, false);
                    //temp.set(AltFeature::Lateral, false);
                    //temp.set(AltFeature::DelayedRelease, false);
                }
                2 => {
                    //Strident Fricative
                    //temp.set(AltFeature::Approximant, false);
                    //temp.set(AltFeature::Sonorant, false);
                    temp.set(AltFeature::Continuant, true);
                    //temp.set(AltFeature::Nasal, false);
                    temp.set(AltFeature::Strident, true);
                    //temp.set(AltFeature::Lateral, false);
                    //temp.set(AltFeature::DelayedRelease, false);
                }
                3 => {
                    //nasal
                    //temp.set(AltFeature::Approximant, false);
                    temp.set(AltFeature::Sonorant, true);
                    //temp.set(AltFeature::Continuant, false);
                    temp.set(AltFeature::Nasal, true);
                    //temp.set(AltFeature::Strident, false);
                    //temp.set(AltFeature::Lateral, false);
                    //temp.set(AltFeature::DelayedRelease, false);
                }
                4 => {
                    //Lateral fricatives
                    //temp.set(AltFeature::Approximant, false);
                    //temp.set(AltFeature::Sonorant, true);
                    temp.set(AltFeature::Continuant, true);
                    //temp.set(AltFeature::Nasal, true);
                    //temp.set(AltFeature::Strident, false);
                    temp.set(AltFeature::Lateral, true);
                    //temp.set(AltFeature::DelayedRelease, false);
                }
                5 => {
                    //trills
                    //temp.set(AltFeature::Approximant, false);
                    temp.set(AltFeature::Sonorant, true);
                    temp.set(AltFeature::Continuant, true);
                    //temp.set(AltFeature::Nasal, true);
                    //temp.set(AltFeature::Strident, false);
                    //temp.set(AltFeature::Lateral, false);
                    //temp.set(AltFeature::DelayedRelease, false);
                }
                6 => {
                    //approximates
                    //temp.set(AltFeature::Approximant, false);
                    temp.set(AltFeature::Sonorant, true);
                    temp.set(AltFeature::Continuant, true);
                    temp.set(AltFeature::Nasal, true);
                    temp.set(AltFeature::Strident, false);
                    temp.set(AltFeature::Lateral, false);
                    temp.set(AltFeature::DelayedRelease, false);
                }
                7 => {
                    //taps
                    //temp.set(AltFeature::Approximant, false);
                    temp.set(AltFeature::Sonorant, true);
                    temp.set(AltFeature::Continuant, true);
                    temp.set(AltFeature::Nasal, false);
                    temp.set(AltFeature::Strident, false);
                    temp.set(AltFeature::Lateral, false);
                    temp.set(AltFeature::DelayedRelease, false);
                }
                8 => {
                    //Lateral
                    //temp.set(AltFeature::Approximant, true);
                    temp.set(AltFeature::Sonorant, true);
                    temp.set(AltFeature::Continuant, true);
                    temp.set(AltFeature::Nasal, false);
                    temp.set(AltFeature::Strident, false);
                    temp.set(AltFeature::Lateral, true);
                    temp.set(AltFeature::DelayedRelease, false);
                }
                _ => {}
            }

            match p / 2 {
                0 => {
                    //biLabial
                    temp.set(AltFeature::Labial, true);
                    temp.set(AltFeature::Coronal, false);
                    //temp.set(AltFeature::Anterior   , false);
                    //temp.set(AltFeature::Distributed, false);
                    temp.set(AltFeature::Dorsal, false);
                    temp.set(AltFeature::Pharyngeal, false);
                }
                1 => {
                    //labiodental
                    temp.set(AltFeature::Labial, true);
                    temp.set(AltFeature::Coronal, false);
                    //temp.set(AltFeature::Anterior   , false);
                    //temp.set(AltFeature::Distributed, false);
                    temp.set(AltFeature::Dorsal, false);
                    temp.set(AltFeature::Pharyngeal, false);
                }
                2 => {
                    //dental
                    temp.set(AltFeature::Labial, false);
                    temp.set(AltFeature::Coronal, true);
                    temp.set(AltFeature::Anterior, true);
                    temp.set(AltFeature::Distributed, true);
                    temp.set(AltFeature::Dorsal, false);
                    temp.set(AltFeature::Pharyngeal, false);
                }
                3 => {
                    //alveolar
                    temp.set(AltFeature::Labial, false);
                    temp.set(AltFeature::Coronal, true);
                    temp.set(AltFeature::Anterior, true);
                    temp.set(AltFeature::Distributed, false);
                    temp.set(AltFeature::Dorsal, false);
                    temp.set(AltFeature::Pharyngeal, false);
                }
                4 => {
                    //postalveolar
                    temp.set(AltFeature::Labial, false);
                    temp.set(AltFeature::Coronal, true);
                    temp.set(AltFeature::Anterior, false);
                    temp.set(AltFeature::Distributed, true);
                    temp.set(AltFeature::Dorsal, false);
                    temp.set(AltFeature::Pharyngeal, false);
                }
                5 => {
                    //retroflex
                    temp.set(AltFeature::Labial, false);
                    temp.set(AltFeature::Coronal, true);
                    temp.set(AltFeature::Anterior, false);
                    temp.set(AltFeature::Distributed, false);
                    temp.set(AltFeature::Dorsal, false);
                    temp.set(AltFeature::Pharyngeal, false);
                }
                6 => {
                    //palatal
                    temp.set(AltFeature::Labial, false);
                    temp.set(AltFeature::Coronal, false);
                    //temp.set(AltFeature::Anterior   , false);
                    //temp.set(AltFeature::Distributed, false);
                    temp.set(AltFeature::Dorsal, true);
                    temp.set(AltFeature::High, true);
                    temp.set(AltFeature::Low, false);
                    temp.set(AltFeature::Front, true);
                    temp.set(AltFeature::Back, false);
                    temp.set(AltFeature::Pharyngeal, false);
                }
                7 => {
                    //velar
                    temp.set(AltFeature::Labial, false);
                    temp.set(AltFeature::Coronal, false);
                    //temp.set(AltFeature::Anterior   , false);
                    //temp.set(AltFeature::Distributed, false);
                    temp.set(AltFeature::Dorsal, true);
                    temp.set(AltFeature::High, true);
                    temp.set(AltFeature::Low, false);
                    //temp.set(AltFeature::Front      , false);
                    temp.set(AltFeature::Back, true);
                    temp.set(AltFeature::Pharyngeal, false);
                }
                8 => {
                    //uvular
                    temp.set(AltFeature::Labial, false);
                    temp.set(AltFeature::Coronal, false);
                    //temp.set(AltFeature::Anterior   , false);
                    //temp.set(AltFeature::Distributed, false);
                    temp.set(AltFeature::Dorsal, true);
                    temp.set(AltFeature::High, false);
                    temp.set(AltFeature::Low, true);
                    //temp.set(AltFeature::Front      , false);
                    temp.set(AltFeature::Back, true);
                    temp.set(AltFeature::Pharyngeal, false);
                }
                9 => {
                    //Pharyngeal
                    temp.set(AltFeature::Labial, false);
                    temp.set(AltFeature::Coronal, false);
                    //temp.set(AltFeature::Anterior   , false);
                    //temp.set(AltFeature::Distributed, false);
                    temp.set(AltFeature::Dorsal, false);
                    temp.set(AltFeature::Pharyngeal, true);
                }
                10 => {
                    //glottal
                    temp.set(AltFeature::Labial, false);
                    temp.set(AltFeature::Coronal, false);
                    //temp.set(AltFeature::Anterior   , false);
                    //temp.set(AltFeature::Distributed, false);
                    temp.set(AltFeature::Dorsal, false);
                    temp.set(AltFeature::Pharyngeal, false);
                }
                _ => {}
            }
        }

        1 => {
            temp.set(AltFeature::Syllabic, false);
            temp.set(AltFeature::Consonental, true);
            temp.set(AltFeature::Sonorant, false);
            match m {
                0 => {
                    //temp.set(AltFeature::Approximant, false);
                    temp.set(AltFeature::Continuant, false);
                    temp.set(AltFeature::Nasal, false);
                    temp.set(AltFeature::Strident, false);
                    temp.set(AltFeature::Lateral, false);
                    temp.set(AltFeature::DelayedRelease, false);
                    temp.set(AltFeature::Voice, true);
                    temp.set(AltFeature::ConstrictedGlottis, true);
                }
                1 => {
                    temp = AltFeatureBundle::new();
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

            match p {
                0 => {
                    temp.set(AltFeature::Labial, true);
                    temp.set(AltFeature::Coronal, false);
                    temp.set(AltFeature::Dorsal, false);
                    temp.set(AltFeature::Pharyngeal, false);
                }
                1 => {
                    temp.set(AltFeature::Labial, false);
                    temp.set(AltFeature::Coronal, true);
                    temp.set(AltFeature::Anterior, true);
                    temp.set(AltFeature::Distributed, false);
                    temp.set(AltFeature::Dorsal, false);
                    temp.set(AltFeature::Pharyngeal, false);
                }
                2 => {
                    temp.set(AltFeature::Labial, false);
                    temp.set(AltFeature::Coronal, false);
                    temp.set(AltFeature::Dorsal, true);
                    temp.set(AltFeature::High, true);
                    temp.set(AltFeature::Low, false);
                    temp.set(AltFeature::Front, false);
                    temp.set(AltFeature::Back, false);
                    temp.set(AltFeature::Pharyngeal, false);
                }
                3 => {
                    temp.set(AltFeature::Labial, false);
                    temp.set(AltFeature::Coronal, false);
                    temp.set(AltFeature::Dorsal, true);
                    temp.set(AltFeature::High, true);
                    temp.set(AltFeature::Low, false);
                    temp.set(AltFeature::Front, false);
                    temp.set(AltFeature::Back, true);
                    temp.set(AltFeature::Pharyngeal, false);
                }
                4 => {
                    temp.set(AltFeature::Labial, false);
                    temp.set(AltFeature::Coronal, false);
                    temp.set(AltFeature::Dorsal, true);
                    temp.set(AltFeature::High, false);
                    temp.set(AltFeature::Low, true);
                    temp.set(AltFeature::Front, false);
                    temp.set(AltFeature::Back, true);
                    temp.set(AltFeature::Pharyngeal, false);
                }
                _ => {}
            }
        }
        2 => {// VOWELS
            //temp.set(AltFeature::Sonorant, true);
            temp.set(AltFeature::Consonental, false);
            temp.set(AltFeature::Sonorant, true);
            temp.set(AltFeature::Syllabic, true);
            ////temp.set(AltFeature::Approximant, true); //Can probably be removed entirely

            temp.set(AltFeature::Labial, true);
            temp.set(AltFeature::Round, false);

            temp.set(AltFeature::Coronal, false);

            temp.set(AltFeature::Dorsal, true);



            temp.set(AltFeature::Voice, true);
            temp.set(AltFeature::SpreadGlottis, false);
            temp.set(AltFeature::Continuant, true);
            temp.set(AltFeature::Nasal, false);
            temp.set(AltFeature::Strident, false);
            temp.set(AltFeature::Lateral, false);
            temp.set(AltFeature::ConstrictedGlottis, false);
            temp.set(AltFeature::DelayedRelease, false);
            match m {
                //Height
                0 => {
                    //Semivowel
                    temp.set(AltFeature::Syllabic, false);
                    temp.set(AltFeature::High, true);
                    temp.set(AltFeature::Low, false);
                    //temp.set(AltFeature::Tense, false);
                }
                1 => {
                    //close/High Tense vowel
                    temp.set(AltFeature::High, true);
                    temp.set(AltFeature::Low, false);
                    //temp.set(AltFeature::Tense, true);
                }
                2 => {
                    //near close/lax High vowel
                    temp.set(AltFeature::High, true);
                    temp.set(AltFeature::Low, false);
                    //temp.set(AltFeature::Tense, false);
                }
                3 => {
                    //mid close/mid Tense vowel
                    temp.set(AltFeature::High, false);
                    temp.set(AltFeature::Low, false);
                    //temp.set(AltFeature::Tense, true);
                }
                4 => {
                    //mid vowel/central
                    temp.set(AltFeature::High, false);
                    temp.set(AltFeature::Low, false);
                    //temp.set(AltFeature::Tense, false);
                }
                5 => {
                    //mid open vowel/mid lax
                    temp.set(AltFeature::High, false);
                    temp.set(AltFeature::Low, false);
                    //temp.set(AltFeature::Tense, false);
                }
                6 => {
                    //near open vowel/Low lax
                    temp.set(AltFeature::High, false);
                    temp.set(AltFeature::Low, true);
                    //temp.set(AltFeature::Tense, false);
                }
                7 => {
                    //open vowwel/Low Tense
                    temp.set(AltFeature::High, false);
                    temp.set(AltFeature::Low, true);
                    //temp.set(AltFeature::Tense, true);
                }
                _ => {}
            }

            temp.set(AltFeature::Round, p % 2 == 1);

            match p / 2 {
                0 => {
                    temp.set(AltFeature::Front, true);
                    temp.set(AltFeature::Back, false);
                }
                1 => {
                    temp.set(AltFeature::Front, false);
                    temp.set(AltFeature::Back, false);
                }
                2 => {
                    temp.set(AltFeature::Front, false);
                    temp.set(AltFeature::Back, true);
                }
                _ => {}
            }
        }
        _ => {}
    }
    temp //.clone()
}

static CONSONENT_CHART: [[&str; 22]; 9] = [
    [
        "p", "b", "", "", "", "", "t", "d", "", "", "t`", "d`", "c", "J\\", "k", "g", "q", "G\\",
        "", "", "?", "",
    ],
    [
        "p\\", "B", "f", "v", "T", "D", "", "", "", "", "", "", "C", "j\\", "x", "G", "X", "R\\",
        "X\\", "?\\", "h", "h\\",
    ],
    [
        "", "", "", "", "", "", "s", "z", "S", "Z", "s`", "z`", "s\\", "z\\", "", "", "", "", "",
        "", "", "",
    ],
    [
        "", "m", "", "F", "", "", "", "n", "", "", "", "n`", "", "J", "", "N", "", "N\\", "", "",
        "", "",
    ],
    [
        "", "", "", "", "", "", "K", "K\\", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
    ],
    [
        "", "B\\", "", "", "", "", "", "r", "", "", "", "", "", "", "", "", "", "R", "", "", "", "",
    ],
    [
        "", "", "", "v\\", "", "", "", "r\\", "", "", "", "r\\`", "", "", "", "", "", "", "", "",
        "", "",
    ],
    [
        "", "", "", "", "", "", "", "4", "", "", "", "r`", "", "", "", "", "", "", "", "", "", "",
    ],
    [
        "", "", "", "", "", "", "", "l", "", "", "", "l`", "", "L", "", "L\\", "", "", "", "", "",
        "",
    ],
];

static NON_PULMONIC_CONSONANT_CHART: [[&str; 5]; 2] = [
    //exludes ejectives for reasons
    ["b_<", "d_<", "f_<", "g_<", "G_<"],
    ["O\\", "|\\", "!\\", "=\\", "|\\|\\"],// |\\|\\ is not being parsed properly
];
//Front     central     "back"
//needs 3 features to describe height???
pub static VOWEL_CHART: [[&str; 6]; 8] = [
    ["j", "H", "", "", "M\\", "w"],
    ["i", "y", "1", "}", "M", "u"], //+high +tense -mid -low OR +raised -retracted
    ["I", "Y", "", "", "", "U"],    //                       OR +raised +retracted
    ["e", "2", "@\\", "8", "7", "o"], //+high +tense +mid -low OR -raised -retracted
    ["", "", "@", "", "", ""],      //-high -tense +mid -low OR -raised
    ["E", "9", "3", "3\\", "V", "O"], //-high -tense +mid +low OR -raised +retracted
    ["{", "", "6", "", "", ""],     //-high -tense -mid +low OR -raised +retracted
    ["a", "&", "a_\"", "", "A", "Q"], //-high +tense -mid +low OR -raised +retracted
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
