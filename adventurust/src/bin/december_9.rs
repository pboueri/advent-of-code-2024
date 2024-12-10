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
    for span_size in 0..9 {
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
    let mut span_size = 0;
    let mut offset = 0;
    let mut new_span = true;
    let mut new_file_id = 0;
    let mut old_file_id = 0;
    while offset < file_layout.len() {
        if file_layout[offset].is_some(){
            new_file_id = file_layout[offset].unwrap();
        }
        if new_file_id == old_file_id && file_layout[offset].is_some() {
            new_span = false;
            span_size += 1;
        } else {
            if !new_span {
                file_sizes.push((offset - span_size, span_size));
            }
            new_span = true;
            span_size = 1;
        }
        offset += 1;
        if offset == file_layout.len() {
            file_sizes.push((offset - span_size, span_size));
        }
        old_file_id = new_file_id;
    }
    file_sizes
}


fn defrag_files(file_layout: &mut Vec<Option<u32>>) {
    let mut empty_spots = build_span_set(file_layout);
    let file_sizes = get_file_spans(file_layout);
    debug!("Empty spots: {:?}", empty_spots);
    debug!("File sizes: {:?}", file_sizes);
    for file_size in file_sizes.into_iter().rev() {
        let mut start = file_size.0;
        let size = file_size.1;
        debug!("Start: {start}, Size: {size}, file_id {:?}", file_layout[start]);
        //TODO: Move to spots big enough (same or BIGGER)
        //TODO: Update empty spots to be smaller than they used to be (but still present)
        let mut earliest_slot = (file_layout.len() as usize, size);
        for i in size..9 {
            if empty_spots.get(&(i as i32)).unwrap().len() > 0 {
                if empty_spots.get(&(i as i32)).unwrap().last().unwrap() < &earliest_slot.0 {
                    earliest_slot = (i, *empty_spots.get(&(i as i32)).unwrap().last().unwrap());
                }
            }
        }

        if earliest_slot.0 == 0 || earliest_slot.0 > start {
            debug!("No empty spots big enough");
            continue;
        }

        debug!("Earliest slot: {:?}", earliest_slot);
        
        let mut offset: usize = earliest_slot.1;
        let mut file_number = file_layout[start].unwrap();
        
        empty_spots.get_mut(&(size as i32)).unwrap().pop();
        for _ in 0..size {
            file_layout[offset] = Some(file_number);
            offset += 1;
        }
        for _ in 0..size {
            file_layout[start] = None;
            start += 1;
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
    print_file_layout(&defrag_reallocate);
    defrag_files(&mut defrag_reallocate);
    print_file_layout(&defrag_reallocate);
    info!("Answer 2: {}", checksum(&defrag_reallocate));
    
}