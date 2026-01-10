use itertools::Itertools;

use super::util::utilities::split_on_first;
use super::{
    generate::NatClasses,
    xsampa::{
        segment_pop,
        xsampa,
        // xsampa_split_segments,
        SegmentIter,
        TokenIter, //  character_translate,
    },
};
use crate::{
    generate::SylGenSpec,
    segment::{
        feature::{
            feature_shape::FeatureGeometryBundle, feature_table::FeatureTable,
            ComplexFeatureBundleShell, Feature, FeatureBundle,
        },
        segment_sound_change::{
            sound_change_utilities::{self, parse_scstring, segment_raw_scstring},
            PhonObject, SoundChangeShell,
        },
        segment_string::SegmentStringShell,
    },
    util::{split_on::split_on, utilities::concat_by_line},
};
use std::env::args;

// const TES1: &str = "[+syl,+dors,-length]->[-tense]/__";
// const TES2: &str = "[+syl,+length]->[-length]/__";
// const TES3: &str = "k->c/__[+dors,+front]";
// const TES4: &str = "[+dors,+syl,+high,-tense]->[+tense,-high]/__";
// const TES5: &str = "[+syl,+dors,-low]->/__#";
// const TES6: &str = "a->@/__#";
// const TES7: &str = "c->t_s/__";

const TEST_STRS: &[&str] = &[
    "genus",
    // "metipsimu",
    "kae_^lum",
    "kanem",
    // "we:ki:nus",
    // "ratio:nem",
    // "kentum",
    "kapra",
    "eo:",
    "rene",
    // "kabal:um",
    "sapu:tum",
    // "sta:re",
    // "wi:ta",
    "media:num",
    // "fi:lius",
    "wenit",
    "wiridia:rium",
    // "tripa:lia:re",
    // "faktum",
    "kambia:re",
    // "wi:ginti:"
];

//These are the non-stress related sound changes from latin to old french
const PROTO_CENTRAL_ROMANCE: &[&str] = &[
    // "VN{F,#}->V~:F",

    // "[+cor,-cont]->[+dist]/__",
    "h->/__",
    "m->_~/__#",
    "[+cons,+nas]->_~:/__[+cont,+cons]",
    "oe_^->e:/__",
    "ae_^->e/__",
    "w->B/__[+syl]",
    "b->B/[+syl]__[+syl]",
    "[+syl,-len]->/[+syl][-syl,-len]__[-syl,-len][+syl]",
    "[-low,+syl]->[+high,-syl]/__[+syl]",
    "(k,t)j->t_j/__",
    // "kj->t_j/__",
    // "tj->t_j/__",
    // "dj->dz/__",
    // "n->:_~/__[+cont,+cons]",
    "[+syl,-len,-low]->[-atr]/__", //Short vowel laxing
    //
    "->I/#__s[+cons]",
    "[+syl,+len]->[-len]/__", //Loss of vowel length
    "[+syl,+nas]->[-nas]/__", //Denasaliazation
    "j->_j/[+cons]__",
    // "[+cons,+long]->[-long]/__[+cons]",
    // "[+cons,+long]->[-long]/[+cons]__",
    "k_j->k_j:/__",
    "g->j/__[+front,-low,+syl]",
    "ijI->i/__",
];

const PRO_ROM_TO_PRO_WEST: &[&str] = &[
    "[+dors,+high,+cons,-son]->[+high,+cor]/__[+front,-low,+syl]", //Palatalization of /k/ and /g/
    "d_j->j/__",
    "[+syl,+high,-atr]->[+atr,-high]/__", //Lowering of short lax hight vowels. E.g. : I -> e and U -> o
    "[+cons,-voi,-cont]->[+voi]/[+voi,+syl]__[+voi]", //First Lenition
    "kt->jt/__",
];

const PRO_ROM_TO_PRO_GALLO: &[&str] = &[
    // "[+syl,-low]->@/__#", //Reduction of word final non-low vowels
    "[+syl,-low]->@/__([+cor,+cons])_s#", //Reduction of word final non-low vowels
    "Oj->uj/__[+cons]",
    "[+cor,+high,+cons,-voi,-lab]->t-s/__", //Affrication of palatalized /k/ and /t/
    "[+dors,+cor,+high,+cons,+voi,-son,-lab]->z/__", //Affrication of palatalized /k/ and /t/
    // "[+cor,+high,+cons,+voi,-son]->j/[+syl]__",
    "j->d-Z/(#,r)__",
    // "j->d-Z/r__",
    "a->{/__[+high,+front]",
    "[+cons,-cont,+voi,-son]->[+cont]/[+voi,-cons]__[+voi,-cons]",
    "l_j->L/__",
    "r_j->r/__",
    "aw->O/__",
    // "@->/__#",//Loss of word final /@/
    "@->/__([+cor,+cons])_s#", //Loss of word final /@/
    "->@/[+cons]r__#",         //Loss of word final /@/
    "[+len,-syl]->[-len]/__([-syl],#)",
    // "[+len,-syl]->[-len]/__[-syl]",
    "[+cons,-son]->[-voi]/__#",
    "B->/__[+round]",
    "a->@/__#", //Reduction of word final /a/
    "B->v/__",
];
type F = ComplexFeatureBundleShell<FeatureGeometryBundle>;
type SegmentString = SegmentStringShell<F>;
type SoundChange = SoundChangeShell<F>;

