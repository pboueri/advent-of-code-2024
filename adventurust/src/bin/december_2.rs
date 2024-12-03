use std::cmp;
use std::fs;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    file_path: String
}



fn get_list_of_levels (file_path: String) -> Vec<Vec<i32>> {
    
    let raw_data = fs::read_to_string(file_path).expect("Failed to read file");
    
    let mut lists = Vec::<Vec<i32>>::new();

    for line in raw_data.lines() {
        lists.push(line.split_whitespace().map(|x| {
            x.parse::<i32>().expect("Failed to parse number")
        }).collect());
    }
    
    lists
}

fn check_if_safe(levels: &[i32]) -> Option<usize> {
    let len = levels.len();
    let slice_1 = &levels[..len-1];
    let slice_2 =  &levels[1..];
    let ascending = (slice_1[0] - slice_2[0]) > 0;
    for i in 0..len-1 {
        let diff = slice_1[i] - slice_2[i];
        let monotonic_check = (ascending && diff > 0) || (!ascending && diff < 0);
        let step_check = (diff.abs() > 0) && (diff.abs() <= 3);
        if !monotonic_check || !step_check {
            return Some(i);
        } 
    }
    None
}

fn solve_puzzle_one(lists: &Vec<Vec<i32>>) -> i32 {
    let mut valid = 0;
    
    for levels in lists {
       let bad_index = check_if_safe(levels);
       if bad_index.is_none(){
            valid+=1;
       }
    }

    valid
}



fn solve_puzzle_two(lists: &Vec<Vec<i32>>) -> i32 {
    let mut valid = 0;
    
    for levels in lists {

        let bad_index= check_if_safe(levels);
        if bad_index.is_none(){
            
            valid +=1;
            continue;
        }

        // Try both the problem child and those around it
        let lower = if bad_index.unwrap() < 2 { 0 } else { bad_index.unwrap() - 2};
        let upper = cmp::min(bad_index.unwrap()+2,levels.len());
        for i in lower..upper{
            let mut levels_copy = levels.clone();
            levels_copy.remove(i);
            let second_try = check_if_safe(&levels_copy);
            if second_try.is_none(){
                valid +=1;
                break;
            } 
        }
    }
    valid
}

fn main(){
    let args = Cli::parse();
    let lists = get_list_of_levels(args.file_path);
    println!("First Line: {:?}", lists.first().unwrap());
    println!("Answer 1: {}", solve_puzzle_one(&lists));
    println!("Answer 2: {}", solve_puzzle_two(&lists));
}