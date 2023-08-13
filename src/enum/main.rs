#![allow(unused_imports)]
#![allow(unused)]
#![allow(dead_code)]

use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    
    enum Day {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday
    }

    impl Day {
        fn is_weekend(&self) -> bool {
            match self {
                Day::Sunday | Day::Saturday => true,
                _ => false
            }
        } 
    }

    let today:Day = Day::Monday;
    println!("{}", today.is_weekend());
}
