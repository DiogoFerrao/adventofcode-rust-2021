#![allow(dead_code)]

use std::fs;

struct Position {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

pub fn part1(){
    let mut position = Position { horizontal: 0, depth: 0 , aim: 0};

    let filename = "lib/day2.txt";

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    for line in contents.lines(){
        let distance = line.split_whitespace().collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
        if line.contains("forward"){
            position.horizontal += distance;
        } else if line.contains("down"){
            position.depth += distance;
        } else if line.contains("up"){
            position.depth -= distance;
        }
    }

    println!("Part1: {}", position.horizontal * position.depth);

}

pub fn part2(){
    let mut position = Position { horizontal: 0, depth: 0 , aim: 0};

    let filename = "lib/day2.txt";

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    for line in contents.lines(){
        let distance = line.split_whitespace().collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
        if line.contains("forward"){
            position.horizontal += distance;
            position.depth += position.aim * distance;
        } else if line.contains("down"){
            position.aim += distance;
        } else if line.contains("up"){
            position.aim -= distance;
        }
    }
    println!("Part2: {}", position.horizontal * position.depth);
}