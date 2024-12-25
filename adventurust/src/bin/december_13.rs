use std::fs;
use clap::Parser;
use log::{self, debug, error, info};

use good_lp::{constraint, variable, Solution, SolverModel, ProblemVariables};
use good_lp::solvers::highs::highs;

use simplex::{Simplex, SimplexConstraint};
/*
Thinking through the problem:
- Each problem is an LP, just use an LP solver. 
- Constraints are minimize s.t. constraints
- The constraints are:
    - Minimize the joint cost
    - Constraints are:
        - Meeting the prize
        - The number of button presses
*/

const MAX_PRESSES: i32 = 100;
const COST_A: i32 = 3;
const COST_B: i32 = 1;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Problem {
    a: (i32, i32),
    b: (i32, i32),
    prize: (i32, i32),
}
impl Problem {
    fn print(&self) {
        println!("a: {:?}", self.a);
        println!("b: {:?}", self.b);
        println!("prize: {:?}", self.prize);
    }

    fn solve_lp(&self) -> Option<(i32, i32)> {
        let mut problem = ProblemVariables::new();
        let a = problem.add(variable().integer().min(0).max(MAX_PRESSES));
        let b = problem.add(variable().integer().min(0).max(MAX_PRESSES));
        let solution = problem.minimise(a * COST_A + b * COST_B)
            .using(highs)
            .with(constraint!(a*self.a.0 + b*self.b.0 == self.prize.0))
            .with(constraint!(a*self.a.1 + b*self.b.1 == self.prize.1))
            .solve();
        match solution {
            Ok(solution) => {  
                
                let a = solution.value(a).round() as i32;
                let b = solution.value(b).round() as i32;
                debug!("Solution: ({a},{b}) -- Problem {:?}", self);
                if a*self.a.0 + b*self.b.0 != self.prize.0 && a*self.a.1 + b*self.b.1 != self.prize.1 {
                    error!("{:?}", self);
                    error!("{a}, {b}");
                    error!("{:?}", solution);
                    panic!("Solution is invalid!");
                }
                Some((a, b))
            },
            Err(_) => None
        }
    }

    fn solution_part_2_cost(&self) -> Option<(i64, i64)>{
        let PREPEND = "10000000000000".to_string();
        let mut problem = ProblemVariables::new();
        let a = problem.add(variable().integer().min(0));
        let b = problem.add(variable().integer().min(0));
        let mut x_target = PREPEND.clone();
        x_target.push_str(self.prize.0.to_string().as_str());

        let mut y_target = PREPEND.clone();
        y_target.push_str(self.prize.1.to_string().as_str());
        let x_target = x_target.parse::<f64>().unwrap();
        let y_target = y_target.parse::<f64>().unwrap();
        debug!("x_target: {x_target}, y_target: {y_target}");
        
        let x_1 = self.a.0 as f64;
        let y_1 = self.a.1 as f64;
        let x_2 = self.b.0 as f64;
        let y_2 = self.b.1 as f64;
        
    
        let a = (y_target - x_target / x_2  * y_2) / (y_1 - x_1 / x_2 * y_2);
        let b = (x_target - a * x_1) / x_2;
        if a.fract() == 0.0 && b.fract() == 0.0  && !a.is_sign_negative()  && !b.is_sign_negative(){
            debug!("Solution: ({a},{b}) -- Problem {:?}", self);
            debug !("x_target: {x_target} = {} ", a * x_1 + b * x_2);
            debug !("y_target: {y_target} = {} ", a * y_1 + b * y_2);
            return Some((a as i64, b as i64));
        }
        
        return None;
       
    }

    fn alt_solution(&self) -> Option<(i64, i64)> {
        let x_target = self.prize.0 as f64;
        let y_target = self.prize.1 as f64;
        let x_1 = self.a.0 as f64;
        let y_1 = self.a.1 as f64;
        let x_2 = self.b.0 as f64;
        let y_2 = self.b.1 as f64;
        let a = (y_target - x_target / x_2  * y_2) / (y_1 - x_1 / x_2 * y_2);
        let b = (x_target - a * x_1) / x_2;
        if a.fract() == 0.0 && b.fract() == 0.0 {
            debug!("Solution: ({a},{b}) -- Problem {:?}", self);
            debug !("x_target: {x_target} = {} ", a * x_1 + b * x_2);
            debug !("y_target: {y_target} = {} ", a * y_1 + b * y_2);
            return Some((a as i64, b as i64));
        }
        
        return None;
    }
    fn solution_cost(&self) -> i32 {
        let solution = self.solve_lp();
        if let Some(solution) = solution {
            solution.0 * COST_A + solution.1 * COST_B
        } else {
            0
        }
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    file_path: String,
}

fn read_problems(file_path: String) -> Vec<Problem> {
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let mut problems = Vec::new();
    let number_regex = regex::Regex::new(r"(\d+)").unwrap();
    for line in contents.lines().collect::<Vec<&str>>().chunks(4) {
        let a = number_regex.find_iter(line[0]).map(|x| x.as_str().parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let b = number_regex.find_iter(line[1]).map(|x| x.as_str().parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let prize = number_regex.find_iter(line[2]).map(|x| x.as_str().parse::<i32>().unwrap()).collect::<Vec<i32>>();
        problems.push(Problem {
            a: (a[0], a[1]),
            b: (b[0], b[1]),
            prize: (prize[0], prize[1]),
        });
    }
    problems
}

fn main(){
    env_logger::init();

    let args = Cli::parse();
    let file_path = args.file_path;
    let problems = read_problems(file_path);
    info!("Read {} problems", problems.len());
    let mut total_cost = 0;
    let mut total_solved = 0;
    for problem in problems.iter() {
        let cost  = problem.solution_cost();
        if cost != 0 {
            total_cost += cost;
            total_solved += 1;
        }
    }
    info!("Total Cost {total_cost}, Total Solved {total_solved}");

    let mut total_cost_part_2: i64 = 0;
    let mut total_solved_part_2 = 0;
    for (no, problem) in problems.iter().enumerate() {
        let cost  = problem.solution_part_2_cost();
        if cost.is_some() {
            info!("Problem {no} solved");
            total_cost_part_2 += cost.unwrap().0 * COST_A as i64 + cost.unwrap().1 * COST_B as i64;
            total_solved_part_2 += 1;
        }
    }
    info!("Total Cost Part 2: {total_cost_part_2}, Total Solved Part 2: {total_solved_part_2}");
}