#![allow(dead_code)]

use std::io::{self, Write};
use std::time::Instant;
use std::collections::HashSet;

use std::env;

use std::fs::OpenOptions;

fn unit_propagate(l: i32, cnf_formula: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    // initialize a new set of clauses
    let mut new_cnf_formula = Vec::new(); 
    // for each clause in cnf_formula
    for c in cnf_formula {
        // if l does not satisfy c
        if !c.contains(&l) {
            // remove the negation of l from c
            let new_c: Vec<i32> = c.iter().filter(|&x| x != &-l).cloned().collect(); 
            // add the new clause to the new set
            new_cnf_formula.push(new_c); 
        }
    }
    return new_cnf_formula;
}

fn choose_literal(cnf_formula: &Vec<Vec<i32>>) -> Option<i32> {
    // for each clause in cnf_formula
    for c in cnf_formula {
        // if c is not empty
        if !c.is_empty() {
            // return the first literal
            return Some(c[0]);
        }
    }
    return None;
}

fn dpll_s(cnf_formula: Vec<Vec<i32>>) -> bool {
    let mut cnf_formula = cnf_formula; 
    
    // unit propagation:
    while cnf_formula.iter().any(|c| c.len() == 1) {
        // get a unit clause
        let l = cnf_formula.iter().find(|c| c.len() == 1).unwrap()[0]; 
        cnf_formula = unit_propagate(l, &cnf_formula);
    }

    //println!("{:?}", cnf_formula);
    cnf_formula = pure_literal(cnf_formula.clone());
    //println!("{:?}", cnf_formula);
    
    // cnf_formula is empty
    if cnf_formula.is_empty() {
        return true;
    }
    // cnf_formula contains an empty clause
    if cnf_formula.contains(&Vec::new()) {
        return false;
    }
    
    let l = choose_literal(&cnf_formula).unwrap(); 

    let mut cnf_formula1 = cnf_formula.clone(); 
    cnf_formula1.push(vec![l]);

    let mut cnf_formula2 = cnf_formula.clone(); 
    cnf_formula2.push(vec![-l]); 

    return dpll_s(cnf_formula1) || dpll_s(cnf_formula2);
}

fn dpll_p(cnf_formula: Vec<Vec<i32>>) -> bool {

    let mut cnf_formula = cnf_formula; 
    
    // unit propagation:
    while cnf_formula.iter().any(|c| c.len() == 1) {
        // get a unit clause
        let l = cnf_formula.iter().find(|c| c.len() == 1).unwrap()[0]; 
        cnf_formula = unit_propagate(l, &cnf_formula);
    }
    
    cnf_formula = pure_literal(cnf_formula.clone());

    // cnf_formula is empty
    if cnf_formula.is_empty() {
        return true;
    }

    // cnf_formula contains an empty clause
    if cnf_formula.contains(&Vec::new()) {
        return false;
    }
    
    let l = choose_literal(&cnf_formula).unwrap(); 

    let mut cnf_formula1 = cnf_formula.clone(); 
    cnf_formula1.push(vec![l]);

    let mut cnf_formula2 = cnf_formula.clone(); 
    cnf_formula2.push(vec![-l]); 

    //let handle1 = thread::spawn(move || dpll_p(cnf_formula1));
    //let handle2 = thread::spawn(move || dpll_p(cnf_formula2));

    // Wait for the threads to finish and get their results
    //let result1 = handle1.join().unwrap();
    //let result2 = handle2.join().unwrap();

    //return dpll(cnf_formula1) || dpll(cnf_formula2);

    let result = rayon::join(|| dpll_p(cnf_formula1), || dpll_p(cnf_formula2));
    return result.0 || result.1;
}

fn cnf_to_vec(cnf: String) -> Vec<Vec<i32>> {
    // declare an empty vector of vectors
    let mut result = Vec::new();
    // iterate over the lines of the DIMACS format
    for line in cnf.lines() {
        // skip the comments and the header line
        if line.starts_with('c') || line.starts_with('p') {
            continue;
        }
        // split the line by whitespace and parse the integers
        let mut clause: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        clause.retain(|&x| x != 0);
        // push the clause to the result vector
        result.push(clause);
    }
    return result;
}

fn pure_literal(cnf_formula: Vec<Vec<i32>>) -> Vec<Vec<i32>>{
    let mut result : Vec<Vec<i32>> = Vec::new();
    let pure : HashSet<_> = cnf_formula.iter().flat_map(|row| row.iter()).cloned().collect();
    //println!("pure:{:?}", pure);
    for c in cnf_formula.iter() {
        let mut flag = false;
        for l in c {
            if !pure.contains(&-l) {
                flag = true;
                break;
            }
        };
        if !flag {
            result.push(c.clone());
        }
    }
    result
}

fn main() {
    // set number of threads
    let args: Vec<_> = env::args().collect();
    if args.len() < 1 {
        println!("Argument error!");
        return;
    }
    let num_threads = args[1].parse::<usize>().unwrap();
    let example = &args[2];

    rayon::ThreadPoolBuilder::new().num_threads(num_threads).build_global().unwrap();

    let mut input = String::new();

    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Failed to read line");

        if line.is_empty() {
            break;
        }

        input.push_str(line.as_str());
    }

    let cnf = cnf_to_vec(input);
    //println!("{:?}", cnf);

    // ODKOMENTIRAT OVO ZA SERIJSKI

    
    let now = Instant::now();
    let _ = dpll_s(cnf.clone());
    let elapsed = now.elapsed().as_millis();
    let serial_out = format!("S,1,{:.2?},{}", elapsed, example);
    let output_string = format!("{}", serial_out);
    
    

    /*
    if result1 {
        println!("The formula is satisfiable.");
    } else {
        println!("The formula is unsatisfiable.");
    }
    */

    // ODKOMENTIRAT OVO ZA PARALEL

     /*
     let now = Instant::now();
     let _ = dpll_p(cnf.clone());
     let elapsed = now.elapsed().as_millis();
     let parralel_out = format!("P,{},{:.2?},{}", num_threads, elapsed, example);
     let output_string = format!("{}", parralel_out);
     */
    

    /*
    if result2 {
        println!("The formula is satisfiable.");
    } else {
        println!("The formula is unsatisfiable.");
    }
    */

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("results.txt")
        .unwrap();

    if let Err(e) = writeln!(file, "{}", output_string) {
        eprintln!("Couldn't write to file: {}", e);
    }

    /* 
    let path = "./results.txt";
    let mut output = File::create(path).unwrap();

    let _ = write!(output, "{}", parralel_out);*/

}
