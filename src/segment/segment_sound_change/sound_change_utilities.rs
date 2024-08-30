use itertools::Itertools;

use crate::xsampa::is_initial;

use super::sound_change_segment_string::SCString;

pub fn segment_raw_scstring(str: &str) -> Option<Vec<&str>> {
    let sciter = SCStringIter::new(str);

    Some(sciter.collect_vec())
}

pub fn parse_segmented_scstring(str_arr: &[&str]) -> Option<Vec<SCString>> {
    todo!();
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

fn xsampa_normalize(s: &str) -> &str {
    match s {
        "=" => "_=",
        "~" => "_~",
        _   => s,
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
