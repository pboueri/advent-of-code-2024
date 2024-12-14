use std::{collections::HashSet, fs, hash::Hash};
use clap::Parser;
use log::{self, debug, info};
use itertools::Itertools;


/*
Here we want to create all the pairs of antennae of a specific signal.
We can discard pairs that are "too far" away from each other such that their antinodes are not within the map

Once all the pairs are found, calculating the number of antinodes is easy and should be a set so as not to double count.

*/

#[derive(Debug, Clone, PartialEq,  Eq, Hash)]
struct Coordinate {
    x: i32,
    y: i32,
}


#[derive(Debug, Clone)]
struct Antenna {
    location: Coordinate,
    frequency: char,
}

#[derive(Debug, Clone)]
struct Map {
    width: i32,
    height: i32,
    antennae: Vec<Antenna>,
    antinode_locations: HashSet<Coordinate>,
}

impl Map {
    fn bounds_check(&self, x: i32, y: i32) -> bool {
        !(x < 0 || x >= self.width || y < 0 || y >= self.height)
    }

    fn get_antenna_of_same_frequency(&self, frequency: char) -> Vec<&Antenna> {
        self.antennae.iter().filter(|antenna| antenna.frequency == frequency).collect()
    }

    fn get_all_frequencies(&self) -> Vec<char> {
        self.antennae.iter().map(|antenna| antenna.frequency).unique().collect()
    }

    fn draw_map(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let antenna: Vec<Antenna> = self.antennae.clone().into_iter().filter(|antenna| antenna.location.x == x && antenna.location.y == y).collect();
                if antenna.len() > 0 {
                    print!("{}", antenna[0].frequency);
                } else if self.antinode_locations.contains(&Coordinate { x, y }) {
                    print!("#");
                } else {
                    print!(".");
                }
        }
        println!();
    }
}

}

fn get_antinodes(antenna_1: &Antenna, antenna_2: &Antenna, map: &Map) -> Vec<Coordinate> {
    let x_delta = antenna_1.location.x - antenna_2.location.x;
    let y_delta = antenna_1.location.y - antenna_2.location.y;

    let mut antinodes = Vec::new();
    
    
    let antinode_one = (antenna_1.location.x + x_delta, antenna_1.location.y + y_delta);
    let antinode_two = (antenna_2.location.x - x_delta, antenna_2.location.y - y_delta);
    
    if map.bounds_check(antinode_one.0, antinode_one.1) {
        antinodes.push(Coordinate {
            x: antinode_one.0,
            y: antinode_one.1,
        });
    }
    if map.bounds_check(antinode_two.0, antinode_two.1) {
        antinodes.push(Coordinate {
            x: antinode_two.0,
            y: antinode_two.1,
        });
    }

    antinodes
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    file_path: String,
    target_number: Option<i64>,
}

fn get_map(file_path: String) -> Map {
    let mut antennae = Vec::new();
    let mut antinode_locations = HashSet::new();
    let raw_data = fs::read_to_string(file_path).expect("Failed to read file");


    for (y, line) in raw_data.lines().enumerate() {
      for (x, char) in line.chars().enumerate() {
        if char != '.' {
            antennae.push(Antenna {
                location: Coordinate {
                    x: x as i32,
                    y: y as i32,
                },
                frequency: char,
            });
        }
      }
    }
    Map {
        width: raw_data.lines().next().unwrap().len() as i32,
        height: raw_data.lines().count() as i32,
        antennae,
        antinode_locations,
    }
}

/*
Part 2 is intractable to do all pairwise combos. 

Instead we need to broadcast out from each frequency antenna, summing as we go.
Then for any cell > 2 we would add it as a new antinode.

*/

fn get_antinodes_all_distances(antenna_1: &Antenna, antenna_2: &Antenna, map: &Map) -> Vec<Coordinate> {
    let x_delta = antenna_1.location.x - antenna_2.location.x;
    let y_delta = antenna_1.location.y - antenna_2.location.y;

    let mut antinodes = Vec::new();
    
    
    let mut antinode_1: (i32, i32) = (antenna_1.location.x, antenna_1.location.y);
    while map.bounds_check(antinode_1.0, antinode_1.1) {
        antinodes.push(Coordinate {
            x: antinode_1.0,
            y: antinode_1.1,
        });
        antinode_1 = (antinode_1.0 + x_delta, antinode_1.1 + y_delta);
    }

    let mut antinode_2: (i32, i32) = (antenna_2.location.x, antenna_2.location.y);
    while map.bounds_check(antinode_2.0, antinode_2.1) {
        antinodes.push(Coordinate {
            x: antinode_2.0,
            y: antinode_2.1,
        });
        antinode_2 = (antinode_2.0 - x_delta, antinode_2.1 - y_delta);
    }


    antinodes
}


fn broadcast_frequencies(antennae: &Vec<Antenna>, map: &Map) -> Vec<Coordinate> {
    // To fix this we need to capture the distance to each array from each grid spot. 
    // the number of computations is grid cells * num_arrays in each frequency
    // If the distance is the same from two antenna then we consider that cell an antinode and move on
    // We can defray future costs by only computing grid nodes that are not antinodes

    let mut broadcast_array = vec![vec![HashSet::new() ;map.width as usize];map.height as usize];
    let mut antinode_locations = Vec::new();
    for y in 0..map.height {
        for x in 0..map.width {
            if map.antinode_locations.contains(&Coordinate { x, y }) {
                continue;
            }
            for antenna in antennae.iter() {
                let x_delta = (antenna.location.x - x).abs();
                let y_delta = (antenna.location.y - y).abs();
                if broadcast_array[y as usize][x as usize].contains(&(x_delta, y_delta)) {
                    antinode_locations.push(Coordinate { x, y });
                    break;
                }
                broadcast_array[y as usize][x as usize].insert((x_delta, y_delta));
            }

        }
    }
    antinode_locations
}

fn main(){
    env_logger::init();

    let args = Cli::parse();
    let file_path = args.file_path;
    let mut map = get_map(file_path);
    
    let frequencies = map.get_all_frequencies();
    let mut antinodes = HashSet::new();
    for frequency in frequencies.iter() {
        debug!("Frequency: {frequency}");
        let antennae = map.get_antenna_of_same_frequency(*frequency);
        for antennas in antennae.iter().combinations(2) {
            debug!("Antennas: {antennas:?}");
            let new_antinodes = get_antinodes(antennas[0], antennas[1], &map);
            for new_antinode in new_antinodes.iter() {
                debug!("New antinode: {new_antinode:?}");
                antinodes.insert(new_antinode.clone());
            }
        }
    }
    map.antinode_locations = antinodes;
    debug!("Map: {map:?}");
    info!("Answer 1: {}", map.antinode_locations.len());
    map.draw_map();

    map.antinode_locations = HashSet::new();
    info!("Part 2");
    let mut antinodes_all_distance = HashSet::new();
    for frequency in frequencies.iter() {
        debug!("Frequency: {frequency}");
        let antennae = map.get_antenna_of_same_frequency(*frequency);
        for antennas in antennae.iter().combinations(2) {
            debug!("Antennas: {antennas:?}");
            let new_antinodes = get_antinodes_all_distances(antennas[0], antennas[1], &map);
            for new_antinode in new_antinodes.iter() {
                debug!("New antinode: {new_antinode:?}");
                antinodes_all_distance.insert(new_antinode.clone());
            }
        }
    }
    map.antinode_locations = antinodes_all_distance;
    info!("Answer 2: {}", map.antinode_locations.len());
    map.draw_map();
}