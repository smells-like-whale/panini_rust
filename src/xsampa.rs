//#![feature(array_zip)]

//Takes an XSampa representation and returns the IPA equivalent 
pub fn xsampa(_s: &str) -> String {
    _s.split_whitespace().map(|_x| xsampa_split(_x))
        .map(|_x| _x
                .iter()
                .map(|_y| character_translate(_y))
                .collect())
        .collect::<Vec<String>>()
        .join(" ")
}

//Takes an XSampa representation and returns a String vector of its component tokens 
//For example "dA_~" => ["d","A","_~"]
pub fn xsampa_split(_s: &str) -> Vec<String> {
    TokenIter::new(_s)
        .map(|_x| xsampa_normalize(_x)
        .to_string())
        .collect::<Vec<String>>()
}


//Takes an XSampa representation and returns a String vector of its component segments 
//For Example "dA_~" => ["d", "A_~"]
pub fn xsampa_split_segments(_s: &str) -> Vec<String> {
    SegmentIter::new(_s)
        .map(|_x| xsampa_normalize(&_x)
        .to_string())
        .collect::<Vec<String>>()
}

//This functiopn returns true if the input char is a valid initial character for a XSampa segment
//token
pub fn is_initial(_c: char) -> bool {
    _c.is_ascii_alphanumeric()
        || ['@', '&', '^', ':', '!', '?', '{', '}', '|', '~', '=', '<', '>','"','#'].contains(&_c)
}

