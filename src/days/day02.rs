use crate::utils;
use std::{fmt::Debug, path::Path};

pub fn a<P>(input: P)
where
    P: AsRef<Path> + Debug,
{
    let mut points = 0;

    for line in utils::read_lines(input).unwrap() {
        let line_data = line.unwrap();
        let split: Vec<&str> = line_data.split(' ').collect();
        let chars = split.iter().map(|str| str.chars().nth(0).unwrap()).collect::<Vec<char>>();

        if (chars[0] as u8) == (chars[1] as u8 - 23u8) {
            points += 3;
        } else if (
            (chars[0] == 'A' && chars[1] == 'Y')
            || (chars[0] == 'B' && chars[1] == 'Z')
            || (chars[0] == 'C' && chars[1] == 'X')
        ) {
            points += 6
        }
    
        points += (chars[1] as u32) - 87
    }

    println!("{}", points);
}

pub fn b<P>(input: P)
where
    P: AsRef<Path> + Debug,
{
    let mut points = 0;

    for line in utils::read_lines(input).unwrap() {
        let line_data = line.unwrap();
        let split: Vec<&str> = line_data.split(' ').collect();
        let chars = split.iter().map(|str| str.chars().nth(0).unwrap()).collect::<Vec<char>>();
        let mut player: char = '0';

        if (chars[1] == 'X') {
            player = if chars[0] == 'C' { 'X' } else { (chars[0] as u8 + 24u8) as char }
        } else if (chars[0] == 'Y') {
            player = (chars[0] as u8 + 23u8) as char
        } 

        println!("elf {}\noutput {}\nplayer {}", chars[0], chars[1], player);
        
        // if (chars[0] as u8) == (chars[1] as u8 - 23u8) {
        //     points += 3;
        // } else if (
        //     (chars[0] == 'A' && chars[1] == 'Y')
        //     || (chars[0] == 'B' && chars[1] == 'Z')
        //     || (chars[0] == 'C' && chars[1] == 'X')
        // ) {
        //     points += 6
        // }
    
        points += (chars[1] as u32) - 87
    }

    println!("{}", points);
}
