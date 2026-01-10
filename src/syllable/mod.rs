use crate::segment::{feature::{feature_shape::FeatureGeometryBundle, FeatureBundle}, SegmentShell};

pub trait Syllabic<F: FeatureBundle> {
    fn get(i: u64) -> Option<SegmentShell<F>>; 
    fn get_onset() -> Vec<FeatureGeometryBundle>;
    fn get_rhyme() -> Vec<FeatureGeometryBundle>;
    fn get_nucleus() -> Vec<FeatureGeometryBundle>;
    fn get_coda() -> Vec<FeatureGeometryBundle>;

}


pub struct Syllable {
    onset: Vec<FeatureGeometryBundle>,
    rime: Rime,
    sup: SupraSegmental,
}

struct Rime {
    nucleus: Vec<FeatureGeometryBundle>,
    coda: Vec<FeatureGeometryBundle>,
}

struct SupraSegmental {
    stress: bool,
    tone: (u8, u8, u8),
}
