use std::io;
use std::time::Instant;
use std::sync::{Arc, Mutex};

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

fn dpll_s(cnf_formula: Vec<Vec<i32>>, literals: &mut Vec<i32>) -> bool {
    let mut cnf_formula = cnf_formula; 

    // unit propagation:
    while cnf_formula.iter().any(|c| c.len() == 1) {
        // get a unit clause
        let l = cnf_formula.iter().find(|c| c.len() == 1).unwrap()[0]; 
        // Save the literal to the provided vector
        literals.push(l);
        cnf_formula = unit_propagate(l, &cnf_formula);
    }
    
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

    return dpll_s(cnf_formula1, literals) || dpll_s(cnf_formula2, literals);
}

fn dpll_p(cnf_formula: Vec<Vec<i32>>, literals: &mut Vec<i32>, has_cleared: &Arc<Mutex<bool>>) -> (bool, Vec<i32>) {

    let mut cnf_formula = cnf_formula; 
    
    // unit propagation:
    while cnf_formula.iter().any(|c| c.len() == 1) {
        // get a unit clause
        let l = cnf_formula.iter().find(|c| c.len() == 1).unwrap()[0]; 
        // Save the literal to the provided vector
        literals.push(l);
        cnf_formula = unit_propagate(l, &cnf_formula);
    }

    // cnf_formula is empty
    if cnf_formula.is_empty() {
        return (true, literals.clone());
    }
    // cnf_formula contains an empty clause
    if cnf_formula.contains(&Vec::new()) {
        return (false, literals.clone());
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

    // clone literal vector
    let mut literals_clone1 = literals.clone();
    let mut literals_clone2 = literals.clone();
    // clear literal vector
    literals.clear();

    let result = rayon::join(|| dpll_p(cnf_formula1, &mut literals_clone1, has_cleared), || dpll_p(cnf_formula2, &mut literals_clone2, has_cleared));

    // println!("-1- Literals after parallel execution: ({}) {:?}", literals_clone1.len(), literals_clone1);
    // println!("-2- Literals after parallel execution: ({}) {:?}", literals_clone2.len(), literals_clone2);
    
    // Lock the Mutex in write mode before modifying the flag
    let mut flag = has_cleared.lock().unwrap();
    // send literal vector to main
    if !*flag {
        *flag = true;
        literals.extend_from_slice(&literals_clone1);
        //literals.extend_from_slice(&literals_clone2);
        drop(flag);
        return (result.0.0 || result.1.0, literals.clone());
    } else {
        if !result.0.1.is_empty() {
            literals.extend_from_slice(&result.0.1);
        } else {
            literals.extend_from_slice(&result.1.1);
        }
    }

    return (result.0.0 || result.1.0, literals.clone());
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

fn main() {
    // set number of threads
    let num_threads = 10;
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

    // Shared vectors to collect literals during SAT solving (serial and parallel versions)
    let mut literals_s = Vec::new();
    let mut literals_p = Vec::new();
    // Initialize the flag
    let has_cleared = Arc::new(Mutex::new(false));

    let now = Instant::now();
    let result1 = dpll_s(cnf.clone(), &mut literals_s);
    let elapsed = now.elapsed();
    println!("Serial elapsed: {:.2?}", elapsed);

    if result1 {
        println!("The formula is satisfiable.");
    } else {
        println!("The formula is unsatisfiable.");
    }

    
    let now = Instant::now();
    let result2 = dpll_p(cnf.clone(), &mut literals_p, &has_cleared);
    let elapsed = now.elapsed();
    println!("Parallel elapsed: {:.2?} with {} threads", elapsed, num_threads);
    if result2.0 {
        println!("The formula is satisfiable.");
    } else {
        println!("The formula is unsatisfiable.");
    }
    
    println!("\nLiterals after serial execution: {:?}", literals_s);
    println!("\nLiterals after parallel execution: {:?}", literals_p);
}
