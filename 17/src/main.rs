#[derive(Debug)]
struct State {
    x: u32,
    y: i32,
    v_x: u32,
    v_y: i32,
}

fn step(s: &mut State) {
    s.x += s.v_x;
    s.y += s.v_y;

    s.v_x = if s.v_x > 0 { s.v_x - 1 } else { s.v_x };
    s.v_y -= 1;
}

fn hits_target(
    target_x: (u32, u32),
    target_y: (i32, i32),
    initial_pos: (u32, i32),
    initial_v: (u32, i32),
) -> bool {
    let mut s = State {
        x: initial_pos.0,
        y: initial_pos.1,
        v_x: initial_v.0,
        v_y: initial_v.1,
    };
    loop {
        if s.x > target_x.1 || s.y < target_y.0 {
            // Overshot
            return false;
        } else if s.x >= target_x.0 && s.y <= target_y.1 {
            // In target
            return true;
        } else {
            // Still undershooting so perform another step
            step(&mut s);
        }
    }
}

fn find_solutions(target_x: (u32, u32), target_y: (i32, i32)) -> Vec<(u32, i32)> {
    let mut solutions = Vec::new();
    for v_x in 0..target_x.1 + 1 {
        for v_y in target_y.0 - 1..-target_y.0 + 1 {
            if hits_target(target_x, target_y, (0, 0), (v_x, v_y)) {
                solutions.push((v_x, v_y))
            }
        }
    }
    solutions
}

fn main() {
    let (x_min, x_max) = (185, 221);
    let (y_min, y_max) = (-122, -74);

    // Part 1
    // Since we can find an x such that horizontal movement stops within the target area, we only have to consider y here.
    // As the process of rising and falling is symmetric we have to reach the start y coordinate (= 0) again when falling.
    // We have reached maximum height while still hitting the target if we have a velocity of |y_min| after passing this point.
    // Any higher point would result in a higher velocity at the starting location and thus overshoot the target
    // As we know the final speed v, we can calculate the highest point as \sum_{i = 0}^v v - i = 1/2 * v * (v + 1)
    let highest_point = y_min * (y_min + 1) / 2;
    println!(
        "The highest reachable point while still hitting the target is at y = {}",
        highest_point
    );

    // Part 2
    // We assume x_min to be non-negative and y_min to be non-positive.
    let hits = find_solutions((x_min, x_max), (y_min, y_max));
    println!(
        "There are {} unique possible combinations of initial velocities that hit the target",
        hits.len()
    );
}
