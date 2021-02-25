type TravelingCar<'a> = (&'a crate::Car, usize);

pub fn score(problem: &crate::Problem, solution: &crate::Solution) -> usize {
    let mut traveling_cars = problem.cars.iter().map(|c| (c, 0)).collect();
    let mut score = 0;
    for i in 0..problem.d {
        score += step(problem, solution, i, &mut traveling_cars);
    }

    return score;
}

pub fn step(problem: &crate::Problem, solution: &crate::Solution, current_time: usize, cars: &mut Vec<TravelingCar>) -> usize {
    let mut score = 0;
    let mut passed: Vec<usize> = Vec::with_capacity(cars.len());
    for (car, current_street_idx) in cars {
        if *current_street_idx < car.route.len() {
            let on_street = car.route[*current_street_idx as usize];
            let intersection = problem.streets[on_street].e;
            if !passed.contains(&intersection) {
                let schedule = &solution.schedules[intersection];
                let int_total: usize = schedule.iter().map(|t| t.period).sum();
                let mod_curr_time = current_time % int_total;
                let mut time_offset = 0;
                let mut current_active_street = 0;
                for p in schedule {
                    time_offset += p.period;
                    if mod_curr_time < time_offset {
                        current_active_street = p.street;
                        break;
                    }
                }
                if on_street == current_active_street {
                    passed.push(intersection);
                    *current_street_idx += 1;
                    if *current_street_idx == car.route.len() {
                        score += problem.f + (problem.d - current_time);
                    }
                }
            }
        }
    }
    score
}
