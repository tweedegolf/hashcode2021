use std::collections::HashMap;

use super::*;

pub fn read_problem(content: String, name: String) -> Problem {
    let mut lines = content.lines();
    let mut curline = lines.next().unwrap().split(" ");

    let d: usize = curline.next().unwrap().parse().unwrap();
    let i: usize = curline.next().unwrap().parse().unwrap();
    let s: usize = curline.next().unwrap().parse().unwrap();
    let v: usize = curline.next().unwrap().parse().unwrap();
    let f: usize = curline.next().unwrap().parse().unwrap();

    let mut streets: Vec<Street> = Vec::new();
    let mut street_index : HashMap<String, usize> = HashMap::new();
    for i in 0..s {
        curline = lines.next().unwrap().split(" ");
        let b: usize = curline.next().unwrap().parse().unwrap();
        let e: usize = curline.next().unwrap().parse().unwrap();
        let name: String = curline.next().unwrap().to_string();
        let l: usize = curline.next().unwrap().parse().unwrap();

        streets.push(Street{ b, e, name: name.clone(), l});
        street_index.insert(name, i);
    }

    let mut cars: Vec<Car> = Vec::new();
    for _i in 0..v {
        curline = lines.next().unwrap().split(" ");
        let p: isize = curline.next().unwrap().parse().unwrap();
        let mut route: Vec<usize> = Vec::new();

        for _j in 0..p {
            let name = curline.next().unwrap();
            route.push(street_index[name]);
        }

        cars.push(Car{route});
    }

    Problem{name,d,i,f,streets,cars}
}
