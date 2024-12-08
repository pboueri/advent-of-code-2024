use std::fs;
use clap::Parser;
use log::{self, debug, info, trace};
use itertools::Itertools;


/*
The smart way to do this, I think, is to:
1. Memorize parts of the equations (?)
2. Do a binary search of some sort where you sort the potential equations by how much the operator will increase.
Since left to right implies that order does not matter for which sign you choose. So something like 
[8,9,1,6] would mean that an order of choice of [0,0,0,0] = ["+","+","*","+"] would be the minimum amd you know that 
[3,4,1,2] is the order of increase. So you evaluation [0,0,0,0] if it's too high, fail, if it's too low, then increment half
like so [0,0,1,1] and if that's too high, then [0,0,0,1]. If that's too high then [0,0,1,0] and if that's too low if fails

I need a way to translate the bianry representation of 3 = 1100 = [0,0,1,1] = ["+","+","+","*"]

*/

#[derive(Debug, Clone)]
struct Equation {
    target: i64,
    first_value: i64,
    remaining_values: Vec<i64>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Operations {
    Add,
    Multiply,
    Concatenate,
}
// Consider implemneting look up for minimum value here

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    file_path: String,
    target_number: Option<i64>,
}

fn equation_to_string(equation: &Equation, operations: &Vec<Operations>) -> String {
    let mut result = equation.first_value.to_string();
    for (index, operation) in operations.iter().enumerate() {
        let part = equation.remaining_values[index];
        if *operation == Operations::Add {
            result += " + ";
            
        } else if *operation == Operations::Multiply {
            result += " * ";
        } else {
            result += " || ";
        }
        result += &part.to_string();
    }
    result
}

fn compute_operations(equation: &Equation, operations: &Vec<Operations>) -> i64 {
    let mut result = equation.first_value;
    for (index, operation) in operations.iter().enumerate() {
        let part = equation.remaining_values[index];
        match *operation {
            Operations::Add => {
                result += part;
            }
            Operations::Multiply => {
                result *= part;
            }
            Operations::Concatenate => {
                let mut new_value = result.to_string();
                new_value += &part.to_string();
                result = new_value.parse::<i64>().unwrap();
            }
        }
    }
    result
}

fn calculate_rank(parts: &Vec<i64>) -> Vec<usize> {
    let mut indexed_parts: Vec<(usize, &i64)> = parts.iter().enumerate().collect();
    indexed_parts.sort_by(|a, b| a.1.cmp(b.1));
    
    let mut ranks = vec![0; parts.len()];
    for (rank, (index, _)) in indexed_parts.iter().enumerate() {
        ranks[*index] = rank;
    }
    trace!("\nRanks: {ranks:?}\nParts: {parts:?}");
    ranks
}

fn number_to_rank_boolean(number: i32, length: i32, ranks: Vec<usize>) -> Vec<bool> {
    let mut result = vec![false; length as usize];
    let mut num = number;
    
    for i in 0..length {
        result[i as usize] = num & 1 == 1;
        num >>= 1;
    }
    let mut ordered_result = vec![false; length as usize];
    for (index, rank) in ranks.iter().enumerate() {
        ordered_result[*rank] = result[index];
    }
    ordered_result
}

fn map_to_operations(bools: &Vec<bool>, parts: Vec<i64>, first_value: i64) -> Vec<Operations> {
    let mut operations = Vec::new();
    for (index, part) in parts.iter().enumerate() {
        if !bools[index] {
            if (*part == 1 )|| (first_value == 1 && index==0){
                operations.push(Operations::Multiply);    
            }else{
                operations.push(Operations::Add);
            }
            
        } else {
            if (*part  == 1) || (first_value == 1 && index==0){
                operations.push(Operations::Add);
            } else{
                operations.push(Operations::Multiply);
            }
        }
    }
    
    operations
}


