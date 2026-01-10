use super::feature::{feature_table::FeatureTable, Feature, FeatureBundle, FeatureIndex};

enum FeatureBundleModif {
    FeatureBundle(FeatureTable),
    Assimilate(bool, bool, FeatureTable, FeatureTable), //Direction, Distance, AssimilationFeatures, BlockedBy,
    Dissimilate(),
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SegModifier<F: FeatureIndex> {
    pub token: String,
    pub modif: Vec<(F, bool)>, //This is modified during initialisation, but could be hardcoded
}

impl<F: FeatureIndex> SegModifier<F> {
    // pub fn new(token: &'a str, fet: Feature, b: bool) -> Self {
    //
    //     SegModif {
    //         token,
    //         modif: &[(fet, b)],
    //     }
    // }

    pub fn default() -> Self {
        SegModifier {
            token: String::new(),
            modif: Vec::new(),
        }
    }

    pub fn from_string(mod_token: &str) -> Option<Self> {
        for m in Self::modifiers() {
            if mod_token == m.token {
                return Some(m.clone());
            }
        }
        None
    }

    pub fn modifiers() -> Vec<SegModifier<F>> {
        vec![
            SegModifier {
                token: "".to_string(),
                modif: Vec::new(),
            },
            SegModifier {
                token: "_>".to_string(),
                modif: vec![(F::from_feature(Feature::ConstrictedGlottis), true)],
            },
            SegModifier {
                token: "_h".to_string(),
                modif: vec![(F::from_feature(Feature::SpreadGlottis), true)],
            },
            SegModifier {
                token: "_=".to_string(),
                modif: vec![(F::from_feature(Feature::Syllabic), true)],
            },
            SegModifier {
                token: "_^".to_string(),
                modif: vec![(F::from_feature(Feature::Syllabic), false)],
            },
            SegModifier {
                token: "_0".to_string(),
                modif: vec![(F::from_feature(Feature::Voice), false)],
            },
            SegModifier {
                token: "_~".to_string(),
                modif: vec![(F::from_feature(Feature::Nasal), true)],
            },
            SegModifier {
                token: "_d".to_string(),
                modif: vec![
                    (F::from_feature(Feature::Distributed), true),
                    (F::from_feature(Feature::Anterior), true),
                ],
            },
            SegModifier {
                token: "_-".to_string(),
                modif: vec![(F::from_feature(Feature::Distributed), false)],
            },
            SegModifier {
                token: "_w".to_string(),
                modif: vec![
                    // (F::from_feature(Feature::Labial), true),
                    (F::from_feature(Feature::Round), true),
                    // (F::from_feature(Feature::Dorsal), true),
                ],
            },
            SegModifier {
                token: "_j".to_string(),
                modif: vec![
                    (F::from_feature(Feature::Coronal), true),
                    // (F::from_feature(Feature::Dorsal), true),
                    (F::from_feature(Feature::High), true),
                ],
            },
            SegModifier {
                token: "_G".to_string(),
                modif: vec![
                    (F::from_feature(Feature::Dorsal), true),
                    (F::from_feature(Feature::High), true),
                    (F::from_feature(Feature::Back), true),
                ],
            },
            SegModifier {
                token: "_?\\".to_string(),
                modif: vec![(F::from_feature(Feature::Pharyngeal), true)],
            },
            SegModifier {
                token: ":".to_string(),
                modif: vec![(F::from_feature(Feature::Length), true)],
            },
        ]
    }
}

pub fn trans_mod<'a, F: FeatureIndex>(sm: SegModifier<Feature>) -> SegModifier<F> {
    let sm_c = sm.clone();
    let mut mods = Vec::new();

    SegModifier {
        token: sm_c.token,
        modif: mods,
    }
}

// const MODIFIERS: &[SegModifier<Feature>] = &[
//     SegModifier {
//         token: "".to_string(),
//         modif: Vec::new(),
//     },
//     SegModifier {
//         token: "_>".to_string(),
//         modif: vec![(Feature::ConstrictedGlottis, true)],
//     },
//     SegModifier {
//         token: "_h".to_string(),
//         modif: vec![(Feature::SpreadGlottis, true)],
//     },
//     SegModifier {
//         token: "_=".to_string(),
//         modif: vec![(Feature::Syllabic, true)],
//     },
//     SegModifier {
//         token: "_~".to_string(),
//         modif: vec![(Feature::Nasal, true)],
//     },
//     // SegModif {
//     //     token: "_a".to_string(),
//     //     modif: FeatureTable::new().set_and_return(Feature::Syllabic, true),
//     // },
//     // SegModif {
//     //     token: "_t".to_string(),
//     //     modif: FeatureTable::new().set_and_return(Feature::Nasal, true),
//     // },
//     SegModifier {
//         token: "_d".to_string(),
//         modif: vec![
//             (Feature::Distributed, true),
//             (Feature::Anterior, true),
//         ],
//     },
//     SegModifier {
//         token: "_w".to_string(),
//         modif: vec![
//             (Feature::Labial, true),
//             (Feature::Round, true),
//             (Feature::Dorsal, true),
//         ]
//     },
//     SegModifier {
//         token: "_j".to_string(),
//         modif: vec![
//             (Feature::Coronal, true),
//             (Feature::Dorsal, true),
//             (Feature::High, true),
//         ]
//     },
//     SegModifier {
//         token: "_G".to_string(),
//         modif: vec![
//             (Feature::Dorsal, true),
//             (Feature::High, true),
//             (Feature::Back, true),
//         ]
//     },
//     SegModifier {
//         token: "_?\\".to_string(),
//         modif: vec![(Feature::Pharyngeal, true)]
//     },
//     SegModifier {
//         token: ":".to_string(),
//         modif: vec![(Feature::Length, true)]
//     },
// ];
//