#[test]
fn parse_sound_change_test() {
    println!("---Start parse_sound_change_test---");
    let mut test_sc = PROTO_CENTRAL_ROMANCE.to_vec();
    test_sc.extend_from_slice(PRO_ROM_TO_PRO_WEST);
    test_sc.extend_from_slice(PRO_ROM_TO_PRO_GALLO);

    println!("test 1");
    let words = TEST_STRS
        .iter()
        .map(|word| SegmentString::from_string(word));
    let scs = SoundChange::new_vec(&test_sc).unwrap();

    let mut word_sc_table_by_col: Vec<Vec<SegmentString>> = Vec::new();
    for word in words {
        let mut word_col: Vec<SegmentString> = vec![word];
        for sc in &scs {
            let modded = word_col.last().unwrap().apply(sc);
            word_col.push(modded);
            // println!("Input: {:?}", word.to_string());
            //println!("SoundChange: {:?}", sc.to_string());
            //println!("Word: {}", word.to_string());
        }
        word_sc_table_by_col.push(word_col);
    }

    let mut str_cols = vec![
        "\n".to_string()
            + &test_sc
                .iter()
                .map(|str| str.to_string() + "\n")
                .collect_vec()
                .concat(),
    ];

    str_cols.append(
        &mut word_sc_table_by_col
            .iter()
            .map(|col| {
                let mut str = String::new();
                for word in col {
                    // println!("Word: {:?}\n", word.to_tokens());
                    str.push_str(&word.to_string());
                    str.push('\n');
                }
                str
            })
            .collect_vec(),
    );

    let table = concat_by_line(&str_cols, " ");

    println!("SC table: ");
    println!("{table}");

    //  println!("{words:?}");
    //  println!("test 2");
    //  // let res = word.apply(&tsc[0]);
    //
    //  println!(" Input: {}", word.to_string());
    //  // let res_str = res.to_string();
    //  // println!("Output: {}", res_str);
    //  println!("Output: {}", &word.to_string());

    println!("---End parse_sound_change_test---");
}

#[test]
fn sc_string_iter() {
    let raw_string = "t_h[+voi]p";

    let scstring = sound_change_utilities::parse_scstring::<FeatureTable>(raw_string);

    println!("{scstring:?}");
}

#[test]
fn segment_split() {
    let str = "#[+voi,+syl]wt_h(g,h,s,[+cor])_se";
    let segmented_str = segment_raw_scstring(str);
    println!("{:?}", segmented_str);

    let sc_str = "[-cont]->[+cont,+voi]/[+voi]__[+voi]";
    let sc_str2 = "->?/#__[+voi,+syl]"; //Cannot currently change the length of a subsegment
    let sc = SoundChange::new(sc_str2).unwrap();

    // let fet = SCSegment::FeatureSeg(prs_fet("[+voice, -labial]").unwrap());
    // let seg = Segment::from_string("p");
    // println!("{:?}", seg.matches(&fet));
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
    let mut altfet = FeatureTable::new();
    altfet.set(Feature::Labial, true);

    println!("{}", altfet.to_col_str());
}

#[test]
fn split_test() {
    println!("---Start split_test---");
    let str = "da~ le ZaRdE_~ *A_j[-cons](s)(p,t,k)";
    let mut seg_iter = SegmentIter::new(str);

    println!("{:?}", seg_iter);
    println!("{:?}", segment_pop(str));

    while !seg_iter.is_empty() {
        let next_seg = seg_iter.next();

        println!("{next_seg:?}");
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
        let c = FeatureTable::from_string(char_token);
        let segment_debug = c.to_long_debug_string();
        println!("{}", xsampa(char_token));
        println!("{}", segment_debug);
    }
    println!("---End xsampa_segment_test---");
}

const NAT_CLASSES: &str =   "P:p b t d t-S d-Z k g\n
                             C_f:f v T D s z S Z h\n
                             L:r l w j\n
                             V:I U E { V A r aj Ij Oj Ej aw Uw Ow Ar Or Er Ir\n
                             N:n m N";
const TEST_NAT_CLASS: &str =    "C:ptkfsh?\n
                                P:ptk\nC_f:fsh\n
                                L:rlwj\n
                                V:aeiou\n
                                N:nmN";
const TEST_SYLLABLE_SHAPES: &str = "(C)(L)V(P)\n(N)V(P)";

const TEST_ONSET_PATTERN: &str = "((s)(p,t,k),C)(L)";
const TEST_CODA_PATTERNS: &str = "(N,l)(P)";

#[test]
fn nat_class_test() {
    let nc = NatClasses::new(TEST_NAT_CLASS);
    let nc_str = nc.print();

    let ss = SylGenSpec::new(TEST_SYLLABLE_SHAPES);
    let ss_str = ss.print();

    println!("nat_class_test ran successfully");
    println!("{:?}", ss);
    println!("{}", ss_str);

    println!("{:?}", nc);
    println!("{}", nc_str);

    let str = ss.gen(3,&nc);
    println!("{str}");
}
