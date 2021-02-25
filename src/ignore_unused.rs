use std::collections::HashSet;

use super::*;

pub fn solve(problem: &Problem) -> Solution {
    let mut sol = Solution{ schedules: Vec::new()};
    sol.schedules.resize_with(problem.i as usize, || Vec::new());

    let mut used_streets: HashSet<isize> = HashSet::new();
    for car in problem.cars.iter() {
        for seg in car.route.iter() {
            used_streets.insert(*seg);
        }
    }

    for i in used_streets {
        sol.schedules[problem.streets[i as usize].e as usize].push(LightPeriod{ street: i as isize, period: 1 });
    }

    sol
}
