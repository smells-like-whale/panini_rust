use itertools::Itertools;

use crate::segment::feature::parse_featurebundle;
use crate::segment::segment_string::SegmentString;
use crate::segment::{self, FeatureBundle};
use crate::xsampa::{is_initial, xsampa_split};

use crate::segment::segment_sound_change::SCString;

use super::{SCSegment, SoundChange};

pub fn segment_raw_scstring(str: &str) -> Option<Vec<String>> {
    let sciter = SegmentIter::new(str);

    Some(sciter.collect_vec())
}

pub fn parse_segmented_scstring(str_arr: &[String]) -> Option<SCString> {
    let mut segments = Vec::new();

    for str in str_arr {
        if str == "#" {
            segments.push(SCSegment::WordEdge);
        } else if str.starts_with('[') {
            segments.push(SCSegment::FeatureSeg(parse_featurebundle(str)?));
        } else {
            segments.push(SCSegment::TokenSeg(xsampa_split(str)));
        }
    }

    Some(SCString { segments }) 
}

pub fn parse_scstring(str: &str) -> Option<SCString> {
    let segmented_str = segment_raw_scstring(str)?;
    println!("{segmented_str:?}");
    parse_segmented_scstring(&segmented_str)
}

#[derive(Clone, Copy)]
pub struct SCStringIter<'a> {
    string: &'a str
}
impl<'a>  SCStringIter<'a> { 

    pub fn new(string: &'a str) ->  SCStringIter<'a> {
         SCStringIter {
            string
        }
    }

    fn peek(&self) -> Option<&'a str> {
        if self.string.is_empty() {
            None
        } else {
            let (pop,_) = token_pop(self.string); 
            Some(pop)
        }
    }

    fn unwrap(&self) -> &str {
        self.string
    }

    pub fn is_empty(&self) -> bool {
        match self.peek() {
            Some(_) => false, 
            None => true,
        }
    }

}
pub fn segment_pop<'a>(str: &'a str) -> (String,String) {
    let mut pop_raw: Vec<&str> = Vec::new();

    let mut token_iter = SCStringIter::new(str); 
    
    let first_pop_wrapped = token_iter.next();

    match first_pop_wrapped {
        None => {return ("".to_string(),"".to_string());},
        Some(_s) => {pop_raw.push(_s);},
    } 

    for pop in token_iter {
        if ['_',':'].contains(&pop.chars().nth(0).unwrap()) {
            token_iter.next();
            pop_raw.push(pop);
        } else {
            break;
        }
    }

    (pop_raw.concat(), token_iter.unwrap().to_string())
}

fn xsampa_normalize(s: &str) -> &str {
    match s {
        "=" => "_=",
        "~" => "_~",
        _   => s,
    }
}

#[derive(Debug)]
pub struct SCIter {
    sc: SoundChange,
    seg_string: SegmentString,
}

impl Iterator for SCIter {
    type Item = SegmentString;

    fn next(&mut self) -> Option<Self::Item> {
        let (next, _pivot, rest) = self.seg_string.split_on_first(&self.sc.from)?;
        self.seg_string = rest;
        Some(next)
    }
}



#[derive(Debug)]
pub struct SegmentIter {
    string: String
}

impl SegmentIter {
    pub fn new(string: &str) -> SegmentIter {
        SegmentIter { string: string.to_string() }
    }

    fn peek(&self) -> Option<String> {
        if self.string.is_empty() {
            None
        } else {
            let (pop,_) = segment_pop(&self.string); 
            Some(pop)
        }
    }
    
    pub fn is_empty(&self) -> bool {
        match self.peek() {
            Some(_) => false, 
            None => true,
        }
    }
}

impl Iterator for SegmentIter {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.string.is_empty() {
            None
        } else {
            let (pop,rest) = segment_pop(&self.string);
            self.string = rest;
            Some(pop)
        }
    }
}

impl<'a> Iterator for  SCStringIter<'a> {
    type Item = &'a str; 

    fn next(&mut self) -> Option<Self::Item> {
        if self.string.is_empty() {
            None
        } else {
            let (pop,rest) = token_pop(self.string);
            self.string = rest;
            Some(xsampa_normalize(pop))
        } 
    }

}

fn arr_find<T: Eq+PartialEq>(arr: &[T], elem: T) -> Option<usize> {
    for i in 0..arr.len() {
        if arr[i] == elem {
            return Some(i+1);
        } 
    }
    None
}

fn token_pop(_s: &str) -> (&str, &str) {
    let s: Vec<char> = _s.chars().collect();
    let length = s.len();
    let mut start: usize = 0;

    if length == 0 {
        return _s.split_at(start);
    }
    
    if s[start] == '#' {
        return _s.split_at(1);
    }

    if s[start] == '[' {
        let end = s.iter().position(|&x| x==']');//arr_find(&s,']');
        if end.is_some() {
            return _s.split_at(end.unwrap()+1);
        } 
    }

    if is_initial(s[start]) {
        start+=1;
        
        if length - start == 0 || is_initial(s[start]) {
            return _s.split_at(start);
        }

        match s[start] {
            '\\' => {
                start+=1;

                if length - start == 0 {
                    return _s.split_at(start);
                }

                match s[start] {
                    '`' => {
                        start+=1;
                    },
                    '|' => {
                        start+=1;
                        if length - start == 0 || is_initial(s[start]){
                            return _s.split_at(start);
                        }
                        start+=1;
                    },
                    _   => {},
                }
            },

            '`'  => {
                start+=1;
            },
            '_'  => {
                start+=1;
                if length - start > 0 {
                    match s[start] {
                        '<' => {
                            start+=1;
                        },
                        's' | 'z' => {
                            start+=1;
                            if s.get(start) == Some(&'\\') {
                                start+=1;
                            }
                        },

                        'S' | 'Z' => {
                            start+=1;
                        },

                        _ => {
                            start-=1;
                        },
                    }
                } else {
                    start-=1;
                    return _s.split_at(start);
                }
            },
            _ => {},
        }
    } else if s[start] == '_' {
        start+=1;
        if length - start == 0 {
            return _s.split_at(start);
        }
        start+=1;
        if length - start == 0 {
            return _s.split_at(start);
        }

        if s[start] == '\\' {
            start+=1;
        }
    } else {
        start+=1;
        let (_,tail) = _s.split_at(start);
        return ("<?>",tail);
    }

    _s.split_at(start)

}
