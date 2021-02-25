use std::collections::HashSet;
use rand::seq::IteratorRandom;
use std::collections::HashMap;
use std::cmp;

use super::*;

pub fn solve(problem: &Problem) -> Solution {
    let mut rng = rand::thread_rng();
    let mut sol = Solution{ schedules: Vec::new()};
    sol.schedules.resize_with(problem.i, || Vec::new());

    let mut traffic: Vec<usize> = vec![0; problem.streets.len()];
    let mut most_popular = 0;

    for car in problem.cars.iter() {
        for &street in car.route.iter() {
            traffic[street] += 1;

            if traffic[street] > most_popular {
                most_popular = traffic[street];
            }
        }
    }

    let mut used_streets: HashSet<usize> = HashSet::new();
    for car in problem.cars.iter() {
        for seg in car.route.iter() {
            used_streets.insert(*seg);
        }
    }

    let streets = used_streets.iter().choose_multiple(&mut rng, used_streets.len());

    for i in streets {
        let mut period = 1;
        let popularity = traffic[*i] as f64 / most_popular as f64;

        if popularity > 0.1 {
            period = 2;
        }

        if popularity > 0.25 {
            period = 3;
        }

        if popularity > 0.5 {
            period = 4;
        }

        if popularity > 0.75 {
            period = 5;
        }

        if period > 0 {
            sol.schedules[problem.streets[*i].e].push(LightPeriod{
                street: *i,
                period,
            });
        }
    }

    sol
}
