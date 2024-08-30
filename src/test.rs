// panini_rust.rs
#![allow(dead_code)]
use itertools::Itertools;

use crate::{
    segment::segment_sound_change::sound_change_utilities, 
    segment::token_sound_change::{
        token_sound_change::TokenSoundChange,
        tstring::{
            TString, 
            TokenPhonObject
        }, 
    }, util::split_on::split_on
};
use super::{
        xsampa::{
            segment_pop,
            xsampa,
            xsampa_split_segments,
            SegmentIter,
            TokenIter
            //  character_translate,
        },
    generate::NatClasses, 
    segment::
        FeatureBundle
};
use super::util::utilities::split_on_first;
use std::env::args;



const TES1: &str = "i->I/__";
const TES2: &str = "u->U/__";
const TES3: &str = "e->E/__";
const TES4: &str = "o->O/__";
const TES5: &str = "i:->i/__";
const TES6: &str = "u:->u/__";
const TES7: &str = "e:->e/__";
const TES8: &str = "o:->o/__";
const TES9: &str = "a:->a/__";
const TES_SC: &str = "k->c/__{i,e,I,E,i:,e:}";
const TES10: &str = "I->e/__";
const TES11: &str = "U->o/__";
const TES12: &str = "i->/__#";
const TES13: &str = "u->/__#";
const TES14: &str = "e->/__#";
const TES15: &str = "o->/__#";
const TES16: &str = "E->/__#";
const TES17: &str = "O->/__#";
const TES18: &str = "a->@/__#";
const TES19: &str = "c->ts/__";

const TES_STR: &str = "kikero:";

const TES_LIS: &[&str] = &[
                            TES1,
                            TES2,
                            TES3,
                            TES4,
                            TES5,
                            TES6,
                            TES7,
                            TES8,
                            TES9,
                            TES_SC,
                            TES10,
                            TES11,
                            TES12,
                            TES13,
                            TES14,
                            TES15,
                            TES16,
                            TES17,
                            TES18,
                            TES19,
                        ];

#[test]
fn parse_sound_change_test() {
    println!("---Start parse_sound_change_test---");
    let tsc = TokenSoundChange::new_vec(TES_LIS).unwrap();
    println!("test 1");
    let mut word = TString::from_str(TES_STR);
    println!("{word:?}");
    println!("test 2");
    let res = word.apply_tok_prim(&tsc[0]);
    let scs = TokenSoundChange::new_vec(TES_LIS).unwrap();
    
    println!(" Input: {}", word.to_string());
    for sc in scs {
        word.apply_tok_prim_mut(&sc);
    }
    let res_str = res.to_string();
    //println!("Output: {}", res_str);
    println!("Output: {}", &word.to_string());
   
    println!("---End parse_sound_change_test---");
}

#[test]
fn sc_string_iter() {
    let raw_string = "t[+voi]p";
    
    let segmented_raw_string = sound_change_utilities::segment_raw_scstring(raw_string);

    println!("{segmented_raw_string:?}");
}


#[test] 
fn split_on_test() {
    let str = "aka";
    let str_arr = str.chars().collect_vec();
    let token = &[];
    let split_first = split_on_first(&str_arr, token); 
    println!("{split_first:?}");
    println!("{:?}", split_first.unwrap().1);
    let split_second = split_on_first(&split_first.unwrap().1, token);
    println!("{split_second:?}");
    let split_third = split_on_first(&split_second.unwrap().1, token); 
    println!("{split_third:?}");

    let str_arr_split = split_on(&str_arr, token); // Expected behaviour is to return a split on all segments
    println!("{str_arr_split:?}");
    for spl in str_arr_split {
        println!("{spl:?}");
    }
    println!("{:?}", split_on(&str_arr, token));
}

#[test]
fn get_or_test() {
    let vec = vec![1,2,3];

    for i in -1..4 {
        println!("{}",super::util::utilities::get_or(&vec,i,0));
    }
}

#[test]
fn b() {
    let str = xsampa_split_segments("kikero:");
    let spl_str = split_on(&str, &["k".to_string()]);
    let (_, res) = split_on_first(&str, &["k".to_string()]).unwrap();
    let (fst, rest) = split_on_first(res, &["k".to_string()]).unwrap();
    println!("({:?}, {:?})", fst, rest);
    println!("{:?}", str);
    println!("{:?}", spl_str);
}

#[test]
fn split_test() {
    println!("---Start split_test---");
    let str = "da~ le ZaRdE_~";
    let mut seg_iter = SegmentIter::new(str);
    let mut tok_iter = TokenIter::new(str);
    
    println!("{:?}",seg_iter);
    println!("{:?}", segment_pop(str));

    while !tok_iter.is_empty() || !seg_iter.is_empty() {
        let next_seg = seg_iter.next(); 
        let next_tok = tok_iter.next(); 
    
        println!("{:?}  ::  {:?}", next_seg, next_tok);
    }
    println!("---End split_test---");
}

#[test]
fn xsampa_segment_test() {
    println!("---Start xsampa_segment_test---");
    let arg_temp = args().nth(1);
    if arg_temp == None {
        eprintln!("ERROR: No argument given");
    } else {
        let arg = &arg_temp.unwrap();
        let char_token = arg;
        let c = FeatureBundle::from_string(char_token);
        let segment_debug = c.to_debug_string();
        println!("{}", xsampa(char_token));
        println!("{}", segment_debug);
    }
    println!("---End xsampa_segment_test---");
}

const TEST_NAT_CLASS: &str = "C:ptkfshwjrl\nP:ptk\nF:fsh\nL:rlwj\nV:aeiou";

#[test]
fn nat_class_test() {
    let nc = NatClasses::new(TEST_NAT_CLASS);    
    let str = nc.print();
    println!("nat_class_test ran successfully");
    println!("{:?}", nc);
    println!("{}", str);
}
