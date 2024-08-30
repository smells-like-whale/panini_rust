use std::{fmt::Debug, iter::Peekable, ops::Deref};

use unicode_segmentation::UnicodeSegmentation;

use crate::segment::{
        TokenString, 
        token_sound_change::tstring::TString
    };

pub fn pad_mut_string(str: &mut String, pad_length: usize) {
    let str_len = str.graphemes(true).count();//use graphemes(true) from unicode-segmentation
    if str_len <= pad_length {
        let diff = pad_length - str_len;
        for _ in 0..diff {
            str.push(' ');
        }
    } else {
        eprintln!("ERROR: you cannot pad a list and make it smaller");
    }
}

pub fn concat_by_line(str_arr: &[String], seperator: &str) -> String {
    let mut str_pad_width:Vec<usize> = Vec::new(); //this will hold the max line width of the string in str_arr of the same index
    let mut lines_column_arr: Vec<Vec<String>> = Vec::new();
    
    //First calculate the width of each column and accumulate them into a Vec<usize>
    for str in str_arr {
        let str_lines = str.lines();
        let max = str_lines.map(|s| s.graphemes(true).count()).max();
        str_pad_width.push(max.unwrap_or(0));
    }
    
    //Now for each column, split into lines and pad them
    //then accumulate the lists of lines 
    for i in 0..str_arr.len() {
        let mut line_vec: Vec<String> = Vec::new();
        for line in str_arr[i].lines() {
            let mut tmp = line.to_string();
            pad_mut_string(&mut tmp, str_pad_width[i]);
            line_vec.push(tmp);
        }
        lines_column_arr.push(line_vec);
    } 
    
    //The length of the larget array, aka the number of lines in the final output
    let max = lines_column_arr.iter().map(|x| x.len()).max().unwrap_or(0); 
    
    //Pad the shorter arrays with extra lines of the proper width
    for j in 0..lines_column_arr.len() {
        let diff = max - lines_column_arr[j].len();
        (0..diff).for_each(|_i| {
            lines_column_arr[j].push("".repeat(str_pad_width[j]));
        });
    }
    
    //Accumulate the lines horizontally
    let mut lines = Vec::new();
    for i in 0..max {
        lines.push(seperator.to_string());
        for j in 0..lines_column_arr.len() {
            lines[i].push_str(&format!("{}{seperator}", lines_column_arr[j][i]));
        }
        
    }
    //Accumulate the resulting lines
    let mut output = String::new(); 
    for mut line in lines {
        line.push('\n');
        output.push_str(&line);
    }

    output // not yet used
}

pub fn mut_concat_by_line(str: &mut String, str2: &str, seperator: &str) {
    let str_lines = str.lines().zip(str2.lines());
    
    let mut pad_length = (0,0);
    
    for s in str.lines() {
        if pad_length.0 < s.len() {
            pad_length.0 = s.len();
        }
    }
    for s in str2.lines() {
        if pad_length.1 < s.len() {
            pad_length.1 = s.len();
        }
    }
    let mut output = String::new();
    for (s1,s2) in str_lines {
        let mut z1 = s1.to_string();
        pad_mut_string(&mut z1, pad_length.0);
        let mut z2 = s2.to_string();
        pad_mut_string(&mut z2, pad_length.1);
        output.push_str(&format!("{sep} {} {sep} {} {sep}\n", z1, z2, sep=seperator));
    }
}


