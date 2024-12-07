use clap::Parser;
use enum_map::{enum_map, Enum, EnumMap};
use log::{self, debug, info};
use std::{collections::HashMap, fs, collections::HashSet};

#[derive(Debug, Enum, Copy, Clone, PartialEq, Eq, Hash)]
enum MapType {
    Empty,
    VisitedVertical,
    VisitedHorizontal,
    VisitedBoth,
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
    guard_states: HashSet<(MapType, usize, usize)>,
    prior_space_state: MapType,
    map_size: (usize, usize),
    loop_detected: bool,
    guard_step: EnumMap<MapType, (i8, i8)>,
    map_string: EnumMap<MapType, char>,
    valid_moves: EnumMap<MapType, MapType>,
    guard_present: bool,
    all_turns: HashMap<MapType, Vec<(usize, usize)>>,
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

fn check_if_potential_loop_old(map: &mut MapState) {
    let current_position = map.guard_position;

    for (turn_type, turn_place) in map.all_turns.iter() {
        for turn in turn_place {
            match (map.guard_type, turn_type) {
                (MapType::GuardUp, MapType::GuardDown) => {
                    if current_position.0 == turn.0 && turn.1 >= current_position.1 {
                        map.number_of_loops += 1;
                    }
                }
                (MapType::GuardDown, MapType::GuardUp) => {
                    if current_position.0 == turn.0 && turn.1 <= current_position.1 {
                        map.number_of_loops += 1;
                    }
                }
                (MapType::GuardLeft, MapType::GuardRight) => {
                    if current_position.1 == turn.1 && turn.0 <= current_position.0 {
                        map.number_of_loops += 1;
                    }
                }
                (MapType::GuardRight, MapType::GuardLeft) => {
                    if current_position.1 == turn.1 && turn.0 >= current_position.0 {
                        map.number_of_loops += 1;
                    }
                }
                _ => {}
            }
        }
    }
}

fn check_if_potential_loop(original_map: &MapState) -> bool {
    let mut map = original_map.clone();
    let guard_step = map.guard_step[map.guard_type];
    let new_position = (
        map.guard_position.0 as i32 + guard_step.0 as i32,
        map.guard_position.1 as i32 + guard_step.1 as i32,
    );
    
    if !bounds_check(new_position, map.map_size){
        return false
    }
    map.map[new_position.0 as usize][new_position.1 as usize] = MapType::Obstruction;

    debug!("Simulating {:?} {:?}", map.guard_position, map.guard_type);
    while map.guard_present && !map.loop_detected {
        map_step(&mut map, false);
        if map.guard_states.contains(&(map.guard_type, map.guard_position.0, map.guard_position.1)){
            map.loop_detected = true;
        }
    } 

    map.loop_detected
}


fn get_visit_type(guard_type: MapType, prior_state: MapType) -> MapType {
    if prior_state == MapType::VisitedVertical && (guard_type == MapType::GuardRight || guard_type == MapType::GuardLeft) {
        return MapType::VisitedBoth;
    }
    if prior_state == MapType::VisitedHorizontal && (guard_type == MapType::GuardUp || guard_type == MapType::GuardDown) {
        return MapType::VisitedBoth;
    }
    if prior_state == MapType::VisitedBoth {
        return MapType::VisitedBoth;
    }
    match guard_type {
        MapType::GuardUp => MapType::VisitedVertical,
        MapType::GuardDown => MapType::VisitedVertical,
        MapType::GuardLeft => MapType::VisitedHorizontal,
        MapType::GuardRight => MapType::VisitedHorizontal,
        _ => panic!("Invalid guard type"),
    }
}

fn map_step(map: &mut MapState, simulate: bool) -> &MapState {
    let guard_step = map.guard_step[map.guard_type];
    let guard_position = map.guard_position;
    let new_position = (
        guard_position.0 as i32 + guard_step.0 as i32,
        guard_position.1 as i32 + guard_step.1 as i32,
    );

    if !bounds_check(new_position, map.map_size) {
        map.map[guard_position.0][guard_position.1] = get_visit_type(map.guard_type, map.prior_space_state);
        map.guard_position = (new_position.0 as usize, new_position.1 as usize);
        map.guard_present = false;
        return map;
    }

    map.guard_states.insert((map.guard_type, map.guard_position.0, map.guard_position.1));

    let new_position_state = map.map[new_position.0 as usize][new_position.1 as usize];
    match new_position_state {
        MapType::Empty => {
            map.num_positions_visited += 1;
            map.guard_position = (new_position.0 as usize, new_position.1 as usize);         
            map.map[guard_position.0 as usize][guard_position.1 as usize] = get_visit_type(map.guard_type, map.prior_space_state);
            map.map[new_position.0 as usize][new_position.1 as usize] = map.guard_type;
            map.prior_space_state = MapType::Empty;
        }
        MapType::VisitedVertical | MapType::VisitedHorizontal | MapType::VisitedBoth=> {
            map.guard_position = (new_position.0 as usize, new_position.1 as usize);
            map.map[guard_position.0][guard_position.1] = get_visit_type(map.guard_type, map.prior_space_state);
            map.map[new_position.0 as usize][new_position.1 as usize] = map.guard_type;
            map.prior_space_state = new_position_state;
        }
        MapType::Obstruction => {
            map.prior_space_state =  if map.guard_type == MapType::GuardUp || map.guard_type == MapType::GuardDown {
                MapType::VisitedVertical
            } else {
                MapType::VisitedHorizontal
            };
            map.guard_type = map.valid_moves[map.guard_type];
            map.map[guard_position.0 as usize][guard_position.1 as usize] = get_visit_type(map.guard_type, map.prior_space_state);
            map.all_turns
                .get_mut(&map.guard_type)
                .unwrap()
                .push(guard_position);
            
        }
        _ => {
            panic!("Invalid move {:?} at {:?}", new_position_state, new_position);
        }
    }

    if simulate {
        if check_if_potential_loop(map){
            map.number_of_loops += 1;
        }
    }
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
        MapType::VisitedVertical => '|',
        MapType::VisitedHorizontal => '-',
        MapType::VisitedBoth => '+',
        MapType::Obstruction => '#',
        MapType::GuardUp => '^',
        MapType::GuardDown => 'v',
        MapType::GuardLeft => '<',
        MapType::GuardRight => '>',
    };

