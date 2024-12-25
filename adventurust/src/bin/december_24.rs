use std::{collections::HashMap, fs, hash::Hash};
use clap::Parser;
use itertools::Itertools;
use log::{self, debug, error, info, Log};


/*
Approach:
- Make a reactive program that computes one demand

For Part 2:
- Figure out which bits are wrong based on additions and then debug those specific connections to correct the bit
    - To brute force could just swap all pairs in the subset (cant be more than 10) and see which one is correct
    - continue until all are correct
*/




#[derive(Parser, Debug, Clone, Hash, PartialEq, Eq)]
enum LogicOperation {
    OR,
    AND,
    XOR
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct LogicGate {
    name: String,
    op: LogicOperation,
    input_names: Vec<String>,
    computed: bool,
    output: Option<bool>,
}

impl LogicGate {

    fn get_inputs(&self, state: &HashMap<String,LogicGate>) -> Vec<LogicGate> {
        let mut inputs = Vec::new();
        for input in self.input_names.iter() {
            inputs.push(state.get(input).unwrap().clone());
        }
        inputs
    }

    fn print(&self) {
        println!("{}: {} {:?}", self.name, self.input_names.len(), self.op);
    }
}

#[derive(Debug)]
struct Circuit {
    gates: HashMap<String, LogicGate>,
    computed_values: HashMap<String, bool>,
}

impl Circuit {
    fn new(gates: HashMap<String, LogicGate>) -> Self {
        let mut computed_values = HashMap::new();
        
        // Initialize with any pre-computed gates
        for (name, gate) in &gates {
            if gate.computed {
                computed_values.insert(name.clone(), gate.output.unwrap());
            }
        }
        
        Circuit { gates, computed_values }
    }

    fn can_compute(&self, gate_name: &str) -> bool {
        if self.computed_values.contains_key(gate_name) {
            return false;
        }

        let gate = &self.gates[gate_name];
        gate.input_names.iter().all(|input| self.computed_values.contains_key(input))
    }

    fn compute_gate(&self, gate_name: &str) -> bool {
        let gate = &self.gates[gate_name];
        let input_values: Vec<bool> = gate.input_names.iter()
            .map(|name| self.computed_values[name])
            .collect();

        match gate.op {
            LogicOperation::OR => input_values.iter().any(|&x| x),
            LogicOperation::AND => input_values.iter().all(|&x| x),
            LogicOperation::XOR => input_values.iter().filter(|&&x| x).count() % 2 == 1,
        }
    }

    fn compute_all(&mut self) {
        while self.computed_values.len() < self.gates.len() {
            for name in self.gates.keys().cloned().collect::<Vec<_>>() {
                if self.can_compute(&name) {
                    let result = self.compute_gate(&name);
                    self.computed_values.insert(name, result);
                    
                }
            }
        }
    }

    fn get_output(&self, gate_name: &str) -> Option<bool> {
        self.computed_values.get(gate_name).copied()
    }

    fn get_z_values(&self) -> Vec<(String, bool)> {
        let mut z_values = Vec::new();
        for (name, gate) in &self.gates {
            if name.starts_with("z") {
                if let Some(output) = self.get_output(name) {
                    z_values.push((name.clone(), output));
                }
            }
        }
        z_values.sort_by(|x, y| x.0.cmp(&y.0));
        z_values
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    file_path: String,
    num_1: Option<i32>,
    num_2: Option<i32>,
}

fn read(file_path: String) -> HashMap<String, LogicGate> {
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let mut problems = HashMap::new();

    let input_regex = regex::Regex::new(r"([a-z0-9]{3}): ([0-1])").unwrap();
    let connection_regex = regex::Regex::new(r"([a-z0-9]{3}) (XOR|AND|OR) ([a-z0-9]{3}) -> ([a-z0-9]{3})").unwrap();

    for line in contents.lines(){
        if let Some(input) = input_regex.captures(line) {
            problems.insert(input[1].to_string(), LogicGate {
                name: input[1].to_string(),
                op: LogicOperation::OR,
                input_names: Vec::new(),
                computed: true ,
                output: Some(input[2].parse::<i32>().unwrap() == 1),
            });
        } else if let Some(connection) = connection_regex.captures(line) {
            problems.insert(connection[4].to_string(), LogicGate {
                name: connection[4].to_string(),
                op: match &connection[2] {
                    "XOR" => LogicOperation::XOR,
                    "AND" => LogicOperation::AND,
                    "OR" => LogicOperation::OR,
                    _ => panic!("Unknown operation")
                },
                input_names: vec![connection[1].to_string(), connection[3].to_string()],
                computed: false,
                output: None,
            });
        }
    }
   
    problems

}

fn vec_binary_to_bool_int(vec: &Vec<bool>) -> i64 {
    let mut result = 0;
    for (i, b) in vec.iter().enumerate() {
        if *b {
            result += 2_i64.pow(i as u32);
        }
    }
    result
}

fn i32_to_vec_bool(num: i32) -> Vec<bool> {
    let mut result = Vec::new();
    let mut num = num;
    while num > 0 {
        result.push(num % 2 == 0);
        num /= 2;
    }
    result
}

fn main(){
    env_logger::init();

    let args = Cli::parse();
    let file_path = args.file_path;
    let mut gates = read(file_path);
    if args.num_1.is_some() && args.num_2.is_some() {
        let num_1 = args.num_1.unwrap();
        let num_2 = args.num_2.unwrap();
        for (ix, bool) in i32_to_vec_bool(num_1).iter().enumerate(){
            let gate_name =  format!("x{:0>2}",ix);
            gates.get_mut(&gate_name).unwrap().output = Some(*bool);
        }
        for (ix, bool) in i32_to_vec_bool(num_2).iter().enumerate(){
            let gate_name =  format!("y{:0>2}",ix);
            gates.get_mut(&gate_name).unwrap().output = Some(*bool);
        }
    }

    let mut circuit = Circuit::new(gates);
    for (ix, gate) in circuit.gates.iter().sorted_by_key(|x| x.0).enumerate() {
        if (gate.0.starts_with("x")){
            debug!("{}: {:?}", ix, gate);
        }
        
    }
    // For each gate compute it if it can be computed
    circuit.compute_all();
    
    // You can now get any gate's output using:
    let values = circuit.get_z_values();
    let answer = vec_binary_to_bool_int(&values.iter().map(|(_, v)| *v).collect::<Vec<bool>>());
    info!("Answer: {}", answer);
    
    
}