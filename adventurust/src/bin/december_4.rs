use core::num;
use std::fs;
use clap::Parser;
use itertools::Itertools;



#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    file_path: String
}



fn get_raw_string (file_path: String) -> Vec<String> {
    
    let read_to_string = fs::read_to_string(file_path).expect("Failed to read file");
    let raw_data = read_to_string;

    let mut grid = Vec::new();

    for line in raw_data.lines() {
        grid.push(line.to_string());
    }
    
    grid
}

fn get_all_words(grid: &[String], x: isize, y: isize) -> Vec<String> {
    let mut words = Vec::new();
    let xs = [-1, 0, 1];
    let ys = [-1, 0, 1];
    
    for (x_offset, y_offset) in Itertools::cartesian_product(xs.iter(), ys.iter()) {
        if *x_offset == 0 && *y_offset == 0 {
            continue;
        }
        let mut x_pos = x;
        let mut y_pos = y;
        let mut word = String::new();
        for _ in 0..4 {
            if x_pos < 0 || y_pos < 0 || x_pos as usize >= grid[0].len() || y_pos as usize >= grid.len() {
                break;
            }
            word.push(grid[y_pos as usize].chars().nth(x_pos as usize).unwrap());
            x_pos += x_offset;
            y_pos += y_offset;
        }
        words.push(word);
    }
    words
}

fn is_valid_mas(grid: &[String], x: isize, y: isize) -> bool {
    if x -1 < 0 || y -1 < 0 || x + 1  >= grid[0].len() as isize|| y +1  >= grid.len() as isize {
        return false;
    }

    let top_left = grid[(y -1) as usize].chars().nth((x - 1) as usize).unwrap();
    let top_right = grid[(y -1) as usize].chars().nth((x + 1) as usize).unwrap();
    let bottom_left = grid[(y + 1) as usize].chars().nth((x - 1) as usize).unwrap();
    let bottom_right = grid[(y + 1) as usize].chars().nth((x + 1) as usize).unwrap();

    let diag_1 = format!("{}{}", top_left, bottom_right);
    let diag_2 = format!("{}{}", top_right, bottom_left);

    (diag_1 == "MS" || diag_1 == "SM") && (diag_2 == "MS" || diag_2 == "SM")
}
fn count_all_mas_xs(grid: &[String]) -> i32 {

    let mut num_matches = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y].chars().nth(x).unwrap() == 'A'{
                if is_valid_mas(grid, x as isize, y as isize){
                    num_matches += 1;
                }
            }
        }
    }

    num_matches
}

fn count_all_xmas_matches(grid: &[String]) -> i32 {
    
    let mut num_matches = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y].chars().nth(x).unwrap() == 'X'{
                num_matches += get_all_words(grid, x as isize, y as isize).iter().filter(|x| *x == "XMAS").count() as i32;
            }
        }
    }
    
    num_matches
}


fn main(){
    let args = Cli::parse();
    let program = get_raw_string(args.file_path);
    println!("Answer 1: {}", count_all_xmas_matches(&program));
    println!("Answer 2: {}", count_all_mas_xs(&program));
}