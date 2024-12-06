use clap::Parser;
use enum_map::{enum_map, Enum, EnumMap};
use log::{self, debug, info};
use std::{collections::HashMap, fs};

#[derive(Debug, Enum, Copy, Clone, PartialEq)]
enum MapType {
    Empty,
    Visited,
    Obstruction,
    GuardUp,
    GuardDown,
    GuardLeft,
    GuardRight,
}

#[derive(Debug, Clone)]
struct MapState {
    map: Map,
    guard_position: (usize, usize),
    guard_type: MapType,
    map_size: (usize, usize),
    guard_step: EnumMap<MapType, (i8, i8)>,
    map_string: EnumMap<MapType, char>,
    valid_moves: EnumMap<MapType, MapType>,
    guard_present: bool,
    all_turns: Vec<(usize, usize)>,
    number_of_loops: i32,
    num_positions_visited: usize,
}

type Map = Vec<Vec<MapType>>;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    file_path: String,
}

fn parse_map(file_path: String) -> Map {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines = contents.split("\n");
    let mut map = Vec::new();
    for line in lines {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(match c {
                '.' => MapType::Empty,
                'X' => MapType::Visited,
                '#' => MapType::Obstruction,
                '^' => MapType::GuardUp,
                'v' => MapType::GuardDown,
                '<' => MapType::GuardLeft,
                '>' => MapType::GuardRight,
                _ => panic!("Invalid"),
            })
        }
        map.push(row);
    }
    map
}

fn print_map(map: &Map, map_string: EnumMap<MapType, char>) {
    for row in map {
        for cell in row {
            print!("{}", map_string[*cell]);
        }
        println!();
    }
}

fn check_if_potential_loop(map: &mut MapState) {
    let current_position = map.guard_position;

    for turn in map.all_turns.iter(){
        match map.guard_type {
            MapType::GuardUp => {            
                if current_position.1 == turn.1 && turn.0 > current_position.0 {
                    map.number_of_loops += 1;
                }
            }
            MapType::GuardDown=> {
                if current_position.1 == turn.1 && turn.0 < current_position.0 {
                        map.number_of_loops += 1;
                    }
            }
            MapType::GuardLeft => {
                if current_position.0 == turn.0 && turn.1 < current_position.1{
                    map.number_of_loops += 1;
                }
            }
            MapType::GuardRight => {
                if current_position.0 == turn.0 && turn.1 > current_position.1{
                    map.number_of_loops += 1;
                }
            }
            _ => {}
        }
    }
}

fn map_step(map: &mut MapState) -> &MapState {
    let guard_step = map.guard_step[map.guard_type];
    let guard_position = map.guard_position;
    let new_position = (
        guard_position.0 as i32 + guard_step.0 as i32,
        guard_position.1 as i32 + guard_step.1 as i32,
    );

    if !bounds_check(new_position, map.map_size) {
        map.map[guard_position.0][guard_position.1] = MapType::Visited;
        map.guard_position = (new_position.0 as usize, new_position.1 as usize);
        map.guard_present = false;
        return map;
    }

    let new_position_state = map.map[new_position.0 as usize][new_position.1 as usize];
    match new_position_state {
        MapType::Empty => {
            map.num_positions_visited += 1;
            map.guard_position = (new_position.0 as usize, new_position.1 as usize);
            map.map[guard_position.0][guard_position.1] = MapType::Visited;
            map.map[new_position.0 as usize][new_position.1 as usize] = map.guard_type;
        }
        MapType::Visited => {
            map.guard_position = (new_position.0 as usize, new_position.1 as usize);
            map.map[guard_position.0][guard_position.1] = MapType::Visited;
            map.map[new_position.0 as usize][new_position.1 as usize] = map.guard_type;
        }
        MapType::Obstruction => {
            map.guard_type = map.valid_moves[map.guard_type];
            map.map[guard_position.0 as usize][guard_position.1 as usize] =
                map.valid_moves[map.guard_type];
            map.all_turns.push(map.guard_position);
        }
        _ => {
            panic!("Invalid move");
        }
    }

    check_if_potential_loop(map);
    map
}

fn bounds_check(new_position: (i32, i32), map_size: (usize, usize)) -> bool {
    if new_position.0 >= 0
        && new_position.0 < map_size.0 as i32
        && new_position.1 >= 0
        && new_position.1 < map_size.1 as i32
    {
        return true;
    }
    false
}

fn find_guard_position(map: &Map) -> (usize, usize) {
    for (i, row) in map.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if *cell == MapType::GuardUp
                || *cell == MapType::GuardDown
                || *cell == MapType::GuardLeft
                || *cell == MapType::GuardRight
            {
                return (i, j);
            }
        }
    }
    panic!("No guard found");
}

fn main() {
    env_logger::init();

    let map_string: EnumMap<MapType, char> = enum_map! {
        MapType::Empty => '.',
        MapType::Visited => 'X',
        MapType::Obstruction => '#',
        MapType::GuardUp => '^',
        MapType::GuardDown => 'v',
        MapType::GuardLeft => '<',
        MapType::GuardRight => '>',
    };

    let valid_moves: EnumMap<MapType, MapType> = enum_map! {
        MapType::Empty => MapType::Empty,
        MapType::Visited => MapType::Visited,
        MapType::Obstruction => MapType::Obstruction,
        MapType::GuardUp => MapType::GuardRight,
        MapType::GuardDown => MapType::GuardLeft,
        MapType::GuardLeft => MapType::GuardUp,
        MapType::GuardRight => MapType::GuardDown,
    };

    let guard_step: EnumMap<MapType, (i8, i8)> = enum_map! {
        MapType::Empty => (0, 0),
        MapType::Visited => (0, 0),
        MapType::Obstruction => (0, 0),
        MapType::GuardUp => (-1, 0),
        MapType::GuardDown => (1, 0),
        MapType::GuardLeft => (0, -1),
        MapType::GuardRight => (0, 1),
    };

    let args = Cli::parse();
    let map = parse_map(args.file_path);
    let initial_guard_position = find_guard_position(&map);
    let initial_guard_type = map[initial_guard_position.0][initial_guard_position.1];
    let map_size = (map.len(), map[0].len());
    let mut map_state = MapState {
        map: map,
        map_size: map_size,
        guard_position: initial_guard_position,
        guard_type: initial_guard_type,
        guard_step: guard_step,
        map_string: map_string,
        valid_moves: valid_moves,
        guard_present: true,
        num_positions_visited: 1,
        number_of_loops: 0,
        all_turns: vec![],
    };
    info!("Guard found at: {:?}", map_state.guard_position);

    while map_state.guard_present {
        map_step(&mut map_state);
    }

    print_map(&map_state.map, map_state.map_string);
    info!(
        "Number of positions visited: {}",
        map_state.num_positions_visited
    );
    info!("Number of loops: {}", map_state.number_of_loops);
}
