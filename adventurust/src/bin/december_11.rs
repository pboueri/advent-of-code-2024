use std::{collections::{HashMap, LinkedList}, fs, hash::Hash};
use clap::Parser;
use log::{self, debug, info};
use itertools::enumerate;

/*
Thinking through the problem:
-

Approach:
*/
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    file_path: String,
    num_steps: Option<i32>
}


#[derive(Parser, Debug, PartialEq, Eq, Hash, Clone)]
struct Stone {
    value: String,
}


fn trim_zeroes(input: &str) -> String {
    let x = input.trim_start_matches('0').to_string();
    if x.len() == 0{
        return  "0".to_string()
    }
    x
}

impl Stone {
    fn apply_rule(&self) -> (Stone, Option<Stone>) {
       match self.value.as_str() {
            "0" => (Stone { value: "1".to_string() }, None),
            // if it  is even then split in two and drop leading zeroes
            x if x.len() % 2 == 0 => (Stone { value: x[0..x.len()/2].to_string() }, Some(Stone { value: trim_zeroes(&x[x.len()/2..])})),
            x => (Stone { value:  (x.parse::<i64>().unwrap()*2024).to_string()}, None),
        }
        
    }
}

fn get_stones(file_path: String) -> LinkedList<Stone> {
    let line = fs::read_to_string(file_path).expect("Failed to read file");
    let mut stones = LinkedList::new();
    
    for number in line.split_whitespace(){
        stones.push_back(Stone { value: number.to_string() });
    }
    stones
}


fn blink(stones: &LinkedList<Stone>) -> LinkedList<Stone>{
    let mut new_stones = LinkedList::new();
    for stone in stones {
        let (new_stone, new_stone_2) = stone.apply_rule();
        new_stones.push_back(new_stone);
        if let Some(new_stone_2) = new_stone_2 {
            new_stones.push_back(new_stone_2);
        }
    }
    new_stones
}

fn print_stones(stones: &LinkedList<Stone>) {
    for stone in stones {
        print!("{} ", stone.value);
    }
    println!();
}

fn stone_steps(stone: &Stone, num_steps_left: i32, known_steps: & mut HashMap<(Stone, HashMap<i32, LinkedList<Stone>>>) -> LinkedList<Stone> {
    debug!("stone_steps: {:?} {:?}", stone, num_steps_left);
    if num_steps_left == 0 {
        let mut steps = LinkedList::new();
        steps.push_back(stone.clone());
        return steps
    }

    // TODO: instead of _exact_ steps, we can "fast forward" if we have known steps that are fewer than what has been seen (i.e. if it's 5 steps and we know 4, we just need to get 4 and do the fifth comp)
    if let Some(known_steps) = known_steps.get(&(stone.clone(), num_steps_left)) {
        return known_steps.clone();
    }
    let mut new_stones = LinkedList::new();
    let (new_stone, new_stone_2) = stone.apply_rule();
    new_stones.push_back(new_stone);
    if let Some(new_stone_2) = new_stone_2 {
        new_stones.push_back(new_stone_2);
    }
    let mut new_stones_2 = LinkedList::new();
    for new_stone in new_stones {
        let temp = stone_steps(&new_stone, num_steps_left-1, known_steps);
        for new_stone_2 in temp {
            new_stones_2.push_back(new_stone_2);
        }
    }
    known_steps.insert((stone.clone(), num_steps_left), new_stones_2.clone());

    new_stones_2
}

fn main(){
    env_logger::init();

    let args = Cli::parse();
    let file_path = args.file_path;

    let mut stones = get_stones(file_path);
    let steps = args.num_steps.or_else(|| Some(6)).unwrap();
    print_stones(&stones);

    let mut new_stones = LinkedList::new();
    let mut known_steps = HashMap::new();
    for stone in stones {
        new_stones.append(& mut stone_steps(&stone, steps, &mut known_steps));
    }
    info!("Answer 1: {:?}", new_stones.len());


}  