
use std::{
    fmt::{Debug, Display}, fs::{self, File}, io, iter::Peekable, ops::Deref, path::Path,
    io::prelude::*,
    
};
// use super::split_on::split_on;
use csv;

use itertools::Itertools;
use unicode_segmentation::UnicodeSegmentation;

/*
This is a badly implemented csv struct
I will use the proper library in the final product
But I like to do things myself before I do
*/

// pub struct CSV_ENTRY<const col_size: usize> {
//     entry: [String;col_size]
// }
//
// impl<const col_size: usize> CSV_ENTRY<col_size> {
//     pub fn contains_entry(self,token: &str,col: usize) -> Option<Self> {
//         if self.entry[col] == token {
//             Some(self)
//         } else {
//             None
//         }
//     }
//
// }

const SEP: char = '\t';

pub struct CSV {
    pub col_num: usize,
    pub fields: Vec<String>,
    pub rows: Vec<Vec<String>>
}

impl CSV {
    pub fn import<P: AsRef<Path>>(path: P) -> io::Result<CSV> {
        let raw_utf8 = fs::read(path)?;
        let raw_string = String::from_utf8(raw_utf8).expect("Invalid UTF8");
        Self::from_str(&raw_string)
    }
    pub fn export<P: AsRef<Path>>(self, path: P) -> io::Result<()> {
        let csv_string = self.to_str();

        let mut file = File::create(path)?;
        file.write_all(csv_string.as_bytes())?;
        Ok(())
    }

    fn from_str(raw_str: &str) -> io::Result<CSV> {
        let mut raw_str_iter = raw_str.lines();

        let field_str = raw_str_iter.next().ok_or(io::Error::other("Error: csv file empty"))?;

        let fields: Vec<String> = field_str.split(SEP).map(|str| str.to_string()).collect();
        let col_num = fields.len();
        
        let mut rows: Vec<Vec<String>> = Vec::new();
        for raw_row in raw_str_iter {
            let row: Vec<String> = raw_row.split(SEP).map(|str| str.to_string()).collect();
            rows.push(row);
        }
        
        Ok(CSV {col_num,fields,rows})
        
    } 
    fn to_str(self) -> String {
        let mut csv_str = self.fields[1..].iter().fold(self.fields[0].clone(), |x,y| x+"\t"+y); // replace "\t" with SEP
        csv_str.push('\n');

        for row in self.rows {
            let row_str = row[1..].iter().fold(row[0].clone(), |x,y| x+"\t"+y);
            csv_str.push_str(&row_str);
            csv_str.push('\n');
        }
        csv_str
    } 
}



struct IpaSymbolAssoc {
    ipa_char: char,
    xsampa_token: String,
    tipa_token: String,
}


 pub fn read_csv<P: AsRef<Path>>(path: P) -> io::Result<IpaSymbolAssoc> {
     let raw_utf8 = fs::read(path)?;
     let raw_string = String::from_utf8(raw_utf8).expect("Invalid UTF8");

     todo!()
 }
