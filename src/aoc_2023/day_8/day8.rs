use std::cell::RefCell;
use std::collections::HashMap;
use std::ptr;
use std::ptr::{null, null_mut};
use std::rc::Rc;
use crate::aoc_2023::day_8::day8_data::GraphNode;
use crate::reader::file_io::read_file;

pub fn solve_day_8() {
    let input = read_file("src/aoc_2023/day_8/input.txt");

    // println!("Day 8, Part 1: {}", process_1(input.clone()));
    println!("Day 8, Part 2: {}", process_2(input));
}


fn process_1(input: String) -> i64 {
    let node_name_re = regex::Regex::new(r"(\w+)").unwrap();

    let parts = input.split("\n\n").collect::<Vec<&str>>();
    let instructions = parts[0];
    let nodes_strings = parts[1].split("\n").collect::<Vec<&str>>();

    let mut nodes: HashMap<&str, RefCell<GraphNode>> = HashMap::new();

    let mut first_node: String = "".to_string();
    let mut counter: i64 = 0;

    for node in nodes_strings.clone() {
        let node_strings = node_name_re.find_iter(node).map(|mat| mat.as_str()).collect::<Vec<&str>>();

        if first_node == "" {
            first_node = node_strings[0].to_string();
        }

        let mut new_left: RefCell<GraphNode>;
        let mut new_right: RefCell<GraphNode>;
        let mut new_node = RefCell::new(GraphNode {
            name: node_strings[0].to_string(),
            left: None,
            right: None,
        });

        nodes.insert(node_strings[0], new_node);

        if node_strings[0] == nodes_strings[1] {
            new_left = nodes.get(node_strings[1]).unwrap().clone();
        }
        else {
            new_left = RefCell::new(GraphNode {
                name: node_strings[1].to_string(),
                left: None,
                right: None,
            });

        }

        nodes.get_mut(node_strings[0]).unwrap().get_mut().left = Some(Rc::new(new_left));

        if node_strings[0] == nodes_strings[2] {
            new_right = nodes.get(node_strings[2]).unwrap().clone();
        }
        else {
            new_right = RefCell::new(GraphNode {
                name: node_strings[2].to_string(),
                left: None,
                right: None,
            });
        }

        nodes.get_mut(node_strings[0]).unwrap().get_mut().right = Some(Rc::new(new_right));
    }

    let mut current = "AAA".to_string();
    let instruction_amount = instructions.chars().count();

    while current.clone() != "ZZZ"{
        let instr = instructions.chars().nth((counter % instruction_amount as i64) as usize).unwrap();
        counter += 1;

        let curr_node = nodes.get(current.as_str()).unwrap().clone();

        if instr == 'L' && !curr_node.borrow().left.is_none(){
            if curr_node.borrow().left.clone().unwrap().borrow().name == current.clone() {
                println!("recursion detected on node: {}", current.clone());
                break;
            }
            current = curr_node.borrow().left.clone().unwrap().borrow().name.clone();
        }
        else if instr == 'R' && !curr_node.borrow().right.is_none(){
            if curr_node.borrow().right.clone().unwrap().borrow().name == current.clone() {
                println!("recursion detected on node: {}", current.clone());
                break;
            }
            current = curr_node.borrow().right.clone().unwrap().borrow().name.clone();
        }
        else {
            println!("Error in path!");
            break;
        }

        println!("{}: {}", counter, current.clone());
    }

    counter
}

fn process_2(input: String) -> i64 {
    let node_name_re = regex::Regex::new(r"(\w+)").unwrap();

    let parts = input.split("\n\n").collect::<Vec<&str>>();
    let instructions = parts[0];
    let nodes_strings = parts[1].split("\n").collect::<Vec<&str>>();

    let mut nodes: HashMap<&str, RefCell<GraphNode>> = HashMap::new();

    let mut first_node: String = "".to_string();


    for node in nodes_strings.clone() {
        let node_strings = node_name_re.find_iter(node).map(|mat| mat.as_str()).collect::<Vec<&str>>();

        if first_node == "" {
            first_node = node_strings[0].to_string();
        }

        let mut new_left: RefCell<GraphNode>;
        let mut new_right: RefCell<GraphNode>;
        let mut new_node = RefCell::new(GraphNode {
            name: node_strings[0].to_string(),
            left: None,
            right: None,
        });

        nodes.insert(node_strings[0], new_node);

        if node_strings[0] == nodes_strings[1] {
            new_left = nodes.get(node_strings[1]).unwrap().clone();
        }
        else {
            new_left = RefCell::new(GraphNode {
                name: node_strings[1].to_string(),
                left: None,
                right: None,
            });

        }

        nodes.get_mut(node_strings[0]).unwrap().get_mut().left = Some(Rc::new(new_left));

        if node_strings[0] == nodes_strings[2] {
            new_right = nodes.get(node_strings[2]).unwrap().clone();
        }
        else {
            new_right = RefCell::new(GraphNode {
                name: node_strings[2].to_string(),
                left: None,
                right: None,
            });
        }

        nodes.get_mut(node_strings[0]).unwrap().get_mut().right = Some(Rc::new(new_right));
    }

    // collect all nodes which names end in "A" in a vector
    let mut nodes_a = Vec::new();

    for node in nodes.clone() {
        if node.0.chars().last().unwrap() == 'A' {
            nodes_a.push(node.0.to_string());
        }
    }

    let mut ring_sizes: Vec<i64> = Vec::new();

    let instruction_amount = instructions.chars().count();

    // while all nodes in nodes_a do not have "Z" as last character in their name
    // while &nodes_a.iter().filter(|node| node.chars().last().unwrap() == 'Z').count() != &nodes_a.len() {
    for i in 0..nodes_a.len() {
        let mut counter: i64 = 0;

        let mut curr_node = nodes.get(nodes_a[i].as_str()).unwrap().clone();

        loop {
            let instr = instructions.chars().nth((counter % instruction_amount as i64) as usize).unwrap();

            if instr == 'L' && !curr_node.borrow().left.is_none(){
                if curr_node.borrow().left.clone().unwrap().borrow().name == nodes_a[i].clone() {
                    println!("recursion detected on node: {}", nodes_a[i].clone());
                    break;
                }
                nodes_a[i] = curr_node.borrow().left.clone().unwrap().borrow().name.clone();
            }
            else if instr == 'R' && !curr_node.borrow().right.is_none(){
                if curr_node.borrow().right.clone().unwrap().borrow().name == nodes_a[i].clone() {
                    println!("recursion detected on node: {}", nodes_a[i].clone());
                    break;
                }
                nodes_a[i] = curr_node.borrow().right.clone().unwrap().borrow().name.clone();
            }
            else {
                println!("Error in path!");
                break;
            }
            curr_node = nodes.get(nodes_a[i].as_str()).unwrap().clone();

            counter += 1;
            if curr_node.borrow().name.chars().last().unwrap().clone() == 'Z' {
                break;
            }
        }


        println!("{}: {:?}", counter, nodes_a.clone());

        ring_sizes.push(counter);
    }

    // multiply all entries of ring_sizes
    let mut result = 1;

    ring_sizes.into_iter()
        .reduce(|a, b| {
            result = lcm(a, b);
            result
        });

    return result;
}


// function to calculate greatest common divisor of two numbers
fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

// function to calculate least common multiple of two numbers
fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}