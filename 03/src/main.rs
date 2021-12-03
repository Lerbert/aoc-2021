use input_parser;

fn main() {
    if let Ok(inputs) = input_parser::parse_inputs::<String>("./input") {
        let inputs: Vec<u32> = inputs.iter().filter_map(|s| u32::from_str_radix(s.trim(), 2).ok()).collect();
        if let Some(gamma_rate) = find_most_common_bit(&inputs){
            let width = 12;
            let epsilon_rate = !gamma_rate & (!0 >> (32 - width));
            println!("gamma_rate: {} epsilon_rate: {} power_consumption: {}", gamma_rate, epsilon_rate, gamma_rate * epsilon_rate);
        } else {
            println!("No inputs given.");
        }
    }
}

fn find_most_common_bit(inputs: &Vec<u32>) -> Option<u32> {
    let mut one_count = [- (inputs.len() as i32) / 2; 32];
    for bits in inputs {
        for i in 0..32 {
            one_count[i] += ((bits & (1 << i)) >> i) as i32;
        }
    }
    one_count.iter().map(|n| if n > &0 { 1 } else { 0 }).rev().reduce(|acc, bit| (acc << 1) + bit)
}
