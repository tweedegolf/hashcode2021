use std::collections::HashSet;
use rand::seq::IteratorRandom;

use super::*;

pub fn solve(problem: &Problem) -> Solution {
    let mut rng = rand::thread_rng();
    let mut sol = Solution{ schedules: Vec::new()};
    sol.schedules.resize_with(problem.i, || Vec::new());

    let mut used_streets: HashSet<usize> = HashSet::new();
    for car in problem.cars.iter() {
        for seg in car.route.iter() {
            used_streets.insert(*seg);
        }
    }

    let streets = used_streets.iter().choose_multiple(&mut rng, used_streets.len());

    for i in streets {
        sol.schedules[problem.streets[*i].e].push(LightPeriod{ street: *i, period: 1 });
    }

    sol
}
