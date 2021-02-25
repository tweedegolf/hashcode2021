use std::{cell::RefCell, collections::VecDeque};

pub fn score(problem: &crate::Problem, solution: &crate::Solution) -> usize {
    let mut queue = Vec::new();
    queue.resize_with(problem.streets.len(), || RefCell::new(VecDeque::new()));
    for (idx, car) in problem.cars.iter().enumerate() {
        queue[car.route[0]].borrow_mut().push_back(idx);
    }
    let mut score = 0;
    for i in 0..problem.d {
        score += step(problem, solution, i, &mut queue);
    }

    return score;
}

pub fn get_active_street(schedule: &Vec<crate::LightPeriod>, current_time: usize) -> Option<usize> {
    if schedule.len() > 0 {
        let int_total: usize = schedule.iter().map(|t| t.period).sum();
        let mod_curr_time = current_time % int_total;
        let mut time_offset = 0;
        for p in schedule {
            time_offset += p.period;
            if mod_curr_time < time_offset {
                return Some(p.street);
            }
        }
    }

    None
}

pub fn step(problem: &crate::Problem, solution: &crate::Solution, current_time: usize, queue: &mut Vec<RefCell<VecDeque<usize>>>) -> usize {
    let mut score = 0;
    let mut cars_moved = Vec::new();
    for (street, cars_waiting) in queue.iter().enumerate() {
        if cars_waiting.borrow().len() > 0 {
            let intersection = problem.streets[street].e;
            let schedule = &solution.schedules[intersection];
            let active_street_opt = get_active_street(schedule, current_time);
            if let Some(active_street) = active_street_opt {
                if active_street == street {
                    if !cars_moved.contains(cars_waiting.borrow().get(0).expect("cars cannot be empty here")) {
                        let car = cars_waiting.borrow_mut().pop_front().expect("cars cannot be empty here");
                        let car_route = &problem.cars[car].route;
                        let next_street_idx = 1 + car_route.iter().position(|e| *e == active_street).expect("car is on street not in its route");
                        if next_street_idx < car_route.len() {
                            let next_street = car_route[next_street_idx];
                            queue[next_street].borrow_mut().push_back(car);
                            cars_moved.push(car);
                        } else {
                            score += problem.f + (problem.d - current_time);
                        }
                    }
                }
            }
        }
    }
    score
}
