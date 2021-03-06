mod read_problem;
mod write_solution;
mod baseline;
mod ignore_unused;
mod score;
mod random;
mod street_count;
mod relax_wait;

#[derive(Debug, Clone)]
pub struct Street {
    b: usize,
    e: usize,
    name: String,
    l: usize,
}

#[derive(Debug, Clone)]
pub struct Car {
    route: Vec<usize>
}

#[derive(Debug, Clone)]
pub struct Problem {
    name: String,
    d: usize,
    i: usize,
    f: usize,
    streets: Vec<Street>,
    cars: Vec<Car>,
}

#[derive(Debug, Clone)]
pub struct LightPeriod {
    street: usize,
    period: usize,
}

#[derive(Debug, Clone)]
pub struct Solution {
    schedules: Vec<Vec<LightPeriod>>,
}

fn main() {
    let paths: Vec<String> = vec![
        "data/a.txt".to_string(),
        "data/b.txt".to_string(),
        "data/c.txt".to_string(),
        "data/d.txt".to_string(),
        "data/e.txt".to_string(),
        "data/f.txt".to_string(),
    ];

    let mut total_score = 0;

    for name in paths.iter() {
        let contents = std::fs::read_to_string(&name).unwrap();
        let problem = read_problem::read_problem(contents, name.clone());
        let solution = street_count::solve(&problem);
        //let score = relax_wait::score(&problem, &solution);
        //let solution = ignore_unused::solve(&problem);
        let solution = relax_wait::improve(&problem, solution);
        let score = relax_wait::score(&problem, &solution).0;
        total_score += score;
        println!("Problem '{}': {}", problem.name, score);
        write_solution::write_solution(&problem, &solution);
    }

    println!("Total score: {}", total_score);
}
