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

        if chars[0] as u8 == chars[0] as u8 - 23u8 {
            points += 3;
        } else if 
            chars[0] == 'A' && chars[0] == 'Y'
            || chars[0] == 'B' && chars[0] == 'Z'
            || chars[0] == 'C' && chars[0] == 'X'
        {
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
        
        if chars[1] == 'X' {
            if chars[0] == 'A' { player = 'Z' }
            if chars[0] == 'B' { player = 'X' }
            if chars[0] == 'C' { player = 'Y' }
        } else if chars[1] == 'Y' {
            if chars[0] == 'A' { player = 'X' }
            if chars[0] == 'B' { player = 'Y' }
            if chars[0] == 'C' { player = 'Z' }
        } else {
            if chars[0] == 'A' { player = 'Y' }
            if chars[0] == 'B' { player = 'Z' }
            if chars[0] == 'C' { player = 'X' }
        }

        if chars[0] as u8 == player as u8 - 23u8 {
            points += 3;
        } else if 
            chars[0] == 'A' && player == 'Y'
            || chars[0] == 'B' && player == 'Z'
            || chars[0] == 'C' && player == 'X'
        {
            points += 6
        }
    
        points += (player as u32) - 87
    }

    println!("{}", points);
}
