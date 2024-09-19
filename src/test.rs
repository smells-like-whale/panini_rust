use itertools::Itertools;

use crate::{
    segment::{
        alt_segment::{AltFeature, AltFeatureBundle}, feature::parse_featurebundle, segment_sound_change::{
            sound_change_utilities::{
                self, 
                parse_scstring
            }, PhonObject, 
            SCSegment, 
            SCString, 
            SoundChange
        }, segment_string::SegmentString, ASegment
    }, 
    util::split_on::split_on
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



// const TES1: &str = "[+syl,+dors,-length]->[-tense]/__";
// const TES2: &str = "[+syl,+length]->[-length]/__";
// const TES3: &str = "k->c/__[+dors,+front]";
// const TES4: &str = "[+dors,+syl,+high,-tense]->[+tense,-high]/__";
// const TES5: &str = "[+syl,+dors,-low]->/__#";
// const TES6: &str = "a->@/__#";
// const TES7: &str = "c->t_s/__";

const TES_STR: &str = "kikjaro:";

const TES_LIS: &[&str] = &[
                            "[+syl,+dors,-length, -low]->[-tense]/__", //Short vowel laxing

                            "[+syl,+length]->[-length]/__", //Loss of vowel length

                            "k->c/__[+dors,+front]", //Palatalization of /k/

                            "[+dors,+syl,+high,-tense]->[+tense,-high]/__", //Lowering of short lax hight vowels. E.g. : I -> e and U -> o

                            "[+syl,+dors,-low]->/__#", //Loss of word final non-low vowels

                            "a->@/__#",//Reduction of word final /a/

                            "c->t_s/__",//Affrication of palatalized /k/
                        ];

#[test]
fn parse_sound_change_test() {
    println!("---Start parse_sound_change_test---");
    let tsc = SoundChange::new_vec(TES_LIS).unwrap();
    println!("test 1");
    let mut word = SegmentString::from_string(TES_STR);
    println!("{word:?}");
    println!("test 2");
    // let res = word.apply(&tsc[0]);
    let scs = SoundChange::new_vec(TES_LIS).unwrap();
   
    println!(" Input: {}", word.to_string());
    for sc in scs {
        // println!("Input: {:?}", word.to_string());
        println!("SoundChange: {:?}", sc.to_string());
        word.apply_mut(&sc);
        println!("Word: {:?}", word.to_string());
    }
    // let res_str = res.to_string();
    // println!("Output: {}", res_str);
    println!("Output: {}", &word.to_string());
   
    println!("---End parse_sound_change_test---");
}

#[test]
fn sc_string_iter() {
    let raw_string = "t_h[+voi]p";
    
    let scstring = sound_change_utilities::parse_scstring(raw_string);

    println!("{scstring:?}");
}

#[test]
fn segment_split() {
    let seg_str = "#awt_hjakr@#";
    let word = SegmentString::from_string(seg_str);
    println!("{:?}", word.to_debug_string());

    let sc_str = "[-cont]->[+cont,+voi]/[+voi]__[+voi]";
    let sc_str2 = "->?/#__[+voi,+syl]"; //Cannot currently change the length of a subsegment
    let sc = SoundChange::new(sc_str2).unwrap();

    let new_word = word.apply(&sc);
    
    let scstr1_str = "[+round][-cont]";
    let scstr1 = parse_scstring(scstr1_str).unwrap();

    // let fet = SCSegment::FeatureSeg(prs_fet("[+voice, -labial]").unwrap());
    // let seg = Segment::from_string("p");
    // println!("{:?}", seg.matches(&fet));

    let split =  match word.split_on_first(&scstr1) {
        Some((x,y,z)) => Some((x.to_string(), y.to_string(), z.to_string())),
        None => None,
    };

    println!("{:?}", new_word.to_string());

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
fn alt_segment() {
    let mut altfet = AltFeatureBundle::new();
    altfet.set(AltFeature::Labial, true);

    println!("{}", altfet.to_string());
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
