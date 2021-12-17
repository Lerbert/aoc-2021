fn main() {
    let (x_min, x_max) = (185, 221);
    let (y_min, y_max) = (-122, -74);
    // Part 1
    // Since we can find an x such that horizontal movement stops within the target area, we only have to consider y here.
    // As the process of rising and falling is symmetric we have to reach the start y coordinate (= 0) again when falling.
    // We have reached maximum height while still hitting the target if we have a velocity of |y_min| after passing this point.
    // Any higher point would result in a higher velocity at the starting location and thus overshoot the target
    // As we know the final speed v, we can calculate the highest point as \sum_{i=0}^v v - i = 1/2 * v * (v + 1)
    let highest_point = y_min * (y_min + 1) / 2;
    println!("{}", highest_point);
}
