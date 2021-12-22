use std::cmp::{max, min};

use anyhow::Result;
use peg;

peg::parser! {
    grammar reboot_parser() for str {
        rule number() -> i32
            = n:$("-"?['0'..='9']+) {? n.parse().or(Err("i32")) }

        rule range() -> Range
            = min:number() ".." max:number() { Range{ min, max: max + 1 } }

        rule cuboid() -> Cuboid
            = "x=" x:range() ",y=" y:range() ",z=" z:range() { Cuboid{ x, y, z } }

        rule command() -> Command
            = c:$("on" / "off") {? match c {
                "on" => Ok(Command::On),
                "off" => Ok(Command::Off),
                c => Err("command")
            } }

        rule reboot_step() -> RebootStep
            = command:command() " " area:cuboid() { RebootStep{ command, area } }

        pub rule reboot_sequence() -> Vec<RebootStep>
            = sequence:reboot_step() ** "\n" { sequence }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Range {
    max: i32,
    min: i32,
}

impl Range {
    fn is_empty(&self) -> bool {
        self.min >= self.max
    }

    fn intersect(&self, other: &Range) -> Range {
        Range {
            min: max(self.min, other.min),
            max: min(self.max, other.max),
        }
    }

    fn contains(&self, other: &Range) -> bool {
        other.is_empty() || (self.min <= other.min && self.max >= other.max && !self.is_empty())
    }
}

#[derive(Clone, Copy, Debug)]
enum Command {
    On,
    Off,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Cuboid {
    x: Range,
    y: Range,
    z: Range,
}

impl Cuboid {
    fn intersect(&self, other: &Cuboid) -> Cuboid {
        Cuboid {
            x: self.x.intersect(&other.x),
            y: self.y.intersect(&other.y),
            z: self.z.intersect(&other.z),
        }
    }

    fn without(&self, other: &Cuboid) -> Vec<Cuboid> {
        if !self.encloses(other) {
            self.without(&self.intersect(other))
        } else if other.is_empty() {
            vec![*self]
        } else {
            if self.x == other.x {
                if self.y == other.y {
                    if self.z == other.z {
                        vec![]
                    } else {
                        let less = Cuboid {
                            z: Range {
                                max: other.z.min,
                                ..self.z
                            },
                            ..*self
                        };
                        let more = Cuboid {
                            z: Range {
                                min: other.z.max,
                                ..self.z
                            },
                            ..*self
                        };
                        let mut middle = Cuboid {
                            z: other.z,
                            ..*self
                        }
                        .without(other);
                        middle.push(less);
                        middle.push(more);
                        middle
                    }
                } else {
                    let less = Cuboid {
                        y: Range {
                            max: other.y.min,
                            ..self.y
                        },
                        ..*self
                    };
                    let more = Cuboid {
                        y: Range {
                            min: other.y.max,
                            ..self.y
                        },
                        ..*self
                    };
                    let mut middle = Cuboid {
                        y: other.y,
                        ..*self
                    }
                    .without(other);
                    middle.push(less);
                    middle.push(more);
                    middle
                }
            } else {
                let less = Cuboid {
                    x: Range {
                        max: other.x.min,
                        ..self.x
                    },
                    ..*self
                };
                let more = Cuboid {
                    x: Range {
                        min: other.x.max,
                        ..self.x
                    },
                    ..*self
                };
                let mut middle = Cuboid {
                    x: other.x,
                    ..*self
                }
                .without(other);
                middle.push(less);
                middle.push(more);
                middle
            }
        }
    }

    fn without_all(&self, others: &[Cuboid]) -> Vec<Cuboid> {
        others.iter().fold(vec![*self], |acc, o| {
            acc.iter().flat_map(|c| c.without(&o)).collect()
        })
    }

    fn size(&self) -> u128 {
        if self.is_empty() {
            0
        } else {
            (self.x.max - self.x.min) as u128
                * (self.y.max - self.y.min) as u128
                * (self.z.max - self.z.min) as u128
        }
    }

    fn is_empty(&self) -> bool {
        self.x.is_empty() || self.y.is_empty() || self.z.is_empty()
    }

    fn encloses(&self, other: &Cuboid) -> bool {
        other.is_empty()
            || (self.x.contains(&other.x)
                && self.y.contains(&other.y)
                && self.z.contains(&other.z)
                && !self.is_empty())
    }
}

#[derive(Clone, Copy, Debug)]
pub struct RebootStep {
    area: Cuboid,
    command: Command,
}

fn reboot(reboot_sequence: &[RebootStep]) -> u128 {
    let mut on: Vec<Cuboid> = Vec::new();
    for step in reboot_sequence {
        match step.command {
            Command::On => on.extend(step.area.without_all(on.as_slice())),
            Command::Off => on = on.iter().flat_map(|c| c.without(&step.area)).collect(),
        }
    }
    on.iter().map(|c| c.size()).sum::<u128>()
}

fn initialization_sequence(reboot_sequence: &[RebootStep]) -> u128 {
    let init_region = Cuboid {
        x: Range { min: -50, max: 51 },
        y: Range { min: -50, max: 51 },
        z: Range { min: -50, max: 51 },
    };
    reboot(
        reboot_sequence
            .iter()
            .filter(|s| init_region.encloses(&s.area))
            .map(|&s| s)
            .collect::<Vec<_>>()
            .as_slice(),
    )
}

fn main() -> Result<()> {
    let inputs = include_str!("../input").trim();
    let reboot_sequence = reboot_parser::reboot_sequence(inputs)?;

    let on_after_init = initialization_sequence(reboot_sequence.as_slice());
    println!("{} cubes are on after initialization", on_after_init);
    let on_after_reboot = reboot(reboot_sequence.as_slice());
    println!("{} cubes are on after full reboot", on_after_reboot);

    Ok(())
}
