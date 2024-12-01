use std::fs;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct CLI {
    file_path: String
}



fn get_sorted_lists (file_path: String) -> (Vec<i32>, Vec<i32>) {
    
    let raw_data = fs::read_to_string(file_path).expect("Failed to read file");
    
    let mut list_1 = Vec::<i32>::new();
    let mut list_2 = Vec::<i32>::new();
    for line in raw_data.lines() {
        let mut nums = line.split_whitespace().map(|x| {
            x.parse::<i32>().expect("Failed to parse number")
        });
        list_1.push(nums.next().unwrap());
        list_2.push(nums.next().unwrap());
    }
    
    list_1.sort();
    list_2.sort();
    (list_1, list_2)
}

fn solve_puzzle_one(list_1: &Vec<i32>, list_2: &Vec<i32>) -> i32 {
    let mut diff = 0;
    for (num1, num2) in list_1.iter().zip(list_2.iter()) {
        diff += (num1 - num2).abs();
    }
    diff
}

fn solve_puzzle_two(list_1: &Vec<i32>, list_2: &Vec<i32>) -> i32 {
    let mut sim_score = 0;
    for num1  in  list_1.iter() {
        let mut multiplier = 0;
        for num2 in list_2.iter(){
            if num1 == num2 {
                multiplier += 1;
            }
        }
        sim_score += multiplier*num1;
    }
    sim_score
}

fn main(){
    let args = CLI::parse();
    let (list_1, list_2) = get_sorted_lists(args.file_path);
    println!("Answer 1: {}", solve_puzzle_one(&list_1, &list_2));
    println!("Answer 2: {}", solve_puzzle_two(&list_1, &list_2));
}