//How to process suprasegmentals? I can handle them as symbols, but not as parts of the syllable 
//as they really must be. I need an algorithm for parsing out syllables given a predetermined 
//syllable structure, maybe it should be implemented alongside the generate module in the near future.
//A similar, and possibly related representation, could be used for tone as well.
use crate::{
    util::utilities::split_on_first, 
    xsampa::xsampa_split_segments
};

const TEST_NAT_CLASS: &str = "C:ptkfshwjrl\nP:ptk\nF:fsh\nL:rlwj\nV:aeiou";
const TEST_SYLLABLE_SHAPES: &str = "(C)(L)V(P)";

#[derive(Debug)]
pub struct NatClasses {
    class_labels: Vec<(String,Vec<String>)>,
}

impl NatClasses {
    pub fn new(input: &str) -> Self {
        let tmp = input.lines().filter_map(|x| x.split_once(":"));
        println!("{:?}", tmp);
        let mut result: Vec<(String, Vec<String>)> = Vec::new();

        for (c, sounds_raw) in tmp {
            let sounds = xsampa_split_segments(sounds_raw);
            result.push((c.to_string(), sounds));
        }
        
        NatClasses {
            class_labels: result,
        }
    }

    pub fn print(&self) -> String{
        let mut str = String::new();

        for (s,sn) in self.class_labels.iter() {
            str.push_str(s);
            str.push_str(" : ");
            for x in sn {
                str.push_str(x);
            }
            str.push('\n');
        }
        str
    }
}

