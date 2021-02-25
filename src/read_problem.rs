use std::collections::HashMap;

use super::*;

pub fn read_problem(content: String) -> Problem {
    let mut lines = content.lines();
    let mut curline = lines.next().unwrap().split(" ");

    let d: isize = curline.next().unwrap().parse().unwrap();
    let i: isize = curline.next().unwrap().parse().unwrap();
    let s: isize = curline.next().unwrap().parse().unwrap();
    let v: isize = curline.next().unwrap().parse().unwrap();
    let f: isize = curline.next().unwrap().parse().unwrap();

    let mut streets: Vec<Street> = Vec::new();
    let mut street_index : HashMap<String, isize> = HashMap::new();
    for i in 0..s {
        curline = lines.next().unwrap().split(" ");
        let b: isize = curline.next().unwrap().parse().unwrap();
        let e: isize = curline.next().unwrap().parse().unwrap();
        let name: String = curline.next().unwrap().to_string();
        let l: isize = curline.next().unwrap().parse().unwrap();

        streets.push(Street{ b, e, name: name.clone(), l});
        street_index.insert(name, i);
    }

    let mut cars: Vec<Car> = Vec::new();
    for _i in 0..v {
        curline = lines.next().unwrap().split(" ");
        let p: isize = curline.next().unwrap().parse().unwrap();
        let mut route: Vec<isize> = Vec::new();

        for _j in 0..p {
            let name = curline.next().unwrap();
            route.push(street_index[name]);
        }

        cars.push(Car{route});
    }

    Problem{d,i,f,streets,cars}
}
