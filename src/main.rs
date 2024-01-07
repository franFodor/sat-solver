use std::io;
use std::time::Instant;
use std::sync::{Arc, RwLock};

fn unit_propagate(l: i32, cnf_formula: &Vec<Vec<i32>>, literals: &Arc<RwLock<Vec<i32>>>) -> Vec<Vec<i32>> {
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

    // Save the literal to the shared vector
    literals.write().unwrap().push(l);

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

fn dpll_s(cnf_formula: Vec<Vec<i32>>, literals: &Arc<RwLock<Vec<i32>>>) -> bool {
    let mut cnf_formula = cnf_formula; 
    
    // unit propagation:
    while cnf_formula.iter().any(|c| c.len() == 1) {
        // get a unit clause
        let l = cnf_formula.iter().find(|c| c.len() == 1).unwrap()[0]; 
        cnf_formula = unit_propagate(l, &cnf_formula, literals);
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

fn dpll_p(cnf_formula: Vec<Vec<i32>>, literals: &Arc<RwLock<Vec<i32>>>) -> bool {

    let mut cnf_formula = cnf_formula; 
    
    // unit propagation:
    while cnf_formula.iter().any(|c| c.len() == 1) {
        // get a unit clause
        let l = cnf_formula.iter().find(|c| c.len() == 1).unwrap()[0]; 
        
        // print literal and thread name
        // let thread_id = rayon::current_thread_index().unwrap_or(0);
        // println!("Thread {:?} - Literal {} ", thread_id, l);
        
        cnf_formula = unit_propagate(l, &cnf_formula, literals);
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

    //let handle1 = thread::spawn(move || dpll_p(cnf_formula1));
    //let handle2 = thread::spawn(move || dpll_p(cnf_formula2));

    // Wait for the threads to finish and get their results
    //let result1 = handle1.join().unwrap();
    //let result2 = handle2.join().unwrap();

    //return dpll(cnf_formula1) || dpll(cnf_formula2);

    let result = rayon::join(|| dpll_p(cnf_formula1, literals), || dpll_p(cnf_formula2, literals));
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
    let literals_s = Arc::new(RwLock::new(Vec::new()));
    let literals_p = Arc::new(RwLock::new(Vec::new()));

    let now = Instant::now();
    let result1 = dpll_s(cnf.clone(), &literals_s);
    let elapsed = now.elapsed();
    println!("Serial elapsed: {:.2?}", elapsed);


    if result1 {
        println!("The formula is satisfiable.");
    } else {
        println!("The formula is unsatisfiable.");
    }

    let now = Instant::now();
    let result2 = dpll_p(cnf.clone(), &literals_p);
    let elapsed = now.elapsed();
    println!("Parallel elapsed: {:.2?} with {} threads", elapsed, num_threads);
    if result2 {
        println!("The formula is satisfiable.");
    } else {
        println!("The formula is unsatisfiable.");
    }
    
    println!("\nLiterals after serial execution: {:?}", literals_s.read().unwrap());
    println!("\nLiterals after parallel execution: {:?}", literals_p.read().unwrap());
}
