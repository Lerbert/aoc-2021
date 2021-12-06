use std::fs;

#[derive(Debug)]
struct FishMap {
    day: usize,
    fishes: [u128; 7],
    new_fishes: [u128; 3],
}

impl FishMap {
    fn new(fishes: [u128; 7]) -> Self {
        FishMap {
            day: 0,
            fishes,
            new_fishes: [0; 3],
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
        self.fishes[self.get_index_fishes(7)] += self.new_fishes[self.get_index_new_fishes(0)];
        self.new_fishes[self.get_index_new_fishes(0)] = 0
    }

    fn spawn_new_fishes(&mut self) {
        self.new_fishes[self.get_index_new_fishes(2)] = self.fishes[self.get_index_fishes(0)]
    }

    fn get_index_fishes(&self, i: usize) -> usize {
        (self.day + i) % 7
    }

    fn get_index_new_fishes(&self, i: usize) -> usize {
        (self.day + i) % 3
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
        let mut fishes = FishMap::new(initial_fishes);
        for _ in 0..256 {
            fishes.advance_day();
        }
        println!("{:?} {}", fishes, fishes.count_fishes());
    }
}
