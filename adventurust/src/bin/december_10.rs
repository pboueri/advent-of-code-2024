use std::{collections::HashMap, fs, collections::HashSet};
use clap::Parser;
use log::{self, debug, info};
use itertools::enumerate;

/*
Thinking through the problem:
- For each trailhead find all the viable paths
- If a viable path intersects with an existing viable path we can ignore it
- Once a summit is found we keep track of it. It's a recursive problem where we keep track of:
    - possible next states
    - visited states
    - summits hit

Approach:
1. For each trailhead
    - Iterate through all possible next states
    - If a next state is in a visited state, return 0
    - If a next state is a summit, return the summit
*/
const TRAILHEAD: i32 = 0;
const SUMMIT: i32 = 9;



#[derive(Parser, Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
   fn add(&self, other: &Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

}

type TrailMapLinks = HashMap<Point, HashSet<Point>>;

struct TrailMap {
    ascents: TrailMapLinks,
    descents: TrailMapLinks,
    raw_map: Vec<Vec<i32>>,
    summits: HashSet<Point>,
    trailheads: HashSet<Point>,

}

impl TrailMap {
    fn print(&self) {
        for line in &self.raw_map {
            for value in line {
                print!("{}", value);
            }
            println!();
        }
    }
    
    fn get_height(&self, point: &Point) -> i32 {
        self.raw_map[point.y as usize][point.x as usize]
    }
}


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    file_path: String,
    target_number: Option<i64>,
}

fn build_map(file_map: &Vec<Vec<i32>>) -> TrailMap {
    let mut ascents = HashMap::new();
    let mut descents = HashMap::new();
    let mut summits = HashSet::new();
    let mut trailheads = HashSet::new();

    for (y, line) in file_map.iter().enumerate() {
        for (x, current_height) in line.iter().enumerate() {
            let point = Point { x: x as i32, y: y as i32 };
            let mut ascents_for_point = HashSet::new();
            let mut descents_for_point = HashSet::new();
            if current_height == &TRAILHEAD {
                trailheads.insert(point.clone());
            }
            if current_height == &SUMMIT {
                summits.insert(point.clone());
            }
            
            for movement in &[
                Point { x: 0, y: -1 },
                Point { x: 0, y: 1 },
                Point { x: -1, y: 0 },
                Point { x: 1, y: 0 },
            ] {
                let new_point = point.add(movement);
                if new_point.x < 0 || new_point.y < 0  {
                    continue;
                }
                if let Some(line) = file_map.get(new_point.y as usize) {
                    if let Some(new_height) = line.get(new_point.x as usize) {
                        if *new_height==current_height+1 {
                            ascents_for_point.insert(new_point);
                        }
                    }
                }
                if let Some(line) = file_map.get(new_point.y as usize) {
                    if let Some(new_height) = line.get(new_point.x as usize) {
                        if *new_height==current_height-1 {
                            descents_for_point.insert(new_point);
                        }
                    }
                }
            }
            ascents.insert(point, ascents_for_point);
            descents.insert(point, descents_for_point);
        }
    }
    TrailMap {
        ascents: ascents,
        descents: descents,
        raw_map: file_map.clone(),
        summits,
        trailheads,
    }
}

fn get_file_map(file_path: String) -> TrailMap {
   let lines = fs::read_to_string(file_path).expect("Failed to read file");
   let mut file_map = Vec::new();
   for line in lines.lines() {
       let mut line_vec = Vec::new();
       for c in line.chars() {
           line_vec.push(c.to_digit(10).unwrap() as i32);
       }
       file_map.push(line_vec);
   }

   build_map(&file_map)

}

fn paths_to_summit(current_point: &Point, visited_points: &mut HashSet<Point>, trail_map: &TrailMap) -> i32 {
    visited_points.insert(current_point.clone());
    debug!("Visited Points: {:?}: {:?}", visited_points.len(), current_point);
    if trail_map.summits.contains(&current_point) {
        debug!("Found summit: {:?}", current_point);
        return 1;
    }
    let next_points = trail_map.ascents.get(&current_point);
    if next_points.is_none() {
        return 0;
    }
    let mut sum = 0;
    debug!("\t Next points: {:?}", next_points);
    for possible_next_point in next_points.unwrap() {
        if visited_points.contains(possible_next_point) {
            debug!("Already visited point: {:?}", possible_next_point);
            continue;
        }
        sum += paths_to_summit(possible_next_point, visited_points, trail_map);
    }
    sum
}



fn trail_rating(current_point: &Point, trailhead_ratings: &mut HashMap<Point, i32>, trail_map: &TrailMap) {
    if trail_map.trailheads.contains(&current_point) {
        debug!("Found trailhead: {:?}", current_point);
        *trailhead_ratings.get_mut(current_point).unwrap()+=1;
    }
    let next_points = trail_map.descents.get(&current_point);
    if next_points.is_none() {
        return;
    }
    
    debug!("\t Next points: {:?}", next_points);
    for possible_next_point in next_points.unwrap() {
        trail_rating(possible_next_point, trailhead_ratings, trail_map);
    }
    
}

fn main(){
    env_logger::init();

    let args = Cli::parse();
    let file_path = args.file_path;
    let file_map = get_file_map(file_path);
    
    //file_map.print();
    debug!("Trailheads: {:?}", file_map.trailheads);
    debug!("Summits: {:?}", file_map.summits);
    debug!("Ascents: {:?}", file_map.ascents.iter().map(|x| x.1.len()).sum::<usize>());
    let mut total_trail_score = 0;
    for trailhead in file_map.trailheads.iter() {
        let mut visited_points: HashSet<Point> = HashSet::new();
        let trail_score = paths_to_summit(trailhead, &mut visited_points, &file_map);
        total_trail_score += trail_score;
        debug!("Trailhead: {:?} has {} paths to summit", trailhead, trail_score);
    }
    info!("Total trail score: {}", total_trail_score);

    let mut trailhead_ratings: HashMap<Point,i32> = HashMap::new();
    for trailhead in file_map.trailheads.iter() {
       trailhead_ratings.insert(trailhead.clone(),0 as i32);
    }
    for summit in file_map.summits.iter() {
        trail_rating(summit, &mut trailhead_ratings, &file_map);
    }
    info!("Trailhead Ratings: {:?}", trailhead_ratings.iter().map(|x| x.1).sum::<i32>());

}