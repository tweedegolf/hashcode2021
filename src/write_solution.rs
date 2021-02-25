use std::io::Write;

use crate::{read_problem, Problem, Solution, LightPeriod};

pub fn write_solution(problem: &Problem, solution: &Solution) {
    let path = problem.name.clone().replace("data", "out").replace(".txt", "_solution.txt");
    let mut out_file = std::fs::File::create(path).unwrap();

    let mut count = 0;
    for lps in solution.schedules.iter() {
        if lps.len() > 0 {
            count += 1;
        }
    }

    write!(out_file, "{}\n", count).unwrap();
    for (intersection, lps) in solution.schedules.iter().enumerate() {
        if lps.len() > 0 {
            write!(out_file, "{}\n", intersection).unwrap();
            write!(out_file, "{}\n", lps.len()).unwrap();
            for lp in lps.iter() {
                write!(out_file, "{} {}\n", &problem.streets[lp.street].name, lp.period).unwrap();
            }
        }
    }
}

#[test]
fn test_write_solution() {
    let name = "data/a.txt".to_string();
    let contents = std::fs::read_to_string(&name).unwrap();
    let problem = read_problem::read_problem(contents, name);

    let solution = Solution {
        schedules: vec![
            vec![
                LightPeriod{ street: 0, period: 2 },
            ],
            vec![
                LightPeriod{ street: 2, period: 2 },
                LightPeriod{ street: 1, period: 1 },
            ],
            vec![
                LightPeriod{ street: 4, period: 1 },
            ]
        ]
    };

    write_solution(&problem, &solution)
}