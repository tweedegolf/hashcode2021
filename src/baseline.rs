use super::*;

pub fn solve(problem: &Problem) -> Solution {
    let mut sol = Solution{ schedules: Vec::new()};
    sol.schedules.resize_with(problem.i as usize, || Vec::new());
    for (i, street) in problem.streets.iter().enumerate() {
        sol.schedules[street.e as usize].push(LightPeriod{ street: i as isize, period: 1});
    }

    sol
}
