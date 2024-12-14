use std::{collections::HashMap, fs};
use clap::Parser;
use log::{self, debug, info};
use itertools::enumerate;

/*
Thinking through the problem:
1. Number of unique files = length of file // 2
2. It'd be a pretty large vector -- but we can compute the length in advance and pre-allocate
3. The file numbers are larger than a single digit, so a string representation isn't ideal

Approach:
- Allocate a large vector summing the odd digits
- fill it up according to the file-map, incrementing the file number. Keep a special space for "Free"
- Moving backwards, keep an indicator of the next free space and insert the last item
- Finally sum up the checksum
*/


const MAX_SIZE: i32 = 9;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    file_path: String,
    target_number: Option<i64>,
}

fn get_file_map(file_path: String) -> String {
   fs::read_to_string(file_path).expect("Failed to read file")
}

fn vector_size(file_map: &str) -> u32 {
    let mut size = 0;
    for (i, num) in enumerate(file_map.chars()) {
        let repitition = num.to_digit(10).unwrap();
        size += repitition;
    }
    return size
}

fn allocate_vector(file_map: &str, size: u32) -> Vec<Option<u32>> {
    let mut vector = vec![None; size as usize];
    let mut file_number = 0;
    let mut offset = 0;
    let chars = file_map.chars().collect::<Vec<char>>();

    for chunk in chars.chunks(2) {
        let file_size = chunk[0].to_digit(10).unwrap();
        let mut empty_space = 0;
        if chunk.len() == 2{
            empty_space = chunk[1].to_digit(10).unwrap();
        }
        for _ in 0..file_size {
            vector[offset] = Some(file_number);
            offset += 1;
        }
        file_number += 1;
        for _ in 0..empty_space {
            vector[offset] = None;
            offset += 1;
        }
    }
    vector
}

fn reallocate_files(file_layout: &mut Vec<Option<u32>>) {
    let mut earliest_empty = 0;
    let mut last_file = file_layout.len() - 1;
    while earliest_empty < last_file {
        if(file_layout[earliest_empty].is_none() && file_layout[last_file].is_some()) {
            file_layout[earliest_empty] = file_layout[last_file];
            file_layout[last_file] = None;
        }
        if(file_layout[earliest_empty].is_some()) {
            earliest_empty += 1;
        }
        if file_layout[last_file].is_none() {
            last_file -= 1;
        }
    }
}

fn build_span_set (file_layout: &Vec<Option<u32>>) -> HashMap<i32, Vec<usize>>{
    let mut empty_spots = HashMap::new();
    for span_size in 0..10 {
        empty_spots.insert(span_size as i32, Vec::<usize>::new());
    }
    let mut offset = 0;
    let mut new_span: bool = true;
    let mut span_size = 0;
    while offset < file_layout.len(){
        if file_layout[offset].is_none() {
            new_span = false;
            span_size += 1;
        } else {
            if !new_span {
                empty_spots.get_mut(&span_size).unwrap().push(offset- (span_size as usize));
            }
            new_span = true;
            span_size = 0;
        }
        offset += 1;
    }
    // reverse the order of the empty spots
    for (_, value) in empty_spots.iter_mut() {
        value.reverse();
    }
    empty_spots
}

fn get_file_spans(file_layout: &Vec<Option<u32>>) -> Vec<(usize, usize)>{
    let mut file_sizes: Vec<(usize, usize)> = Vec::new();
    let mut span_size = 1;
    let mut offset = 1;
    let mut new_file_id = file_layout[0];
    let mut old_file_id = file_layout[0];
    while offset < file_layout.len() {
        new_file_id = file_layout[offset];

        match (old_file_id, new_file_id) {
            (Some(old_file), Some(new_file)) if old_file == new_file => {
                span_size += 1;
            },
            (Some(old_file), Some(new_file)) if old_file != new_file => {
                file_sizes.push((offset - span_size, span_size));
                span_size = 1;
            },
            (None, Some(new_file)) => {
                span_size = 1;
            },
            (Some(_), None) => {
                file_sizes.push((offset - span_size, span_size));
                span_size = 1;
            },
            (None, None) => {
            },
            _ => {
                panic!("Unexpected file state")
            }
        }
        
        offset += 1;
        if offset == file_layout.len() {
            if old_file_id.is_some() {
                file_sizes.push((offset - span_size, span_size));
            }
            break;
        }
        old_file_id = new_file_id;
    }
    file_sizes
}


