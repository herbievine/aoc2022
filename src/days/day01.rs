use crate::utils;
use std::{fmt::Debug, path::Path};

pub fn a<P>(input: P)
where
    P: AsRef<Path> + Debug,
{
    let mut calories: Vec<u32> = vec![0];
    let mut arr_idx = 0;

    for line in utils::read_lines(input).unwrap() {
        let line_data = line.unwrap();

        if line_data.len() > 0 {
            calories[arr_idx] += line_data.parse::<u32>().unwrap();
        } else {
            calories.push(0);
            arr_idx += 1;
        };
    }

    calories.sort();

    println!("{}", calories.last().unwrap())
}

pub fn b<P>(input: P)
where
    P: AsRef<Path> + Debug,
{
    let mut calories: Vec<u32> = vec![0];
    let mut arr_idx = 0;

    for line in utils::read_lines(input).unwrap() {
        let line_data = line.unwrap();

        if line_data.len() > 0 {
            calories[arr_idx] += line_data.parse::<u32>().unwrap();
        } else {
            calories.push(0);
            arr_idx += 1;
        };
    }

    calories.sort();
    calories.reverse();

    println!("{}", calories[0] + calories[1] + calories[2])
}
