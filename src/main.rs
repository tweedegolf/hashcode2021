mod read_problem;

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
    let contents = std::fs::read_to_string(&args[1]).unwrap();

    println!("{:?}", read_problem::read_problem(contents));
}