fn get_earliest_slot_big_enough(file_start: usize, file_size: usize, empty_spots: &HashMap<i32, Vec<usize>>) -> (usize, usize) {
    let mut earliest_slot = (file_start, file_size);
    for slot_size in (file_size as i32)..(MAX_SIZE + 1) {
        let available_slots = empty_spots.get(&slot_size);
        if available_slots.unwrap().len() > 0{
            // if is earlier
            let empty_spot = available_slots.unwrap().last().unwrap();
            if empty_spot <= &earliest_slot.0 {
                earliest_slot = (*empty_spot, (slot_size as usize));
            }
        }
    }
    earliest_slot
}

fn defrag_files(file_layout: &mut Vec<Option<u32>>) {
    let mut empty_spots = build_span_set(file_layout);
    let file_sizes = get_file_spans(file_layout);
    debug!("Empty spots: {:?}", empty_spots);
    debug!("File sizes: {:?}", file_sizes);
    for file_size in file_sizes.into_iter().rev() {
        let mut start = file_size.0;
        let size = file_size.1;
        debug!("Avalilable spots: {:?}", empty_spots);
        debug!("Start: {start}, Size: {size}, file_id {:?}", file_layout[start]);
        let slot_to_insert = get_earliest_slot_big_enough(start, size, &empty_spots);
        if slot_to_insert.0 == start {
            debug!("No empty spots big enough");
            continue;
        }

        // Move the file
        debug!("Slot to insert: {:?}", slot_to_insert);
        let mut offset: usize = slot_to_insert.0;
        let mut file_number = file_layout[start].unwrap();
        
        for _ in 0..size {
            file_layout[offset] = Some(file_number);
            offset += 1;
        }
        for _ in 0..size {
            file_layout[start] = None;
            start += 1;
        }

        // Change slot size
        empty_spots.get_mut(&(slot_to_insert.1 as i32)).unwrap().pop();
        // If there are remaining empty spaces add them to the empty spots
        if slot_to_insert.1 - size >  0 {
            empty_spots.get_mut(&(slot_to_insert.1 as i32 - size as i32)).unwrap().push(offset);
            empty_spots.get_mut(&(slot_to_insert.1 as i32 - size as i32)).unwrap().sort_by(|a, b| b.cmp(a));
        }
        
    }
}

fn checksum(file_layout: &Vec<Option<u32>>) -> i64 {
    let mut checksum: i64 = 0;
    for (pos,file) in enumerate(file_layout) {
        if file.is_some() {
            checksum += file.unwrap() as i64 * pos as i64;
        }
    }
    checksum
}

fn print_file_layout(file_layout: &Vec<Option<u32>>) {
    for file in file_layout {
        if file.is_some() {
            print!("{}", file.unwrap());
        } else {
            print!(".");
        }
    }
    println!();
}

fn main(){
    env_logger::init();

    let args = Cli::parse();
    let file_path = args.file_path;
    
    let file_map = get_file_map(file_path);
    let size = vector_size(&file_map);
    let vector = allocate_vector(&file_map, size);
    let mut naive_reallocate = vector.clone();
    info!("Size: {size}");
    reallocate_files(&mut naive_reallocate);
    info!("Answer 1: {}", checksum(&naive_reallocate));

    let mut defrag_reallocate = vector.clone();
    defrag_files(&mut defrag_reallocate);
    info!("Answer 2: {}", checksum(&defrag_reallocate));
    
}