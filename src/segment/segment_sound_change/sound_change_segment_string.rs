use crate::segment::{segment_string::SegmentString, FeatureBundle, TokenString, FEATURES};

use super::{PhonObject, SoundChange};



impl PhonObject for SegmentString {
    fn apply(&self, sc: &SoundChange) -> Self {
        let from = &sc.from;
        let to   = &sc.to;

        let mut post = self.clone();
        let mut pre = SegmentString::new();

        loop {//Medial Phase
            let split = post.split_on_first(from);
            match split {
                Some((prefix, pivot, tail)) => {
                    pre.append(&prefix);
                    post = tail;
                    
                    // println!("{:?}", (&pre.to_string(), &pivot.to_string(), &post.to_string()));
                    
                    if pre.matches_backwards(&sc.before) && post.matches_forward(&sc.after) {
                        pre.append(&pivot.apply_scstring(to));
                    } else {
                        pre.append(&pivot);
                    }
                }
                None => {
                    pre.append(&post);
                    break;
                }
            }
        }
        pre
    }

    fn apply_mut(&mut self, sc: &SoundChange) {
        *self = self.apply(sc);
    }
}