#[derive(Clone, Copy)]
pub struct TokenIter<'a> {
    string: &'a str
}
impl<'a> TokenIter<'a> { pub fn new(string: &'a str) -> TokenIter<'a> {
        TokenIter {
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

impl<'a> Iterator for TokenIter<'a> {
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

fn token_pop(_s: &str) -> (&str, &str) {
    let s: Vec<char> = _s.chars().collect();
    let length = s.len();
    let mut start: usize = 0;

    if length == 0 {
        return _s.split_at(start);
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



//fn alt_character_translate(_s: &str) -> char {
//    for (x,y) in THE_LIST.iter() {
//        if x == &_s {return *y}
//    }
//    '?'
//}

pub fn character_translate(_s: &str) -> char {
    findr(_s, &TOKEN_TRANS_LIST).unwrap_or('?')
}


fn findr<A,B>(_a: A, tuple_list: &[(A,B)]) -> Option<B> 
    where 
        A: std::cmp::PartialEq,
        B: std::marker::Copy,
   {
    for (a,b) in tuple_list.iter() {
        if a == &_a {return Some(*b)}
    }
    None
}

fn xsampa_normalize(s: &str) -> &str {
    match s {
        "=" => "_=",
        "~" => "_~",
        _   => s,
    }
}

pub fn segment_pop<'a>(str: &'a str) -> (String,String) {
    let mut pop_raw: Vec<&str> = Vec::new();

    let mut token_iter = TokenIter::new(str); 
    
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

/*
 * 0 to 108 are segmental tokens
 * 109 to 164 are non-segmental/modifiers and diacritics
 */
const TOKEN_TRANS_LIST: [(&str,char); 174] =[
       ( "p"       , 'p'),
       ( "b"       , 'b'),
       ( "b_<"     , '\u{0253}'),
       ( "p\\"     , '\u{0278}'),
       ( "B"       , '\u{03B2}'),
       ( "B\\"     , '\u{0299}'),
       ( "m"       , 'm'),
       ( "f"       , 'f'),
       ( "v"       , 'v'),
       ( "F"       , '\u{0271}'),
       ( "v\\"     , '\u{028B}'),
       ( "T"       , '\u{03B8}'),
       ( "D"       , '\u{00F0}'),
       ( "t"       , 't'),
       ( "d"       , 'd'),
       ( "d_<"     , '\u{0257}'),
       ( "n"       , 'n'),
       ( "s"       , 's'),
       ( "z"       , 'z'),
       ( "K"       , '\u{026C}'),
       ( "K\\"     , '\u{026E}'),
       ( "4"       , '\u{027E}'),
       ( "r"       , 'r'),
       ( "r\\"     , '\u{0279}'),
       ( "l"       , 'l'),
       ( "S"       , '\u{0283}'), //( "t_S"   , '\u{0283}'),
       ( "Z"       , '\u{0292}'), //( "d_Z"   , '\u{0292}'),
       ( "t`"      , '\u{0288}'),
       ( "d`"      , '\u{0256}'),
       ( "n`"      , '\u{0273}'),
       ( "s`"      , '\u{0282}'),
       ( "z`"      , '\u{0290}'),
       ( "r`"      , '\u{027D}'),
       ( "r\\`"    , '\u{027B}'),
       ( "l`"      , '\u{026D}'),
       ( "c"       , 'c'),
       ( "J\\"     , '\u{025F}'),
       ( "f_<"     , '\u{0284}'),
       ( "J"       , '\u{0272}'),
       ( "s\\"     , '\u{0255}'),
       ( "z\\"     , '\u{0291}'),
       ( "C"       , '\u{00E7}'),
       ( "j\\"     , '\u{029D}'),
       ( "L"       , '\u{028E}'),
       ( "j"       , 'j'),
       ( "k"       , 'k'),
       ( "g"       , 'g'),
       ( "g_<"     , '\u{0260}'),
       ( "x"       , 'x'),
       ( "G"       , '\u{0263}'),
       ( "M\\"     , '\u{0270}'),
       ( "N"       , '\u{014B}'),
       ( "L\\"     , '\u{029F}'),
       ( "q"       , 'q'),
       ( "G\\"     , '\u{0262}'),
       ( "G_<"     , '\u{029B}'),
       ( "R"       , '\u{0281}'),
       ( "R\\"     , '\u{0280}'),
       ( "X"       , '\u{03C7}'),
       ( "N\\"     , '\u{0274}'),
       ( "?"       , '\u{0294}'),
       ( "h"       , 'h'),
       ( "h\\"     , '\u{0266}'),
       ( "?\\"     , '\u{0295}'),
       ( "X\\"     , '\u{0127}'),
       ( ">\\"     , '\u{02A1}'),
       ( "<\\"     , '\u{02A2}'),
       ( "H\\"     , '\u{029C}'),
       ( "5"       , '\u{026B}'),
       ( "l\\"     , '\u{027A}'),
       ( "w"       , 'w'),
       ( "x\\"     , '\u{0267}'),
       ( "H"       , '\u{0265}'),
       ( "W"       , '\u{028D}'),
       ( "i"       , 'i'),
       ( "y"       , 'y'),
       ( "M"       , '\u{026F}'),
       ( "u"       , 'u'),
       ( "}"       , '\u{0289}'),
       ( "1"       , '\u{0268}'),
       ( "I"       , '\u{026A}'),
       ( "Y"       , '\u{028F}'),
       ( "U"       , '\u{028A}'),
       ( "e"       , 'e'),
       ( "2"       , '\u{00F8}'),
       ( "7"       , '\u{0264}'),
       ( "o"       , 'o'),
       ( "@"       , '\u{0259}'),
       ( "8"       , '\u{0275}'),
       ( "@\\"     , '\u{0258}'),
       ( "@`"      , '\u{025A}'),
       ( "E"       , '\u{025B}'),
       ( "9"       , '\u{0153}'),
       ( "V"       , '\u{028C}'),
       ( "O"       , '\u{0254}'),
       ( "3"       , '\u{025C}'),
       ( "3\\"     , '\u{025E}'),
       ( "{"       , '\u{00E6}'),
       ( "6"       , '\u{0250}'),
       ( "a"       , 'a'),
       ( "&"       , '\u{0276}'),
       ( "A"       , '\u{0251}'),
       ( "Q"       , '\u{0252}'),
       ( "P"       , '\u{028B}'),
       ( "O\\"     , '\u{0298}'),
       ( "!\\"     , '\u{01C3}'),
       ( "|\\"     , '\u{01C0}'),
       ( "|\\|\\"  , '\u{01C1}'),
       ( "=\\"     , '\u{01C2}'),
       ( "t_s"     , '\u{02A6}'),
       ( "d_z"     , '\u{02A3}'),
       ( "t_S"   , 'ʧ',),
        ( "d_Z"   , 'ʤ'),
        ("t_s\\" , 'ʨ'),
        ("d_z\\" , 'ʥ'),

       ( "~"       , '\u{0303}'),
       ( "="       , '\u{0329}'),
       ( ":"       , '\u{02D0}'),
       ( "_\""     , '\u{0308}'),
       ( "_+"      , '\u{031F}'),
       ( "_-"      , '\u{0320}'),
       ( "_/"      , '\u{030C}'),
       ( "_0"      , '\u{0325}'),
       ( "_~"      , '\u{0303}'),
       ( "_="      , '\u{0329}'),
       ( "_>"      , '\u{02BC}'),
       ( "_?\\"    , '\u{02E4}'),
        //"_U"
       ( "_\\"     , '\u{0302}'),
       ( "_^"      , '\u{032F}'),
       ( "_}"      , '\u{031A}'),
       ( "_A"      , '\u{0318}'),
       ( "_a"      , '\u{033A}'),
       ( "_B"      , '\u{030F}'),
       ( "_B_L"     , '\u{1DC5}'),
       ( "_c"      , '\u{031C}'),
       ( "_d"      , '\u{032A}'),
       ( "_e"      , '\u{0334}'),
       ( "<F>"     , '\u{2198}'),
       ( "_F"      , '\u{0302}'),
       ( "_G"      , '\u{02E0}'),
       ( "_H"      , '\u{0301}'),
       ( "_H_T"    , '\u{1DC4}'),
       ( "_h"      , '\u{02B0}'),
       ( "_j"      , '\u{02B2}'),
       ( "_k"      , '\u{0330}'),
       ( "_L"      , '\u{0300}'),
       ( "_l"      , '\u{02E1}'),
       ( "_M"      , '\u{0304}'),
       ( "_P"      , '\u{033B}'), //Laminal
       ( "_m"      , '\u{1d50}'), //m superscript
       ( "_p"      , '\u{033C}'), //Linguolabial
       ( "_N"      , '\u{1d51}'), //Linguolabial
       ( "_n"      , '\u{207F}'), //n superscript
       ( "_O"      , '\u{0339}'),
       ( "_o"      , '\u{031E}'),
       ( "_q"      , '\u{0319}'),
       ( "<R>"     , '\u{2197}'),
       ( "_R"      , '\u{030C}'),
       ( "_R_F"    , '\u{1DC8}'),
       ( "_r"      , '\u{031D}'),
       ( "_T"      , '\u{030B}'),
       ( "_t"      , '\u{0324}'),
       ( "_v"      , '\u{032C}'),
       ( "_w"      , '\u{02B7}'),
       ( "_x"      , '\u{033D}'),
       ( "_X"      , '\u{0306}'),
       ( "."       , '\u{002E}'),
       ("\""      , '\u{02C8}'),
       ( "%"       , '\u{02CC}'),
       ( ":\\"     , '\u{02D1}'),
        //"="),
        //"*"),
        //"<"),
        //">"),
       ( "^"       , '\u{2191}'),
       ( "!"       , '\u{2193}'),
       ( "|"       , '\u{007C}'),
       ( "||"      , '\u{2016}'),
        //"t_K"   , '\u{026C}'),
        //"d_K\\" , '\u),
    ];
//left tells if the input is a valid token, right tells if it is a non-segmental token
pub fn token_type(_s: &str) -> (bool,bool) {
    let mut i: usize = 0;
    for (x,_) in TOKEN_TRANS_LIST {
        if x == _s {return (true,i>114)}
    i+=1;
    }
    (false,false)
}

//fn is_token(_s: &str) -> bool {
//
//    for (x,_) in TOKEN_TRANS_LIST {
//        if x == _s {return true}
//    }
//    false
//}
//
//fn is_non_segmental_token(_s: &str) -> bool {
//    for (x,_) in NON_SEGMENTAL_TOKENS {
//        if x == _s { return true }
//    }
//    false
//}

pub fn is_non_segmental_char(c: char) -> bool {
    for (_,y) in NON_SEGMENTAL_TOKENS {
        if y == c { return true }
    }
    false
}



static NON_SEGMENTAL_TOKENS: [(&str,char); 56] = [
       ( "~"       , '\u{0303}'),
       ( "="       , '\u{0329}'),
       ( ":"       , '\u{02D0}'),
       ( "_\""     , '\u{0308}'),
       ( "_+"      , '\u{031F}'),
       ( "_-"      , '\u{0320}'),
       ( "_/"      , '\u{030C}'),
       ( "_0"      , '\u{0325}'),
       ( "_="      , '\u{0329}'),
       ( "_>"      , '\u{02BC}'),
       ( "_?\\"    , '\u{02E4}'),
        //"_U"
       ( "_\\"     , '\u{0302}'),
       ( "_^"      , '\u{032F}'),
       ( "_}"      , '\u{031A}'),
       ( "_A"      , '\u{0318}'),
       ( "_a"      , '\u{033A}'),
       ( "_B"      , '\u{030F}'),
       ( "_B_L"    , '\u{1DC5}'),
       ( "_c"      , '\u{031C}'),
       ( "_d"      , '\u{032A}'),
       ( "_e"      , '\u{0334}'),
       ( "<F>"     , '\u{2198}'),
       ( "_F"      , '\u{0302}'),
       ( "_G"      , '\u{02E0}'),
       ( "_H"      , '\u{0301}'),
       ( "_H_T"    , '\u{1DC4}'),
       ( "_h"      , '\u{02B0}'),
       ( "_j"      , '\u{02B2}'),
       ( "_k"      , '\u{0330}'),
       ( "_L"      , '\u{0300}'),
       ( "_l"      , '\u{02E1}'),
       ( "_M"      , '\u{0304}'),
       ( "_m"      , '\u{033B}'),
       ( "_N"      , '\u{033C}'),
       ( "_n"      , '\u{207F}'),
       ( "_O"      , '\u{0339}'),
       ( "_o"      , '\u{031E}'),
       ( "_q"      , '\u{0319}'),
       ( "<R>"     , '\u{2197}'),
       ( "_R"      , '\u{030C}'),
       ( "_R_F"    , '\u{1DC8}'),
       ( "_r"      , '\u{031D}'),
       ( "_T"      , '\u{030B}'),
       ( "_t"      , '\u{0324}'),
       ( "_v"      , '\u{032C}'),
       ( "_w"      , '\u{02B7}'),
       ( "_x"      , '\u{033D}'),
       ( "_X"      , '\u{0306}'),
       ( "."       , '\u{002E}'),
       ( "\""      , '\u{02C8}'),
       ( "%"       , '\u{02CC}'),
       ( ":\\"     , '\u{02D1}'),
        //"="),
        //"*"),
        //"<"),
        //">"),
       ( "^"       , '\u{2191}'),
       ( "!"       , '\u{2193}'),
       ( "|"       , '\u{007C}'),
       ( "||"      , '\u{2016}'),
];

