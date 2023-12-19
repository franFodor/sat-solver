use std::io;

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

// choose literal function
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

fn dpll(cnf_formula: Vec<Vec<i32>>) -> bool {
    let mut cnf_formula = cnf_formula; 
    
    // unit propagation:
    while cnf_formula.iter().any(|c| c.len() == 1) {
        // get a unit clause
        let l = cnf_formula.iter().find(|c| c.len() == 1).unwrap()[0]; 
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
    return dpll(cnf_formula1) || dpll(cnf_formula2);
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
        let clause: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        // push the clause to the result vector
        result.push(clause);
    }

    return result;
}

fn input() -> io::Result<String> {
    let mut buffer: String = String::new();
    io::stdin().read_line(&mut buffer)?;

    Ok(buffer.trim_end().to_owned())
}

fn main() {

    let cnf_f = input().unwrap();

    let cnf = cnf_to_vec(cnf_f);
    let result = dpll(cnf);

    if result == false {
        println!("The formula is unsatisfiable.");
    } else {
        println!("The formula is satisfiable.");
    }
}
