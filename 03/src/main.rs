use input_parser;

fn main() {
    if let Ok(inputs) = input_parser::parse_inputs::<String>("./input") {
        let inputs: Vec<u32> = inputs.iter().filter_map(|s| u32::from_str_radix(s.trim(), 2).ok()).collect();
        let width = 12;
        let (gamma_rate, epsilon_rate, power_consumption) = find_power_consumption(&inputs, width);
        println!("gamma_rate: {} epsilon_rate: {} power_consumption: {}", gamma_rate, epsilon_rate, power_consumption);

        let (o2_rating, co2_rating, lif_support_rating) = find_life_support_rating(&inputs, width);
        println!("o2_rating: {} co2_rating: {} life_support_rating: {}", o2_rating, co2_rating, lif_support_rating);
    }
}

fn find_power_consumption(inputs: &Vec<u32>, width: u32) -> (u32, u32, u32) {
    let gamma_rate = (0..width)
        .map(|pos| find_most_common_bit(inputs, pos))
        .rev()
        .fold(0, |acc, bit| (acc << 1) + bit.expect("no most common bit found"));
    let epsilon_rate = invert_with_width(gamma_rate, width);
    (gamma_rate, epsilon_rate, gamma_rate * epsilon_rate)
}

fn find_life_support_rating(inputs: &Vec<u32>, width: u32) -> (u32, u32, u32) {
    let o2_rating = find_by_bit_pattern(inputs.clone(), width, |most_common| most_common.unwrap_or(1));
    let co2_rating = find_by_bit_pattern(inputs.clone(), width, |most_common| 1 - most_common.unwrap_or(1));
    (o2_rating, co2_rating, o2_rating * co2_rating)
}

fn find_by_bit_pattern<F: Fn(Option<u32>) -> u32>(mut numbers: Vec<u32>, width: u32, bit_filter: F) -> u32 {
    for pos in (0..width).rev() {
        let most_common = find_most_common_bit(&numbers, pos);
        numbers = numbers.into_iter().filter(|x| get_bit(*x, pos) == bit_filter(most_common)).collect();
        if numbers.len() == 1 {
            break
        }
    }
    numbers[0]
}

fn find_most_common_bit(inputs: &Vec<u32>, pos: u32) -> Option<u32> {
    let balance = inputs.iter()
        .map(|bits| get_bit(*bits, pos))
        .fold(0, |acc, x| acc + if x == 0 { -1 } else { 1 });
    if balance > 0 { Some(1) } else if balance < 0 { Some(0) } else {None}
}

fn get_bit(x: u32, pos: u32) -> u32 {
    (x & (1 << pos)) >> pos
}

fn invert_with_width(x: u32, width: u32) -> u32 {
    !x & (!0 >> (32 - width))
}
