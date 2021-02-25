mod read_problem;
mod write_solution;
mod baseline;
mod ignore_unused;

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

    for name in paths.iter() {
        let contents = std::fs::read_to_string(&name).unwrap();
        let problem = read_problem::read_problem(contents, name.clone());
        let solution = ignore_unused::solve(&problem);
        write_solution::write_solution(&problem, &solution);
    }
}