fn search_for_solution_no_concatenate(equation: Equation) -> bool{
    // https://stackoverflow.com/a/75693863
    let num_operations = equation.remaining_values.len();
    let total_combinations = (2 as i32).pow(num_operations as u32);
    let mut left = 0;
    let mut right: i32 = total_combinations as i32-1;
    
    let parts_rank = calculate_rank(&equation.remaining_values);
    // Unfortunately I can't figure out how to order the precedence of the operations in terms of their importants.
    // I think there's probably a way to do it based on both operands, but unfortunately i ran out of time. Brute forcing it.
    // while left <= right {
    //     let mid = left + (right - left) / 2;
    //     let bools = number_to_rank_boolean(mid, num_operations as i32, parts_rank.clone());
    //     let operations: Vec<Operations> = map_to_operations(&bools, equation.remaining_values.clone(), equation.first_value);
    //     let result = compute_operations(&equation, &operations);
    //     let target = equation.target;
    //     let equation_string =  equation_to_string(&equation, &operations);
    //     trace!("Equation: {equation_string}={result} -->  {target}\n\tOperations: {operations:?}\n\t{parts_rank:?}\n\t{bools:?}\n\t total:{total_combinations} left:{left} right:{right} mid:{mid}");
    //     if  result == equation.target {
    //         debug!("Found solution: {:?}, {:?}", result, operations);
    //         return true;
    //     } else if result > equation.target{
    //         right = mid - 1;
    //     } else {
    //         left = mid + 1;
    //     }
    //     if left > right {
            
    //         debug!("No solution found: {target} ({result}): {left}>{right} ({total_combinations}). Operations {:?}", operations);
    //     }
    // }
    for i in 0..total_combinations {
        
        let bools = number_to_rank_boolean(i, num_operations as i32, parts_rank.clone());
        let operations: Vec<Operations> = map_to_operations(&bools, equation.remaining_values.clone(), equation.first_value);
        let result = compute_operations(&equation, &operations);
        let target = equation.target;
        let equation_string =  equation_to_string(&equation, &operations);
        trace!("Equation: {equation_string}={result} -->  {target}\n\tOperations: {operations:?}\n\t{parts_rank:?}\n\t{bools:?}\n\t total:{total_combinations}");
        if  result == equation.target {
            debug!("Found solution: {:?}, {:?}", result, operations);
            return true;
        }
    }
    false
}   


fn generate_operation_combinations(length: usize) -> Vec<Vec<Operations>> {
    let total_combinations = 3_usize.pow(length as u32);
    let mut result = Vec::with_capacity(total_combinations);
    
    for i in 0..total_combinations {
        let mut combination = Vec::with_capacity(length);
        let mut num = i;
        
        for _ in 0..length {
            let operation = match num % 3 {
                0 => Operations::Add,
                1 => Operations::Multiply,
                2 => Operations::Concatenate,
                _ => unreachable!()
            };
            combination.push(operation);
            num /= 3;
        }
        
        result.push(combination);
    }
    
    result
}

fn search_for_solution_with_concatenate(equation: Equation) -> bool{
    let num_operations = equation.remaining_values.len();
    let total_combinations = (3 as i32).pow(num_operations as u32);

    
    for operations in generate_operation_combinations(num_operations) {
        let result = compute_operations(&equation, &operations);
        let target = equation.target;
        let equation_string =  equation_to_string(&equation, &operations);
        trace!("Equation: {equation_string}={result} -->  {target}\n\tOperations: {operations:?}");
        if  result == equation.target {
            debug!("Found solution: {:?}: {}", result, equation_to_string(&equation, &operations));
            return true;
        }
    }
    false
}



fn get_equations(file_path: String) -> Vec<Equation>{
    let mut equations = Vec::new();
    let raw_data = fs::read_to_string(file_path).expect("Failed to read file");
    for line in raw_data.lines() {
        let parts: Vec<&str> = line.split(": ").collect();
        let target = parts[0].parse::<i64>().unwrap();
        let parts_str: Vec<&str> = parts[1].split_ascii_whitespace().collect();
        let mut parts: Vec<i64> = Vec::new();
        for part in parts_str.iter() {
            parts.push(part.parse::<i64>().unwrap());
        }
        let first_value = parts[0];
        let remaining_values = parts[1..].to_vec();
        equations.push(Equation {target, first_value, remaining_values });
    }
    equations
}




fn main(){
    env_logger::init();

    let args = Cli::parse();
    let file_path = args.file_path;
    let mut equations = get_equations(file_path);
    let mut sum = 0;
    if args.target_number.is_some() {
        equations.retain(|equation| equation.target == args.target_number.unwrap());
    }
    for equation in equations.iter() {
        if search_for_solution_no_concatenate(equation.clone()){
            sum += equation.target;
        }
    }
    info!("Answer 1: {}", sum);
    let mut concact_sum = 0;
    for equation in equations.iter() {
        if search_for_solution_with_concatenate(equation.clone()){
            concact_sum += equation.target;
        }
    }
    
    info!("Answer 2: {}", concact_sum);
}