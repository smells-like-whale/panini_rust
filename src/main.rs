// panini_rust.rs #![allow(dead_code)]
use panini_rust::{
    segment::{
        self, 
        feature_sound_change::{
            FeaturePhonObject, 
            FeatureSoundChange
        }, 
        Segment, 
        SegmentString,
        token_sound_change::{
            tstring::{
                TString, 
                TokenPhonObject,
            }, 
            token_sound_change::TokenSoundChange,
        },
    }, 
    xsampa, 
    util::{
        self,
        utilities::concat_by_line,
    }
};
use std::env::args;


//fn test<'a, I: Iterator<Item=String>>(args: &mut I) {
//    let str = &args.next().unwrap();
//    
//    println!("{str}");
//    let unbracketed = util::unbracket_arr(str, ('[',']')).unwrap();
//
//    println!("{unbracketed:?}");
//
//}

fn test<'a, I: Iterator<Item=String>>(args: &mut I) {
    let str = &args.next().unwrap();
     
    let seg1 = Segment::from_string(str);
    
    let seg2 = Segment::from_feature_bundle(seg1.features);

    let result = concat_by_line(&[seg1.to_debug_string(),seg2.to_debug_string()], "|");

    println!("{result}");

}

fn sound_change_main<'a, I: Iterator<Item=String>>(mut args: &mut I) {
    match args.next().as_deref().unwrap() {
        "segment" =>{segment_sound_change_main(&mut args);},
        "token" =>{token_sound_change_main(&mut args)},
        _ =>{},
    }
    

}

fn segment_sound_change_main<'a, I: Iterator<Item=String>>(args: &mut I) {
    let str_tokens = &args.next().unwrap();
    
    let seg_sound_change = FeatureSoundChange::new(str_tokens).unwrap();

    // println!("{:?}", seg_sound_change);

    let mut word = SegmentString::from_string(&args.next().unwrap());
    let new_word = word.apply_seg_prim(&seg_sound_change);

    word.apply_seg_prim_mut(&seg_sound_change);

    //let x = sound_change.to_string();
    println!("{} : {}", new_word.to_string(), word.to_string());
}

fn token_sound_change_main<'a, I: Iterator<Item=String>>(args: &mut I) {
    let str_tokens = &args.next().unwrap();
    let tok_sound_change = TokenSoundChange::new(str_tokens).unwrap();
    let mut word = TString::from_str(&args.next().unwrap());
    let new_word = word.apply_tok_prim(&tok_sound_change);

    word.apply_tok_prim_mut(&tok_sound_change);

    //let x = sound_change.to_string();
    println!("{} : {}", new_word.to_string(), word.to_string());
}


fn main(){
    actual_main()
}

fn actual_main() {
    let mut args = args().skip(1);
    let next = &args.next();
    match next.as_deref() {
        None => {eprintln!("ERROR: No argument given");}
        Some("xsampa") => {xsampa_main(&mut args);}
        Some("xpa") => {
            let str = args.next().unwrap();
            println!("{} : {}", xsampa::xsampa(&str), xsampa::character_translate("|\\|\\"));
        }
        Some("segment") => {println!("{}", segment_main(args.next()));}
        Some("compare") => {compare_main(&mut args);}
        Some("ipa") => {println!("{}", segment::print_xsampa_chart());}
        Some("test") => {test(&mut args);}
        Some("sc") => {sound_change_main(&mut args);}
        Some("help") => {println!(" xsampa, segment, help");}
        _ => {eprintln!("ERROR: Invalid argument");}
    }
}

fn compare_main<'a, I: Iterator<Item=String>>(args: &mut I) {
    let mode = args.next();
    match mode.as_deref() {
        None => {eprintln!("ERROR: No argument given");}
        Some("segment") => {
            let str1 = segment_main(args.next());
            let str2 = segment_main(args.next());
            
            //let str = util::concat_by_line(&str1, &str2, "|");
            let str = concat_by_line(&[str1, str2], "|");
            println!("{}", str);

        }
        //Some("xsampa") =>
        _ => {eprintln!("ERROR: Invalid argument");}
    }

}

//fn xsampa_main<'a, I: Iterator<Item=String>>(args: &mut I) {
//    let arg = args.next();
//    if arg == None {
//        eprintln!("ERROR: No argument given");
//    } else {
//        let str = &arg.unwrap();
//        let c = segment::Word::from_string(str);
//        //let d: Vec<Vec<String>> = str.split_whitespace().map(|x| xsampa_split(x)).collect();
//        println!("{}", c.to_long_string());
//        //println!("{:?}", d);
//    }
//}

fn xsampa_main<'a, I: Iterator<Item=String>>(args: &mut I) {
    let option = args.next();
    let str_tokens = &args.next().unwrap();
    let string_tokens = str_tokens.trim().split_whitespace().map(|str| str.to_string());
    
    let words: Vec<SegmentString> = string_tokens
                    .map(|str| SegmentString::from_string(&str))
                    .collect();

    match option.as_deref() {
        None => {eprintln!("Error: No option given");},
        Some("-l") => {
            let x = SegmentString::print_words(&words);
            println!("{x}");
        },
        Some("-s") => {
            let x: String = concat_by_line(&words.iter().map(|word| word.to_string()+" ").collect::<Vec<_>>(), " ");
            println!("{x}");
        },
        _ => {eprintln!("ERROR: Invalid argument");}
    }

    

}

fn segment_main(str_arg: Option<String>) -> String {
    match str_arg {
        None => {
            eprintln!("ERROR: No argument given");
            "".to_string()
        }
        Some(str) => {
            let seg = Segment::from_string(&str);
            //println!("Note: The Feature List does not take into account the features modified by non-segmental diacritics\n");
            seg.to_debug_string()

        }
    }
}




/* 0x, 0o, 0b for hexadecimal, octal, and binary respectively
 * underscores for readability such as 1_000 or 1_000_000
 */
