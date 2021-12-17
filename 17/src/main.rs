use std::cmp::max;

type Range = (i32, i32);

#[derive(Debug)]
struct State {
    x: u32,
    y: u32,
    v_x: u32,
    v_y: u32,
}

fn step(s: &mut State) {
    s.x += s.v_x;
    s.y += s.v_y;

    s.v_x = if s.v_x > 0 { s.v_x - 1 } else { s.v_x };
    s.v_y += 1;
}

fn hits_target(target_x: Range, target_y: Range, x: u32, y: u32, v_x: u32, v_y: i32) -> bool {
    if v_y > 0 {
        let x_offset = (2 * v_y + 1) * (2 * v_x as i32 - (2 * v_y + 1) + 1) / 2;
        let x_offset = if x_offset < 0 { v_x * (v_x + 1) / 2 } else { x_offset as u32 };
        let x = x + x_offset;
        hits_target(
            target_x,
            target_y,
            x,
            y,
            max(v_x as i32 - 2 * v_y - 1, 0) as u32,
            -v_y,
        )
    } else {
        let mut s = State {
            x,
            y,
            v_x,
            v_y: (-v_y) as u32,
        };
        loop {
            if s.x as i32 > target_x.1 || -(s.y as i32) < target_y.0  {
                // Overshot
                return false
            } else if s.x as i32 >= target_x.0 && -(s.y as i32) <= target_y.1 {
                // In target
                return true
            } else {
                step(&mut s);
            }
        }
    }
}

fn find_solutions(target_x: Range, target_y: Range) -> Vec<(i32, i32)>{
    let mut solutions = Vec::new();
    for v_x in 0..target_x.1 + 1 {
        for v_y in target_y.0 - 1..-target_y.0 + 1 {
            if hits_target(target_x, target_y, 0, 0, v_x as u32, v_y) {
                solutions.push((v_x, v_y))
            }
        }
    }
    solutions
}

fn main() {
    // let (x_min, x_max) = (185, 221);
    // let (y_min, y_max) = (-122, -74);

    let (x_min, x_max) = (20, 30);
    let (y_min, y_max) = (-10, -5);
    // Part 1
    // Since we can find an x such that horizontal movement stops within the target area, we only have to consider y here.
    // As the process of rising and falling is symmetric we have to reach the start y coordinate (= 0) again when falling.
    // We have reached maximum height while still hitting the target if we have a velocity of |y_min| after passing this point.
    // Any higher point would result in a higher velocity at the starting location and thus overshoot the target
    // As we know the final speed v, we can calculate the highest point as \sum_{i = 0}^v v - i = 1/2 * v * (v + 1)
    let highest_point = y_min * (y_min + 1) / 2;
    println!("{}", highest_point);

    // Part 2
    // We assume x_min to be non-negative and y_min to be non-positive.
    // Let v_x and v_y be the initial horizontal and vertical velocity, respectively.
    // After s steps we can compute the position x_s with:
    // x_s = sum_{i = 0}^{s - 1} max(v_x - i, 0) = min(sum_{i = 0}^{s - 1} v_x - i, sum_{i = 0}^{v_x - 1} v_x - i) = 1/2 * min(s * (2 * v_x - s + 1), v_x * (v_x + 1))
    // For y_s we consider two cases:
    // 1. v_y <= 0
    // We can compute y_s with:
    // y_s = sum_{i = 0}^{s - 1} v_y + i = s * v_y + 1/2 * s * (s - 1) = 1/2 * s * (2 * v_y + s - 1)
    // 2. v_y > 0
    // So our arc rises above y = 0 for some amount of time steps s'.
    // As we already observed above, this part of the arc symmetric.
    // It takes v_y steps for the vertical velocity to reach zero at height sum_{i = 0}^{v_y - 1} v_y - i = 1/2 * v_y * (v_y + 1).
    // Since the arc is symmetric, we reach our starting height again after v_y + 1 steps, so our velocity at this point is - v_y - 1.
    // Now we can calculate the position after s + 2 * v_y + 1 steps as y_s using the formula from case one.
    // If we only consider the part below y = 0, this behaves as if we started at (x_{2 * v_y + 1}, 0) with initial velocities max(v_x - 2 * v_y - 1, 0) and -v_y.
    let hits = find_solutions((x_min, x_max), (y_min, y_max));
    println!("{:?} {}", hits, hits.len());
    // println!("{:?}", hits_target((x_min, x_max), (y_min, y_max), 0, 0, 6, 8));
    // println!("{:?}", (2 * 8 + 1) * (2 * 6 as i32 - (2 * 8 + 1) + 1) / 2);
}
