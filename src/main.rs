mod read_problem;
mod write_solution;
mod baseline;
mod ignore_unused;

#[derive(Debug)]
pub struct Street {
    b: isize,
    e: isize,
    name: String,
    l: isize,
}

#[derive(Debug)]
pub struct Car {
    route: Vec<isize>
}

#[derive(Debug)]
pub struct Problem {
    name: String,
    d: isize,
    i: isize,
    f: isize,
    streets: Vec<Street>,
    cars: Vec<Car>,
}

#[derive(Debug)]
pub struct LightPeriod {
    street: isize,
    period: isize,
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