//This function splits an array in two at the first instance of the substring token, returning
//None if no such substring exists. 
//If token is &[] then is split by entry.
//Example: 
//         let x = split_on_first("abcdabcd", "bc");
//         assert_eq!(x, Some(("a","dabcd")));
//
//         let y = split_on_first("abcdabcd", "a");
//         assert_eq!(y, Some(("", "bcdabcd")));
//
//         let y = split_on_first("abcdabcd", "");
//         assert_eq!(y, Some(("a", "bcdabcd")));
pub fn split_on_first<'a, T: Eq+Debug>(arr: &'a [T], token: &[T]) -> Option<(&'a [T], &'a [T])> {
    let mut result: Option<(&[T],&[T])> = None;
    if token.is_empty() {
        match arr.get(0) {
            Some(_) => return Some((&arr[0..1],&arr[1..])),
            None => return None,
        }
    }

    for i in 0..arr.len() {
        if arr[i] == token[0] {
            match arr[i..].strip_prefix(token) {
                Some(strip) => {
                    result = Some((&arr[0..i], strip));
                    break;
                },
                None => {result = Some((arr,&[]));},
            }
        }
    }
    result
}






//takes a string "[a,b,c]" and returns a vector vec![a,b,c]
pub fn unbracket_arr<'a>(str: &'a str, (left, right): (char,char)) -> Option<Vec<String>> {
    let body = str.trim().strip_prefix(left)?.strip_suffix(right)?;
    
    let arr: Vec<String> = body.split(',').map(|x| x.to_string()).collect();

    Some(arr)
}

trait StrTools {
    fn split_once_wrapper(&self, token: &str) -> Option<(&Self,&Self)>;
}

impl StrTools for str {

    fn split_once_wrapper<'a>(&'a self, token: &str) -> Option<(&'a str, &'a str)> {
        let self_split = self.split_once(token);
        if self_split.is_some() {
            self_split
        } else {
            let p = self.strip_prefix(token).map(|x| ("",x));
            p
        }
    }
}

pub fn split_sc(str: &str) -> Option<(String, String, String, String)> {
    let (first, rest1) = str.split_once_wrapper("->")?;
    let (second, rest2) = rest1.split_once_wrapper("/")?;
    let (third, fourth) = rest2.split_once_wrapper("__")?; //Add logic to handle the cases where the relevant entries are empty
    Some((first.to_string(),second.to_string(),third.to_string(),fourth.to_string()))
}

//fn token_gen_recursive(_s: &str) -> Vec<String> {
//    let length = _s.chars().count();
//    let ess: Vec<char> = _s.chars().collect();
//    let mut tokens: Vec<String> = Vec::new();
//    match length {
//        0 => (),
//
//        1 => tokens.push(_s.chars().nth(0).unwrap().to_string()),
//
//        2 => {
//            if is_initial(ess[0]) {
//                match ess[1] {
//                    '\\' => tokens.push([ess[0], ess[1]].iter().cloned().collect::<String>()),
//
//                    '`'  => tokens.push([ess[0], ess[1]].iter().cloned().collect::<String>()),
//
//                    _    => {
//                            tokens.push(ess[0].to_string());
//                            tokens.extend(token_gen_recursive(&_s.chars().skip(1).collect::<String>()));
//                    }
//                }
//            } else if ess[0] == '_' {
//                tokens.push([ess[0], ess[1]].iter().cloned().collect::<String>());
//            } else {
//                tokens.push("<?>".to_string())
//            }
//        }
//
//        _ => {
//            if is_initial(ess[0]) {
//                match ess[1] {
//                    '\\' => {
//                        match ess[2] {
//                            '`' =>  {
//                                    tokens.push([ess[0], ess[1], ess[2]].iter().cloned().collect::<String>());
//                                    tokens.extend(token_gen_recursive(&_s.chars().skip(3).collect::<String>()));
//                            }
//                            '|' => {
//                                    if ess.len() == 3 {
//                                        tokens.push([ess[0],ess[1]].iter().cloned().collect::<String>());
//                                        tokens.push([ess[2]].iter().cloned().collect::<String>());
//                                        tokens.extend(token_gen_recursive(&_s.chars().skip(3).collect::<String>()));
//                                } else if ess[3] == '\\' {
//                                        tokens.push([ess[0], ess[1], ess[2], ess[3]].iter().cloned().collect::<String>());
//                                        tokens.extend(token_gen_recursive(&_s.chars().skip(4).collect::<String>()));
//                                }
// 
//                            }
//                            _   => {
//                                    tokens.push([ess[0], ess[1]].iter().cloned().collect::<String>());
//                                    tokens.extend(token_gen_recursive(&_s.chars().skip(2).collect::<String>()));
//                            } 
//                        }
//                    }
//
//                    '`' => {
//                        tokens.push([ess[0], ess[1]].iter().cloned().collect::<String>());
//                        tokens.extend(token_gen_recursive(&_s.chars().skip(2).collect::<String>()));
//                    }
//
//                    '_' => {
//                        if ess[2] == '<' || false {
//                            tokens.push([ess[0], ess[1], ess[2]].iter().cloned().collect::<String>());
//                            tokens.extend(token_gen_recursive(&_s.chars().skip(3).collect::<String>()));
//                        } else {
//                            tokens.push(ess[0].to_string());
//                            tokens.extend(token_gen_recursive(&_s.chars().skip(1).collect::<String>()));
//                        }
//                    }
//                    _ => {
//                        tokens.push(ess[0].to_string());
//                        tokens.extend(token_gen_recursive(&_s.chars().skip(1).collect::<String>()));
//                    }
//                }
//            } else if ess[0] == '_' {
//                if ess[2] == '\\' {
//                    tokens.push([ess[0], ess[1], ess[2]].iter().cloned().collect::<String>());
//                    tokens.extend(token_gen_recursive(&_s.chars().skip(3).collect::<String>()));
//                } else {
//                    tokens.push([ess[0],ess[1]].iter().cloned().collect::<String>());
//                    tokens.extend(token_gen_recursive(&_s.chars().skip(2).collect::<String>()));
//                }
//            } else {
//                tokens.push("<?>".to_string());
//                tokens.extend(token_gen_recursive(&_s.chars().skip(1).collect::<String>()));
//            }
//        }
//    }
//    tokens
//}




