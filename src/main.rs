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
    let args: Vec<String> = std::env::args().collect();
    let name = args[1].clone();
    let contents = std::fs::read_to_string(&name).unwrap();

    let problem = read_problem::read_problem(contents, name);
    println!("{:?}", problem);
    println!("{:?}", ignore_unused::solve(&problem));
}
