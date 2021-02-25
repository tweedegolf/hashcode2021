struct Street {
    b: isize,
    e: isize,
    name: String,
    l: isize,
}

struct Car {
    route: Vec<isize>
}

struct Problem {
    d: isize,
    i: isize,
    f: isize,
    streets: Vec<Street>,
    cars: Vec<Car>,
}

struct LightPeriod {
    street: isize,
    period: isize,
}

struct Solution {
    schedules: Vec<Vec<LightPeriod>>,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let contents = std::fs::read_to_string(&args[1]).unwrap();

    println!("Hello, world! {}", contents);
}
