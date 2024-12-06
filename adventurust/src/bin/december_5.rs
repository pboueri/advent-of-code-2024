use std::fs;
use clap::Parser;
use log::{self, debug, info};

type PrintCommand = Vec<i32>;


type Rule = (i32, i32);

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    file_path: String
}



fn rule_is_satisfied(print_command: &PrintCommand, rule: &Rule) -> Option<bool> {

    let first = print_command.iter().position(|x| *x == rule.0);
    let second = print_command.iter().position(|x| *x == rule.1);
    if first.is_some() && second.is_some() {
        let first_value = first.unwrap();
        let second_value = second.unwrap();
        return Some(first_value < second_value);
    }
    None
}

fn check_print_command(print_command: &PrintCommand, rules: &Vec<Rule>) -> bool {
    for rule in rules {
        let rule_met = rule_is_satisfied(print_command, rule);
        if  rule_met.is_some() && !rule_met.unwrap() {
            debug!("Print command {:?} does not satisfy rule {:?}", print_command, rule);
            return false;
        }
    }
    true
}

fn get_print_middle_value(print_command: &PrintCommand) -> i32 {
    let middle_index = print_command.len() / 2;
    print_command[middle_index]
}

fn sum_print_commands(print_commands: &Vec<PrintCommand>, rules: &Vec<Rule>) -> i32 {
    let mut sum = 0;
    let mut num_valid_print_commands = 0;
    for print_command in print_commands {
        if check_print_command(print_command, rules) {
            sum += get_print_middle_value(print_command);
            num_valid_print_commands += 1;
        }
    }
    log::info!("Number of valid print commands: {} out of {}", num_valid_print_commands, print_commands.len());
    sum
}


fn sort_rules_by_print_command_order(print_command: &PrintCommand, rules: &Vec<Rule>) -> Vec<Rule> {
    let mut new_rules = rules.clone();
    
    new_rules.sort_by(|a, b| {
        let a_index = print_command.iter().position(|x| *x == a.0).unwrap();
        let b_index = print_command.iter().position(|x| *x == b.0).unwrap();
        a_index.cmp(&b_index)
    });

    new_rules
}

fn fix_print_command(print_command: &PrintCommand, rules: &Vec<Rule>) -> PrintCommand {
    let mut new_print_command = print_command.clone();

    let mut relevant_rules = rules.clone();
    relevant_rules.retain(|rule| rule_is_satisfied(print_command, rule).is_some());
    relevant_rules = sort_rules_by_print_command_order(print_command, &relevant_rules);
    debug!("Relevant rules: {:?} for command {:?}", relevant_rules, print_command);

    

    for rule in rules {
        let rule_met = rule_is_satisfied(&new_print_command, rule);
        if  rule_met.is_some() && !rule_met.unwrap() {
            debug!("Print command {:?} does not satisfy rule {:?}", new_print_command, rule);
            let first_index = new_print_command.iter().position(|x| *x == rule.1).unwrap();
            let second_index = new_print_command.iter().position(|x| *x == rule.0).unwrap();
            new_print_command.remove(second_index);
            new_print_command.insert(first_index,rule.0);
            debug!("Print command fixed to {:?}", new_print_command);       
        }
    }
    if !check_print_command(&new_print_command, &rules){
        return fix_print_command(&new_print_command,  &relevant_rules)
    }
    new_print_command
}

fn fixed_print_commands(print_commands: & Vec<PrintCommand>, rules: &Vec<Rule>) -> Vec<PrintCommand> {
    let mut fixed_print_commands = Vec::new();
    for print_command in print_commands {
        if !check_print_command(print_command, rules) {
            debug!("Print command {:?} does not satisfy rules", print_command);
            fixed_print_commands.push(fix_print_command(print_command, rules));
            debug!("Fixed print command {:?}", fixed_print_commands.last().unwrap());

        }
    } 
    fixed_print_commands
}

fn get_rules_and_prints (file_path: String) -> (Vec<Rule>, Vec<PrintCommand>) {
    
    let read_to_string = fs::read_to_string(file_path).expect("Failed to read file");
    let raw_data = read_to_string;

    let mut rules = Vec::new();
    let mut prints = Vec::new();

    for line in raw_data.lines() {
        if line.contains("|") {
            let parts: Vec<&str> = line.split("|").collect();
            rules.push((parts[0].parse::<i32>().unwrap(), parts[1].parse::<i32>().unwrap()));
        }

        if line.contains(",") {
            let parts: Vec<&str> = line.split(",").collect();
            prints.push(parts.iter().map(|x| x.parse::<i32>().unwrap()).collect());
        }

        
    }

    (rules, prints)
}




fn main(){
    env_logger::init();

    let args = Cli::parse();
    let (rules, prints) = get_rules_and_prints(args.file_path);
    let sum = sum_print_commands(&prints, &rules);
    info!("Answer 1: {sum}");
    let fixed_prints = fixed_print_commands(&prints, &rules);   
    let fixed_sum = sum_print_commands(&fixed_prints, &rules);
    info!("Answer 2: {fixed_sum}");
}