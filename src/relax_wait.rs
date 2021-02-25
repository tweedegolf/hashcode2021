use std::collections::BinaryHeap;

use super::*;

#[derive(Debug, PartialEq, Eq)]
struct CarEvent {
    car: usize,
    step: usize,
    t: usize,
}

impl Ord for CarEvent {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.t.cmp(&self.t)
    }
}

impl PartialOrd for CarEvent {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn next_green(intersection: &Vec<LightPeriod>, street: usize, t: usize) -> usize {
    let tot_period = intersection.iter().fold(0, |acc, x| acc+x.period);
    let mult = t / tot_period;
    let t = t % tot_period;

    let mut t_before: usize = 0;
    let mut idx: usize = intersection.len();
    for (i, period) in intersection.iter().enumerate() {
        if period.street == street {
            idx = i;
            break;
        }
        t_before += period.period;
    }

    if idx == intersection.len() {
        panic!("aah");
    }

    if t < t_before {
        mult * tot_period + t_before
    } else if t >= t_before + intersection[idx].period {
        (mult+1) * tot_period + t_before
    } else {
        mult * tot_period + t
    }
}

pub fn score(problem: &Problem, solution: &Solution) -> usize {
    let mut next_slot: Vec<usize> = Vec::new();
    next_slot.resize(problem.streets.len(), 0);

    let mut events: BinaryHeap<CarEvent> = BinaryHeap::new();

    for (i, car) in problem.cars.iter().enumerate() {
        let start_t = next_green(&solution.schedules[problem.streets[car.route[0]].e], car.route[0], next_slot[car.route[0]]);
        next_slot[car.route[0]] = start_t + 1;
        events.push(CarEvent{car: i, step: 0, t: start_t});
        assert!(car.route.len() > 1);
    }

    let mut score: usize = 0;
    let mut max_queue_delay: usize = 0;
    let mut max_queue_spot: usize = 0;
    while let Some(e) = events.pop() {
        if e.t > problem.d {
            break;
        }

        if e.step + 1 == problem.cars[e.car].route.len() {
            score += problem.f + problem.d - e.t;
            continue
        }

        let next_street = problem.cars[e.car].route[e.step+1];

        if e.step + 2 == problem.cars[e.car].route.len() {
            events.push(CarEvent{car: e.car, step: e.step + 1, t: e.t + problem.streets[next_street].l});
            continue
        }

        
        let bare_ng = next_green(&solution.schedules[problem.streets[next_street].e], next_street, e.t + problem.streets[next_street].l);
        let slot_ng = next_green(&solution.schedules[problem.streets[next_street].e], next_street, next_slot[next_street]);

        if slot_ng > bare_ng {
            if slot_ng - bare_ng > max_queue_delay {
                max_queue_delay = slot_ng - bare_ng;
                max_queue_spot = next_street;
            }

            events.push(CarEvent{car: e.car, step: e.step+1, t: slot_ng});
            next_slot[next_street] = slot_ng + 1;
        } else {
            events.push(CarEvent{car: e.car, step: e.step+1, t: bare_ng});
            next_slot[next_street] = bare_ng + 1;
        }
    }

    score
}
