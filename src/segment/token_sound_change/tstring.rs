use std::ops::{Deref, Index};

use crate::segment::TokenString;

use crate::util::{
    split_on::{
        Splittable,
        SplitOn,
        split_on,
    }
};
//use crate::util::SplitOn;
use crate::{
    xsampa::{
            xsampa, 
            xsampa_split, 
            xsampa_split_segments
        },
};

use itertools::Itertools;
use super::token_sound_change::TokenSoundChange;



pub trait TokenPhonObject: Sized+Clone {
    fn apply_tok_prim(&self, sc: &TokenSoundChange) -> Self;
    fn apply_tok_prim_mut(&mut self, sc: &TokenSoundChange);
    fn apply_tok_prim_vec(&self, scs: &[TokenSoundChange]) -> Self {
        let mut tstr = self.clone();
        for sc in scs {
            tstr.apply_tok_prim_mut(sc);
        }
        tstr
    }
    fn apply_tok_prim_mut_vec(&mut self, scs: &[TokenSoundChange]) {
        for sc in scs {
            self.apply_tok_prim_mut(sc);
        }
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TStr<'a> {
    len: usize,
    start_index: usize,
    tokens:  &'a TString,
}

impl<'a> TStr<'a> {

}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TString {
    tokens: Vec<TokenString>,
}

impl AsRef<Vec<TokenString>> for TString {
    fn as_ref(&self) -> &Vec<TokenString> {
        &self.tokens
    }
}

impl AsMut<Vec<TokenString>> for TString {
    fn as_mut(&mut self) -> &mut Vec<TokenString> {
        &mut self.tokens
    }
}

impl Deref for TString {
    type Target = Vec<TokenString>;

    fn deref(&self) -> &Self::Target {
        &self.tokens
    }
}

impl TString {
    
    pub fn print_words(self_arr: &[Self]) -> String {
        let words: String = self_arr.iter()
            .map(
                |word| word.to_string()
            ).collect::<String>();
        words
    }  
    
    pub fn new() -> Self {
        let val = Vec::new();
        Self { tokens: val }
    }

    pub fn from_arr(token_arr: &[TokenString]) -> Self {
        TString { tokens: token_arr.to_vec()} 
    }

    pub fn from_str(str: &str) -> Self {
        let val_tokens = xsampa_split_segments(str);
        let val = val_tokens.iter().map(|x| xsampa_split(x)).collect();
        Self { tokens: val }
    }

    pub fn to_string(&self) -> String {
        let mut str = String::new();
        
        for i in 0..self.tokens.len() {
            str.push_str(&xsampa(&self.tokens[i].concat()));
        }
        str
    }

    pub fn to_string_vec(&self) -> Vec<String> {
        let mut string_vec: Vec<String> = Vec::new();

        for token in &self.tokens {
            string_vec.push(token.concat());
        }
        string_vec
    }
    
    pub fn insert(&mut self, index: usize, tok_str: TokenString) {
        self.tokens.insert(index, tok_str);
    }
    
    pub fn splice(&mut self, index: usize, tstr: TString) {
        let head = &mut self.tokens; 
        let mut tail = head.split_off(index);
        
        for tok in tstr.tokens {
            head.push(tok);
        }
        head.append(&mut tail);
    }

    pub fn push(&mut self, tok: TokenString) {
        self.tokens.push(tok);
    }

    pub fn push_str(&mut self, tok_str: &TString) {
        for tok in tok_str.tokens() {
            self.push(tok.clone());
        }
    }

    pub fn pop(&mut self) -> Option<TokenString> {
        self.tokens.pop()
    }

    pub fn split_on<'a>(&'a self, tok_str: &'a TString) -> SplitOn<&'a [TokenString]> {
        SplitOn::new(&self, &tok_str) 
        //split_on(&self.tokens, &tok_str.tokens).iter()
        //     .map(
        //     |tokens| TString {tokens: tokens.to_vec()}
        // ).collect_vec()
    }

    pub fn iter(&self) -> std::slice::Iter<Vec<String>> {
        self.tokens.iter()
    }

    pub fn tokens(&self) -> &[TokenString] {
        &self.tokens
    }
    
    pub fn tokens_mut(&mut self) -> &mut [TokenString] {
        &mut self.tokens
    }

}

impl IntoIterator for TString {
    type Item = TokenString;

    type IntoIter =  <Vec<TokenString> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.tokens.into_iter()
    }
}

impl TokenPhonObject for TString {
    fn apply_tok_prim(&self, sc: &TokenSoundChange) -> Self {
        let result = sc.apply_sound_change(self);
        result
    }

    fn apply_tok_prim_mut(&mut self, sc: &TokenSoundChange) {
        let result = sc.apply_sound_change(self);
        *self = result;
    }
}
