use std::fs;
use clap::Parser;
use log::{self, debug, error, info};


/*
*/

const MAX_PRESSES: i32 = 100;
const COST_A: i32 = 3;
const COST_B: i32 = 1;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Robot {
    location: (i32, i32),
    velocity: (i32, i32),
}
impl Robot {
    fn simulate(&self, time: i32, map_size: (i32, i32)) -> (i32, i32) {
        ((self.location.0 + self.velocity.0 * time).rem_euclid(map_size.0), (self.location.1 + self.velocity.1 * time).rem_euclid(map_size.1))
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    file_path: String,
}

fn read(file_path: String) -> Vec<Robot> {
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let mut problems = Vec::new();
    let number_regex = regex::Regex::new(r"(-?\d+)").unwrap();
    for line in contents.lines(){
        let numbers = number_regex.find_iter(line).map(|x| x.as_str().parse::<i32>().unwrap()).collect::<Vec<i32>>();
        problems.push(Robot {
            location: (numbers[0], numbers[1]),
            velocity: (numbers[2], numbers[3]),
        });
        
    }
    problems
}

fn main(){
    env_logger::init();

    let args = Cli::parse();
    let file_path = args.file_path;
    let robots = read(file_path);

    info!("Read {} robots", robots.len());
    info!("{}", -5 % 2);
    let mut quadrant = (0, 0 ,0 ,0 );
    let map_size = (101, 103);
    let mid_y = map_size.1/2;
    let mid_x = map_size.0/2;
    info!("Mid point: {:?}", (mid_x, mid_y));   
    for robot in robots.iter() {
        let point = robot.simulate(100, map_size);
        debug!("Robot at {:?}", point);
        match point {
            (x, y) if x < mid_x && y < mid_y => quadrant.0 += 1,
            (x, y) if x > mid_x && y < mid_y => quadrant.1 += 1,
            (x, y) if x > mid_x && y > mid_y => quadrant.2 += 1,
            (x, y) if x < mid_x && y > mid_y => quadrant.3 += 1,
            _ => {}
        }

    }
    info!("Answer 1: {:?}", quadrant.0 * quadrant.1 * quadrant.2 * quadrant.3);

    
    
}