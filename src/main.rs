mod read_problem;
mod write_solution;
mod baseline;
mod ignore_unused;
mod score;

#[derive(Debug)]
pub struct Street {
    b: usize,
    e: usize,
    name: String,
    l: usize,
}

#[derive(Debug)]
pub struct Car {
    route: Vec<usize>
}

#[derive(Debug)]
pub struct Problem {
    name: String,
    d: usize,
    i: usize,
    f: usize,
    streets: Vec<Street>,
    cars: Vec<Car>,
}

#[derive(Debug)]
pub struct LightPeriod {
    street: usize,
    period: usize,
}

#[derive(Debug)]
pub struct Solution {
    schedules: Vec<Vec<LightPeriod>>,
}

fn main() {
    let mut paths: Vec<String> = vec![
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
        let solution = ignore_unused::solve(&problem);
        let score = score::score(&problem, &solution);
        total_score += score;
        println!("Problem '{}': {}", problem.name, score);
        write_solution::write_solution(&problem, &solution);
    }
    println!("Total score: {}", total_score);
}
