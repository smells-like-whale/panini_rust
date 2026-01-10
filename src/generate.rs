//How to process suprasegmentals? I can handle them as symbols, but not as parts of the syllable
//as they really must be. I need an algorithm for parsing out syllables given a predetermined
//syllable structure, maybe it should be implemented alongside the generate module in the near future.
//A similar, and possibly related representation, could be used for tone as well.
use crate::{
    // util::utilities::split_on_first,
    xsampa::xsampa_split_segments,
};
use rand::seq::SliceRandom;

// const TEST_NAT_CLASS: &str = "C:ptkfshwjrl\nP:ptk\nF:fsh\nL:rlwj\nV:aeiou";
// const TEST_SYLLABLE_SHAPES: &str = "(C)(L)V(P)";

#[derive(Debug)]
pub struct NatClasses {
    class_labels: Vec<(String, Vec<String>)>,
}

impl NatClasses {
    pub fn new(input: &str) -> Self {
        let tmp = input.lines().filter_map(|x| x.split_once(":"));
        // println!("{:?}", tmp);
        let mut class_labels: Vec<(String, Vec<String>)> = Vec::new();

        for (c, sounds_raw) in tmp {
            let sounds = xsampa_split_segments(sounds_raw);
            class_labels.push((c.to_string(), sounds));
        }

        NatClasses { class_labels }
    }

    pub fn print(&self) -> String {
        let mut str = String::new();

        for (s, sn) in self.class_labels.iter() {
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

#[derive(Debug)]
pub struct SylGenSpec {
    //true for optional, false for obligatory
    syl_patterns: Vec<Vec<(String, bool)>>,
}

impl SylGenSpec {
    
    pub fn new(str: &str) -> Self {
        let str_lines = str.lines();

        let mut syl_patterns = Vec::new();
        for str in str_lines {
            syl_patterns.push(Self::parse_pattern(str)); 
        }

        SylGenSpec { syl_patterns }

    }

    pub fn parse_pattern(str: &str) -> Vec<(String, bool)> {
        let mut syl_tokens: Vec<(String, bool)> = Vec::new();
        let chars: &[char] = &str.chars().collect::<Vec<char>>();
        let len = chars.len();
        let mut i = 0;
        while i < len {
            if chars[i] == '(' {
                let start = i + 1;
                let mut tmp_tok = String::new();
                for j in start..len {
                    if chars[j] == ')' {
                        i = j;
                        break;
                    } else {
                        tmp_tok.push(chars[j]);
                    }
                }
                syl_tokens.push((tmp_tok, true));
            } else {
                let mut tmp = chars[i].to_string();
                if i + 1 < len && chars[i + 1] == '_' {
                    i += 1;
                    tmp.push(chars[i]);
                    if i + 1 < len {
                        tmp.push(chars[i + 1]);
                        i += 1;
                    }
                }
                syl_tokens.push((tmp, false));
            }
            i += 1;
        }

        syl_tokens
    }

    pub fn gen(&self, max_len: u8, nc: &NatClasses) -> String {
        let mut result = String::new();
        let mut x = Vec::new(); 
        for i in 1..=max_len {
            x.push(i); 
        }

        let len: u8 = *x.choose(&mut rand::thread_rng()).unwrap();

        for i in 0..len {
            let tmp = self.gen_syl(nc);
            result.push_str(&tmp);
        }

        result
    }

    pub fn gen_syl(&self, nc: &NatClasses) -> String {
        let mut result = String::new();
        for lb in &self.syl_patterns[0] {
            let str = Self::pick_rand(lb,nc).unwrap_or("".to_string());
            result.push_str(&str);
        }

        result
    }

    fn pick_rand((str,b): &(String, bool), nc: &NatClasses) -> Option<String> {
        for (label,arr) in &nc.class_labels {
            if str == label {
                let seg = arr.choose(&mut rand::thread_rng()).map(|s| s.to_string());
                return seg;
            }
        }

        None
         
    }

    pub fn print(&self) -> String {
        let mut str = String::new();

        for (t, b) in &self.syl_patterns[0] {
            if *b {
                let tmp = "(".to_string() + t + ")";
                str.push_str(&tmp);
            } else {
                str.push_str(t);
            }
        }
        str
    }
}
