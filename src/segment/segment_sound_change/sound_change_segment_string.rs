use crate::segment::{FeatureBundle, TokenString, FEATURES};

pub enum SCSegment {
    TokenSeg(TokenString),
    FeatureSeg(FeatureBundle),
}

pub struct SCString {
    segments: Vec<SCSegment>,
}

// impl SCString {
//
//     fn matches_feature(&self, fet_repr: FeatureBundle) -> bool {
//         let mut b = true;
//         
//         for fet in FEATURES {
//             match fet.get(f) {
//                 None => {}
//                 Some(bo) => {
//                     b = bo == self.features.get(f).unwrap_or(false);
//                 }
//             }
//         }
//         b
//     }
// }