    let valid_moves: EnumMap<MapType, MapType> = enum_map! {
        MapType::Empty => MapType::Empty,
        MapType::VisitedVertical => MapType::VisitedVertical,
        MapType::VisitedHorizontal => MapType::VisitedHorizontal,
        MapType::VisitedBoth => MapType::VisitedBoth,
        MapType::Obstruction => MapType::Obstruction,
        MapType::GuardUp => MapType::GuardRight,
        MapType::GuardDown => MapType::GuardLeft,
        MapType::GuardLeft => MapType::GuardUp,
        MapType::GuardRight => MapType::GuardDown,
    };

    let guard_step: EnumMap<MapType, (i8, i8)> = enum_map! {
        MapType::Empty => (0, 0),
        MapType::VisitedVertical => (0, 0),
        MapType::VisitedHorizontal => (0, 0),
        MapType::VisitedBoth => (0, 0),
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
    let mut all_turns = HashMap::new();
    all_turns.insert(MapType::GuardDown, Vec::new());
    all_turns.insert(MapType::GuardLeft, Vec::new());
    all_turns.insert(MapType::GuardUp, Vec::new());
    all_turns.insert(MapType::GuardRight, Vec::new());
    let guard_states = HashSet::new();
    let mut map_state = MapState {
        map: map,
        map_size: map_size,
        guard_position: initial_guard_position,
        guard_states: guard_states,
        prior_space_state: MapType::Empty,
        guard_type: initial_guard_type,
        guard_step: guard_step,
        map_string: map_string,
        valid_moves: valid_moves,
        guard_present: true,
        num_positions_visited: 1,
        number_of_loops: 0,
        all_turns: all_turns,
        loop_detected: false,
    };
    info!("Guard found at: {:?}", map_state.guard_position);

    while map_state.guard_present {
        info!("Guard position: {:?}", map_state.guard_position);
        map_step(&mut map_state, true);
    }

    print_map(&map_state.map, map_state.map_string);
    info!(
        "Number of positions visited: {}",
        map_state.num_positions_visited
    );
    info!("Number of loops: {}", map_state.number_of_loops);
}
