use std::fs;
use std::ops;

#[derive(Debug)]
struct RingArray<T: Sized, const N: usize> {
    data: [T; N],
}

impl<T: Sized, const N: usize> RingArray<T, N> {
    fn from(data: [T; N]) -> Self {
        RingArray { data }
    }

    fn iter(&self) -> std::slice::Iter<T> {
        self.data.iter()
    }
}

impl<T: Sized, const N: usize> ops::Index<usize> for RingArray<T, N> {
    type Output = T;

    fn index(&self, i: usize) -> &Self::Output {
        &self.data[i % N]
    }
}

impl<T: Sized, const N: usize> ops::IndexMut<usize> for RingArray<T, N> {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.data[i % N]
    }
}

#[derive(Debug)]
struct FishSimulation {
    day: usize,
    fishes: RingArray<u128, 7>,
    new_fishes: RingArray<u128, 3>,
}

impl FishSimulation {
    fn new(fishes: [u128; 7]) -> Self {
        FishSimulation {
            day: 0,
            fishes: RingArray::from(fishes),
            new_fishes: RingArray::from([0; 3]),
        }
    }

    fn advance_day(&mut self) {
        self.spawn_new_fishes();
        self.grow_up_new_fishes();
        self.day += 1;
    }

    fn count_fishes(&self) -> u128 {
        self.fishes.iter().sum::<u128>() + self.new_fishes.iter().sum::<u128>()
    }

    fn grow_up_new_fishes(&mut self) {
        self.fishes[self.day + 7] += self.new_fishes[self.day];
        self.new_fishes[self.day] = 0
    }

    fn spawn_new_fishes(&mut self) {
        self.new_fishes[self.day + 2] = self.fishes[self.day]
    }
}

fn main() {
    if let Ok(inputs) = fs::read_to_string("./input") {
        let mut initial_fishes: [u128; 7] = [0; 7];
        for fish in inputs.split(',') {
            if let Ok(fish) = fish.trim().parse::<u128>() {
                assert!(fish < initial_fishes.len() as u128);
                initial_fishes[fish as usize] += 1;
            }
        }
        simulate_fish_growth(initial_fishes, 256, |day| (day == 80) | (day == 256))
    }
}

fn simulate_fish_growth<F: Fn(u32) -> bool>(
    initial_fishes: [u128; 7],
    days: u32,
    is_output_day: F,
) {
    let mut fishes = FishSimulation::new(initial_fishes);
    for day in 0..=days {
        if is_output_day(day) {
            println!("There are {} fishes on day {}", fishes.count_fishes(), day);
        }
        fishes.advance_day();
    }
}
