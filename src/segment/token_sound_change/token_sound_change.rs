use itertools::Itertools;

//use crate::segment::TokenString;

use crate::segment::TokenString;
use crate::util::{
    utilities::{
        after_matches, 
        before_matches, 
        split_sc, 
    },
    split_on::SplitOn,
};

use super::tstring::TString;




//Transforms a subsequence of tokens, of the form [from] to [to] 
//when the subsequence [before] occurs before the subsequence, and [after], after
//Eg: let sound_change_example = SoundChange::new("k->p/V__t"); 
//"Attested in romanian, Lat:octō -> Rom:opt", with additional terminal vowel deletion.
//Note that this struct uses SegmentString instead of Word, the Word struct contains a massive
//amount of information that SoundChange and its associated functions will never use.
#[derive(Clone,Debug)]
pub struct TokenSoundChange { 
    from: TString,
    to: TString,
    before: Vec<TString>,
    after: Vec<TString>,
}

impl TokenSoundChange {

pub fn new(str: &str) -> Option<TokenSoundChange> {
    let (from_raw,to_raw,before_raw,after_raw) = split_sc(str)?;
    let from = TString::from_str(&from_raw);
    let to   = TString::from_str(&to_raw);

    let before_str = parse_list_str(&before_raw)?;
    let before = before_str.iter()
                    .map(|x| TString::from_str(x))
                    .collect::<Vec<TString>>(); 
    
    let after_str = parse_list_str(&after_raw)?;
    let after = after_str.iter()
                    .map(|x| TString::from_str(x))
                    .collect::<Vec<TString>>();
   
    let sc = TokenSoundChange{from,to,before,after};
    Some(sc)
}

pub fn new_vec(strs: &[&str]) -> Option<Vec<TokenSoundChange>> {
        let mut vec = Vec::new();
        for s in strs {
            match TokenSoundChange::new(s) {
                Some(sc) => {vec.push(sc);},
                None => {return None;},
            }
        }
        Some(vec)
    }

    pub fn apply_sound_changes(scs: &[TokenSoundChange], tok_str: TString) -> TString {
        let mut result = tok_str.clone();
        
        for sc in scs {
            result = sc.apply_sound_change(&result); 
        }
        result
    }
    
    // This applies a single sound change to a word.
    // A word defined in this context is a string of XSampa symbols
    // with no spaces.
    pub fn apply_sound_change_mut(&self, tok_str: &mut TString) -> TString {
        tok_str.push(vec!["#".to_string()]);
        tok_str.insert(0, vec!["#".to_string()]);

         //let mut token_list = vec!["#".to_string()];
         //        token_list.extend_from_slice(&xsampa_split_segments(tok_str));
         //        token_list.extend_from_slice(&["#".to_string()]);
        
        let y = tok_str.split_on(&self.from).map({
                |token| TString::from_arr(token)
            }).collect_vec();
        let x = self.apply_sound_change_helper(&y);
        
        let z = x.iter().filter(|&_s| _s != &["#"]);

        let mut result = TString::new();

        for s in z {
            result.push(s.clone());
        }

        result
    }


    // This applies a single sound change to a word.
    // A word defined in this context is a string of XSampa symbols
    // with no spaces.
    pub fn apply_sound_change(&self, tok_str: &TString) -> TString {
        let mut tstr = tok_str.clone();
        tstr.push(vec!["#".to_string()]);
        tstr.insert(0, vec!["#".to_string()]);

        // let mut token_list = vec!["#".to_string()];
        //         token_list.extend_from_slice(&xsampa_split_segments(tok_str));
        //         token_list.extend_from_slice(&["#".to_string()]);
        let y = tstr.split_on(&self.from).collect_vec();
        let x = y.iter().map(
            |tstring| TString::from_arr(tstring)
        ).collect_vec();


        let x = self.apply_sound_change_helper(&x);
        
        let z = x.iter().filter(|&_s| _s != &["#"]);

        let mut result = TString::new();

        for s in z {
            result.push(s.clone());
        }

        result
    }
   
    //fn sound_change_help<'a, 'b>(&self , split: SplitOn<&[TokenString]>) -> TString {
    //    let word_edge = TString::from_str("#");
    //    let mut previous_chunk: TString = word_edge.clone();
    //    let mut result = TString::new(); 
    //    
    //    for current_chunk in split {
    //        if before_matches(&previous_chunk, &self.before) && after_matches(current_chunk, &self.after) {
    //            result.push_str(&previous_chunk);
    //            result.push_str(&self.to);
    //        } else {
    //            result.push_str(&previous_chunk);
    //            result.push_str(&self.from);
    //        }
    //                  
    //        let tmp = TString {tokens: current_chunk.to_vec()};

    //        previous_chunk = tmp;
    //    }
    //    if before_matches(&previous_chunk, &self.before) && &self.after == &[word_edge] {
    //        result.push_str(&previous_chunk);
    //        result.push_str(&self.to);
    //    } else {
    //        result.push_str(&previous_chunk);
    //        result.push_str(&self.from);
    //    }
    //    //Deal with final case here? Initial and medial can be handled easily inside the loop.
    //    result
    //}

    //This is not properly handeling the before_matches case
    fn apply_sound_change_helper(&self, tok_str_split: &[TString]) -> TString {
        if tok_str_split.is_empty() {
            eprintln!("SoundChange.apply_sound_change: invalid input, empty list");
            return TString::new();
        } else if tok_str_split.len() == 1 {
            return tok_str_split[0].clone();
        }

        let mut result = TString::new();

        for i in 0..tok_str_split.len()-1 {
            if before_matches(&tok_str_split[i], &self.before) && after_matches(&tok_str_split[i+1], &self.after) {
                result.push_str(&tok_str_split[i]);
                result.push_str(&self.to);
            } else {
                result.push_str(&tok_str_split[i]);
                result.push_str(&self.from);
            }
        }
        result.push_str(&tok_str_split[tok_str_split.len()-1]);
        result
    }
}

fn parse_list_str(str: &str) -> Option<Vec<&str>> {
    let mut chars = str.chars(); 
    
    let unsplit_string: &str;

    if chars.nth(0).eq(&Some('{')) && chars.last().eq(&Some('}')) {
        unsplit_string = str.get(1..str.len()-1)?;

        let split_string: Vec<&str> = unsplit_string.split(",").collect(); 
        return Some(split_string);
    } else {
        Some(if str=="" {Vec::new()} else {vec![str]})
    }
}


//pub fn parse_sound_changes(lis: &[&str], str: &str) -> String { let scs = lis.iter().map(|x| TokenSoundChange::new(x).unwrap()).collect::<Vec<TokenSoundChange>>();
//    let res = TokenSoundChange::apply_sound_changes(&scs, str);
//    
//    res 
//}


