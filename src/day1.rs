#![allow(dead_code)]

use std::fs;

pub fn part1(){
    let filename = "lib/day1.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut pre_depth = 0;
    let mut depth;
    let mut depth_increase : i32 = 0;
    for line in contents.lines() {
        depth = line.parse::<i32>().unwrap();
        if depth > pre_depth {
            depth_increase += 1;
        }
        pre_depth = depth;
    }
    println!("Part1: {}", depth_increase);
}

pub fn part2(){
    let filename = "lib/day1.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut depth_increase : i32 = 0;
    let mut sum;
    let mut lines = contents.lines();


    let line1 : i32 = lines.next().unwrap().parse::<i32>().unwrap();
    let line2 : i32 = lines.next().unwrap().parse::<i32>().unwrap();
    let line3 : i32 = lines.next().unwrap().parse::<i32>().unwrap();

    let mut pre_sum : i32 = line1 + line2 + line3;

    let mut current_lines = vec![line2, line3];

    for line in contents.lines() {
        current_lines.push(line.parse::<i32>().unwrap());

        sum = current_lines.iter().sum();

        if sum > pre_sum {
            depth_increase += 1;
        }

        pre_sum = sum;
        current_lines.remove(0);
    }

    println!("Part2: {}", depth_increase);
}