pub fn get_or<T: Eq+Clone>(vec: &[T], index: isize, fallback: T) -> T {
    if index < 0 {return fallback;}
    match vec.get(index.abs_diff(0)) {
        Some(x) => x.clone(),
        None => fallback,
    }
}

//This checks if tok[] matches the beginning of arr[].
//E.g.: 
// let arr = [1,2,3,4,5];
// let tok1 = [1,2,3];
// let tok2 = [2,3];
// assert!(match_postfix(arr,tok1));
// assert!(!match_postfix(arr,tok2));
pub fn match_prefix<A: Eq>(arr: &[A], tok: &[A]) -> bool {
    if tok.len() > arr.len() {return false;}
    for i in 0..tok.len()  {
        if tok[i] != arr[i] {return false;}
    }
    true
}

//This checks if tok[] matches the end of arr[].
//E.g.: 
// let arr = [1,2,3,4,5];
// let tok1 = [3,4,5];
// let tok2 = [3,5];
// assert!(match_postfix(arr,tok1));
// assert!(!match_postfix(arr,tok2));
pub fn match_postfix<A: Eq+Debug>(arr: &[A], tok: &[A]) -> bool {
    if tok.len() > arr.len() {return false;}
    
    let arrlen = arr.len();
    let toklen = tok.len();
    for i in 0..toklen {
        if tok.get(toklen-i-1) != arr.get(arrlen-i-1) {return false;}// use get() instead
    }
    true
}

pub fn after_matches(tstr: &[TokenString], toks: &[TString]) -> bool {
    if toks.is_empty() {
        true
    } else {
        toks.iter().any(
            |tok| match_prefix(&tstr, &tok.tokens())
        )
    }
}

pub fn before_matches(tstr: &[TokenString], toks: &[TString]) -> bool {
    if toks.is_empty() {
        true
    } else {
        toks.iter().any(
            |tok| match_postfix(&tstr, tok.tokens())
        )
    }
}

