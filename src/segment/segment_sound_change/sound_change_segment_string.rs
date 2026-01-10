use crate::segment::{
    feature::FeatureBundle, segment_string::SegmentStringShell, SegModif, SegmentShell, TokenString,
};

use super::{PhonObject, SoundChangeShell};

impl<F: FeatureBundle> PhonObject<F> for SegmentStringShell<F> {
    fn apply(&self, sc: &SoundChangeShell<F>) -> Self {
        let from = &sc.from;
        let to = &sc.to;

        let mut post = self.clone();
        let mut pre = SegmentStringShell::new();

        loop {
            //Medial Phase
            let split = post.split_on_first(from);
            match split {
                //For successful splits, apply the relevant sound change
                Some((prefix, pivot, tail)) => {
                    pre.append(&prefix);
                    post = tail;

                    if (pre.matches_backwards(&sc.before) ^ sc.before.block_context)
                        && (post.matches_forward(&sc.after) ^ sc.after.block_context)
                    {
                        // println!("pivot == {pivot:?}");
                        let (modifs, pivot1) = pivot.apply_scstring(to);
                        // println!("{:?} {:?}", modifs,pivot1);

                        for modif in modifs {
                            pre.last_mut().map(|last| last.add_modif(modif));
                        }

                        pre.append(&pivot1);
                    } else {
                        pre.append(&pivot);
                    }
                }
                //If split is unsuccessful, break the loop to return
                None => {
                    pre.append(&post);
                    break;
                }
            }
        }
        // println!("{pre:?}");
        pre
    }

    fn apply_mut(&mut self, sc: &SoundChangeShell<F>) {
        *self = self.apply(sc);
    }
}
