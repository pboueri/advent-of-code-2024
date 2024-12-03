use std::fs;
use clap::Parser;
use regex::Regex;
use std::collections::BTreeMap;



#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    file_path: String
}



fn get_raw_string (file_path: String) -> String {
    
    let read_to_string = fs::read_to_string(file_path).expect("Failed to read file");
    let raw_data = read_to_string;

    let mut string = String::new();

    for line in raw_data.lines() {
        string = [string, line.to_string()].join("");
    }
    
    string
}

fn get_matches (string: &str) -> BTreeMap<usize,String> {
    let re = Regex::new(r"mul[(][0-9]{1,3},[0-9]{1,3}[)]").unwrap();
    let mut matches = BTreeMap::<usize,String>::new();
    for mat in re.find_iter(string){
        matches.insert(
            mat.start(),
            mat.as_str().to_string()
          );
    }
    matches
}

fn get_dos_and_donts(string: &str) -> Vec<(usize, usize)> {
    let re_do = Regex::new(r"do[(][)]").unwrap();
    let re_dont = Regex::new(r"don't[(][)]").unwrap();
    
    let mut do_captures = re_do.find_iter(string).map(|x| x.start());
    let mut dont_captures = re_dont.find_iter(string).map(|x| x.start());
    
    let mut dos = Vec::<(usize, usize)>::new();
    let mut dos_current = Some(0);
    let mut dont_current = dont_captures.next();
    let mut store_next = true;
    while dos_current.is_some() && dont_current.is_some() {
        if dont_current.unwrap() <  dos_current.unwrap() {
            store_next = true;
            dont_current = dont_captures.next();
        } else {
            if store_next {
                dos.push((dos_current.unwrap(), dont_current.unwrap()));
                store_next = false;
            }
            dos_current = do_captures.next();
        }
    }
    
    dos
}

fn calc_mult(mult: &str) -> i32 {
    let re = Regex::new(r"[0-9]{1,3}").unwrap();
    let mut num = 1;
    for num_str in re.find_iter(mult){
        num *= num_str.as_str().parse::<i32>().unwrap();
    }
    num
}

fn add_mults(matches: &Vec<String>) -> i32 {
    let mut sum = 0;
    for mult in matches {
        sum += calc_mult(mult);
    }
    sum
}

fn filter_mults(matches: &BTreeMap<usize,String>, dos: &[(usize,usize)]) -> BTreeMap<usize,String> {
    let mut filtered_matches = BTreeMap::<usize,String>::new();
    let mut do_iter = dos.iter();
    let mut current_pair = do_iter.next().unwrap();
    let (mut start, mut end) = *current_pair;

    for (key, val) in matches {
        while *key > end {
            current_pair = match do_iter.next() {
                Some(pair) => pair,
                None => return filtered_matches,
            };
            start = current_pair.0;
            end = current_pair.1;
        }
        if *key >= start && *key <= end {
            filtered_matches.insert(*key, val.to_string());
        }
    }
    filtered_matches
}

fn main(){
    let args = Cli::parse();
    let program = get_raw_string(args.file_path);
    let matches = get_matches(&program);
    println!("First Answer: {}", add_mults(&matches.values().cloned().collect()));
    println!("Second Answer: {}",add_mults(&filter_mults(&matches, &get_dos_and_donts(&program)).values().cloned().collect()));
}