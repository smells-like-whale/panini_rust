// panini_rust.rs #![allow(dead_code)]
use panini_rust::{
    segment::{
        self,
        feature::{
            feature_shape::{FeatureGeometryBundle, RootFeatures},
            // feature_table::FeatureTable,
            ComplexFeatureBundleShell,
            Feature,
            FeatureBundle,
            FeatureIndex,
        },
        segment_sound_change::{PhonObject, SoundChangeShell},
        segment_string::SegmentStringShell,
        SegmentShell,
    },
    util::utilities::concat_by_line,
    util::csv::CSV,
    xsampa::{character_translate, xsampa, xsampa_split,TOKEN_TRANS_LIST as TLIST},
};
use std::env::args;
use std::path::Path;

// type F = FeatureTable;
// type F = FeatureGeometryBundle;
type F = ComplexFeatureBundleShell<FeatureGeometryBundle>;

type SoundChange = SoundChangeShell<F>;
type SegmentString = SegmentStringShell<F>;
type Segment = SegmentShell<F>;

fn get_row_by_col_1<T: Eq+Clone>(arr: &[Vec<T>],token: &T) -> Option<Vec<T>> {
    for item in arr {
        if &item[0] == token {
            return Some(item.clone());
        } 
    }
    None
}

fn main() {
    let mut args = args().skip(1);
    let next = &args.next();
    match next.as_deref() {
        None => {
            eprintln!("ERROR: No argument given");
            println!("
                xsampa {{-l, -s, -t}}\n
                xpa\n
                segment\n 
                compare\n 
                ipa\n 
                help"
                );
        }
        Some("xsampa") => {
            xsampa_main(&mut args);
        }
        Some("xpa") => {
            let str = args.next().unwrap();
            println!("{} : {}", xsampa(&str), character_translate("|\\|\\"));
        }
        Some("segment") => {
            println!("{}", segment_main(args.next()));
        }
        Some("compare") => {
            compare_main(&mut args);
        }
        Some("ipa") => {
            println!("{}", segment::print_xsampa_chart(true));
        }
        Some("test") => {
            test(&mut args);
        }
        Some("sc") => {
            sound_change(&mut args);
        }
        Some("csv") => {
            let r_w = &args.next();
            match r_w.as_deref() {
                None => {}
                Some("read") => {
                    let test_xsampa = xsampa_split("m_= \"t_hEstn_= n_d@ \"ridn_= ?n_= \"rVj4n_= @ \"si\"Es\"vi \"faj5_=z");
                    let path_str = args.next().unwrap();
                    let path = Path::new(&path_str);

                    let csv = CSV::import(path).unwrap();

                    let mut xsm = String::new();
                    for seg in test_xsampa {
                        let row = get_row_by_col_1(&csv.rows, &seg);
                        let new_seg = match row.as_ref() {
                            Some(arr) => &arr[1],
                            None => " ",
                        };
                        xsm.push_str(new_seg);
                    }

                    println!("{}", xsm);
                    
                }
                Some("write") => {
                    let path_str = args.next().unwrap();
                    let path = Path::new(&path_str);

                    let fields = vec!["ipa_sym".to_string(),"xsampa_sym".to_string()];

                    let mut rows: Vec<Vec<String>> = Vec::new();
                    for (t, c) in TLIST {
                       let row = vec![t.to_string(),c.to_string()];
                       rows.push(row);
                    }
                    
                    let csv = CSV {col_num: 2, fields, rows};

                    csv.export(path);

                }
                _ => {}
            }
        }
        Some("help") => {
            println!("
                xsampa {{-l, -s, -t}}\n
                xpa\n
                segment\n 
                compare\n 
                ipa\n 
                help"
                );
        }
        _ => {
            eprintln!("ERROR: Invalid argument");
        }
    }
}

fn test<'a, I: Iterator<Item = String>>(_args: &mut I) {
    let mut tmp = segment::feature::feature_shape::TEST_FET_GEOM_BUND.clone();
    tmp.set(
        RootFeatures::from_feature(Feature::AdvancedTongueRoot),
        true,
    );
    let mut seg = SegmentStringShell::<FeatureGeometryBundle>::from_string("k_j");
    let s = seg.get_mut(1).unwrap();
    s.update_token();

    let sc =
        SoundChangeShell::<FeatureGeometryBundle>::new("[+cor,+high,+cons,-voi]->t_s/__").unwrap();
    println!("{:?}", sc);

    // seg.apply_mut(&sc);//Panics here
    let str = concat_by_line(
        &[
            "\n".to_string() + &RootFeatures::list_features(),
            s.to_short_debug_string(),
        ],
        " | ",
    );

    println!("{str}");
}

fn sound_change<'a, I: Iterator<Item = String>>(args: &mut I) {
    let str_tokens = &args.next().unwrap();
    let sound_change = SoundChange::new(str_tokens).unwrap();
    let mut word = SegmentString::from_string(&args.next().unwrap());
    let new_word = word.apply(&sound_change);
    // word.apply_mut(&sound_change);

    // word.apply_tok_prim_mut(&tok_sound_change);

    //let x = sound_change.to_string();
    println!(
        "{} => {} via {}",
        word.to_string(),
        new_word.to_string(),
        sound_change.to_string()
    );
}

fn compare_main<'a, I: Iterator<Item = String>>(args: &mut I) {
    let mode = args.next();
    let mut arguments = Vec::new();
    arguments.push("\n".to_string() + &<F as FeatureBundle>::Index::list_features());
    match mode.as_deref() {
        None => {
            eprintln!("ERROR: No argument given");
        }
        Some("segment") => {
            for str in args {
                arguments.push(segment_main(Some(str)));
            }
            // let str1 = segment_main(args.next());
            // let str2 = segment_main(args.next());

            //let str = util::concat_by_line(&str1, &str2, "|");
            let str = concat_by_line(&arguments, " | ");
            println!("{}", str);
        }
        //Some("xsampa") =>
        _ => {
            eprintln!("ERROR: Invalid argument");
        }
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

fn xsampa_main<'a, I: Iterator<Item = String>>(args: &mut I) {
    let option = args.next();
    let str_tokens = &args.next().unwrap();
    // let string_tokens = str_tokens.trim().split_whitespace().map(|str| str.to_string());

    // let words: Vec<SegmentString> = string_tokens
    //                 .map(|str| SegmentString::from_string(&str))
    //                 .collect();

    match option.as_deref() {
        None => {
            eprintln!("Error: No option given");
        }
        Some("-l") => {
            let string_tokens = str_tokens
                .trim()
                .split_whitespace()
                .map(|str| str.to_string());    // Maybe avoid turning this to an owned object for no reason…
            let words: Vec<SegmentString> = string_tokens
                .map(|str| SegmentString::from_string(&str))
                .collect();
            let _x = SegmentString::print_words(&words);
            // let word_long_strings: Vec<String> = words.iter().map(|w| w.to_long_string()).collect();
            // let s = words[0].to_long_string();// Candidate for linewise concat
            // let s = concat_by_line(&word_long_strings, " ");

            println!("{_x}");
        }
        Some("-s") => {
            let x: String = xsampa(str_tokens);
            println!("{x}");
        }
        Some("-t") => {
            let x = xsampa_split(str_tokens);
            println!("{x:?}");
        }
        _ => {
            eprintln!("ERROR: Invalid argument");
        }
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
            seg.to_short_debug_string()
        }
    }
}

/* 0x, 0o, 0b for hexadecimal, octal, and binary respectively
 * underscores for readability such as 1_000 or 1_000_000
 */
