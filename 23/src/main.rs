use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fmt::{self, Display, Formatter};

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl Amphipod {
    fn step_cost(&self) -> u32 {
        match self {
            Self::Amber => 1,
            Self::Bronze => 10,
            Self::Copper => 100,
            Self::Desert => 1000,
        }
    }
}

impl Display for Amphipod {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let s = match self {
            Self::Amber => "A",
            Self::Bronze => "B",
            Self::Copper => "C",
            Self::Desert => "D",
        };
        write!(f, "{}", s)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Tile {
    Occupied(Amphipod),
    Free,
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Occupied(a) => write!(f, "{}", a),
            Self::Free => write!(f, "."),
        }
    }
}

#[derive(Clone, Debug)]
struct State {
    hallway: [Tile; 11],
    rooms: HashMap<Amphipod, (usize, [Tile; 4])>,
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for t in &self.hallway {
            write!(f, "{}", t)?
        }
        write!(f, "\n  ")?;
        let mut rooms: Vec<_> = self.rooms.values().collect();
        rooms.sort_by(|a, b| a.0.cmp(&b.0));
        for (_, r) in &rooms {
            write!(f, "{} ", r[0])?
        }
        write!(f, "\n  ")?;
        for (_, r) in &rooms {
            write!(f, "{} ", r[1])?
        }
        write!(f, "\n  ")?;
        for (_, r) in &rooms {
            write!(f, "{} ", r[2])?
        }
        write!(f, "\n  ")?;
        for (_, r) in &rooms {
            write!(f, "{} ", r[3])?
        }
        Ok(())
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.hallway
            .iter()
            .zip(other.hallway.iter())
            .all(|(&t1, &t2)| t1 == t2)
            && self
                .rooms
                .iter()
                .map(|(k, v)| (k, v, other.rooms.get(k)))
                .all(|(_, v1, v2)| {
                    if let Some(v2) = v2 {
                        v1.0 == v2.0 && v1.1.iter().zip(v2.1.iter()).all(|(&t1, &t2)| t1 == t2)
                    } else {
                        false
                    }
                })
    }
}

impl Eq for State {}

impl State {
    fn from(
        amber_room: [Amphipod; 4],
        bronze_room: [Amphipod; 4],
        copper_room: [Amphipod; 4],
        desert_room: [Amphipod; 4],
    ) -> Self {
        let mut rooms = HashMap::new();
        rooms.insert(
            Amphipod::Amber,
            (
                2,
                [
                    Tile::Occupied(amber_room[0]),
                    Tile::Occupied(amber_room[1]),
                    Tile::Occupied(amber_room[2]),
                    Tile::Occupied(amber_room[3]),
                ],
            ),
        );
        rooms.insert(
            Amphipod::Bronze,
            (
                4,
                [
                    Tile::Occupied(bronze_room[0]),
                    Tile::Occupied(bronze_room[1]),
                    Tile::Occupied(bronze_room[2]),
                    Tile::Occupied(bronze_room[3]),
                ],
            ),
        );
        rooms.insert(
            Amphipod::Copper,
            (
                6,
                [
                    Tile::Occupied(copper_room[0]),
                    Tile::Occupied(copper_room[1]),
                    Tile::Occupied(copper_room[2]),
                    Tile::Occupied(copper_room[3]),
                ],
            ),
        );
        rooms.insert(
            Amphipod::Desert,
            (
                8,
                [
                    Tile::Occupied(desert_room[0]),
                    Tile::Occupied(desert_room[1]),
                    Tile::Occupied(desert_room[2]),
                    Tile::Occupied(desert_room[3]),
                ],
            ),
        );
        State {
            hallway: [Tile::Free; 11],
            rooms,
        }
    }

    fn next_states(&self) -> Vec<Move> {
        let mut moves = Vec::new();
        // Amphipods from hallway can move to their free destination room
        for (pos, tile) in self.hallway.iter().enumerate() {
            if let Tile::Occupied(a) = tile {
                let (idx, _) = self.rooms.get(a).expect("missing room mapping");
                if self.is_hallway_free(pos, *idx) {
                    if let Some(room_idx) = self.is_room_free(*a) {
                        let mut new_state = self.clone();
                        new_state.hallway[pos] = Tile::Free;
                        let (_, new_room) =
                            new_state.rooms.get_mut(a).expect("missing room mapping");
                        new_room[room_idx] = Tile::Occupied(*a);
                        let moved_tiles =
                            (pos as i32 - *idx as i32).abs() as u32 + room_idx as u32 + 1;
                        moves.push(Move {
                            next: new_state,
                            cost: moved_tiles * a.step_cost(),
                        });
                    }
                }
            }
        }
        // Amphipods can move from a room to the hallway
        // Amphipods may never occupy hallway indices 2, 4, 6, and 8
        for (room_type, (idx, room)) in &self.rooms {
            for (room_idx, tile) in room.iter().enumerate() {
                if let Tile::Occupied(a) = tile {
                    if room[..room_idx].iter().all(|t| matches!(t, Tile::Free)) {
                        // Only move out if we are in the wrong room or blocking others below us
                        if a != room_type
                            || room[room_idx + 1..]
                                .iter()
                                .any(|t| matches!(t, Tile::Occupied(other) if other != room_type))
                        {
                            for to in 0..11 {
                                if to != 2
                                    && to != 4
                                    && to != 6
                                    && to != 8
                                    && self.is_hallway_free(*idx, to)
                                {
                                    let mut new_state = self.clone();
                                    new_state.hallway[to] = Tile::Occupied(*a);
                                    let (_, new_room) = new_state
                                        .rooms
                                        .get_mut(room_type)
                                        .expect("missing room mapping");
                                    new_room[room_idx] = Tile::Free;
                                    let moved_tiles = (to as i32 - *idx as i32).abs() as u32
                                        + room_idx as u32
                                        + 1;
                                    moves.push(Move {
                                        next: new_state,
                                        cost: moved_tiles * a.step_cost(),
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
        moves
    }

    fn is_hallway_free(&self, from: usize, to: usize) -> bool {
        if from > to {
            self.hallway[to..from]
                .iter()
                .all(|t| matches!(t, Tile::Free))
        } else {
            self.hallway[from + 1..to + 1]
                .iter()
                .all(|t| matches!(t, Tile::Free))
        }
    }

    fn is_room_free(&self, amphipod: Amphipod) -> Option<usize> {
        let (_, room) = self.rooms.get(&amphipod).expect("missing room mapping");
        room.iter()
            .rev()
            .position(|t| matches!(t, Tile::Free))
            .map(|i| room.len() - 1 - i)
            .and_then(|i| {
                if room[i + 1..]
                    .iter()
                    .all(|t| matches!(t, Tile::Occupied(a) if *a == amphipod))
                {
                    Some(i)
                } else {
                    None
                }
            })
    }

    fn is_final(&self) -> bool {
        self.rooms.iter().all(|(room_type, (_, room))| {
            room.iter()
                .all(|t| matches!(t, Tile::Occupied(a) if a == room_type))
        })
    }

    fn estimate_cost(&self) -> u32 {
        let room_cost: u32 = self
            .rooms
            .iter()
            .map(|(room_type, (idx, room))| {
                room.iter()
                    .enumerate()
                    .map(|(room_idx, t)| {
                        if let Tile::Occupied(a) = t {
                            if a != room_type
                                || room[room_idx + 1..].iter().any(
                                    |t| matches!(t, Tile::Occupied(other) if other != room_type),
                                )
                            {
                                let (to, _) = self.rooms.get(a).expect("missing room mapping");
                                // + 2 to move out and into room
                                ((*to as i32 - *idx as i32).abs() as u32 + 2) * a.step_cost()
                            } else {
                                0
                            }
                        } else {
                            0
                        }
                    })
                    .sum::<u32>()
            })
            .sum();
        let hallway_cost: u32 = self
            .hallway
            .iter()
            .enumerate()
            .map(|(idx, t)| {
                if let Tile::Occupied(a) = t {
                    let (to, _) = self.rooms.get(a).expect("missing room mapping");
                    // + 1 to move into room
                    ((*to as i32 - idx as i32).abs() as u32 + 1) * a.step_cost()
                } else {
                    0
                }
            })
            .sum();
        room_cost + hallway_cost
    }
}

#[derive(Debug)]
struct Move {
    next: State,
    cost: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Path {
    next: usize,
    cost: u32,
    estimate: u32,
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        // if other has lower cost, we have lower Ordering
        other
            .estimate
            .cmp(&self.estimate)
            .then_with(|| other.cost.cmp(&self.cost))
            .then_with(|| self.next.cmp(&other.next))
    }
}

fn lowest_cost_ordering(init: State) -> Option<u32> {
    let mut lowest_costs = HashMap::new();
    let mut pq = BinaryHeap::new();
    let mut discovered_states = vec![init];
    pq.push(Path {
        next: 0,
        cost: 0,
        estimate: 0,
    });

    while let Some(current) = pq.pop() {
        if lowest_costs
            .get(&current.next)
            .map(|p| current > *p)
            .unwrap_or(true)
        {
            lowest_costs.insert(current.next, current);
            let state = &discovered_states[current.next];
            println!("{}", state);
            if state.is_final() {
                return Some(current.cost);
            }

            for mov in state.next_states() {
                let total_cost = current.cost + mov.cost;
                let estimate = total_cost + mov.next.estimate_cost();
                let idx = discovered_states
                    .iter()
                    .position(|s| *s == mov.next)
                    .or_else(|| {
                        discovered_states.push(mov.next);
                        Some(discovered_states.len() - 1)
                    })
                    .unwrap();
                if lowest_costs
                    .get(&idx)
                    .map(|p| p.cost > total_cost)
                    .unwrap_or(true)
                {
                    pq.push(Path {
                        next: idx,
                        cost: total_cost,
                        estimate,
                    })
                }
            }
        }
    }
    None
}

fn main() {
    let burrow = State::from(
        [Amphipod::Copper, Amphipod::Desert, Amphipod::Desert, Amphipod::Copper],
        [Amphipod::Bronze, Amphipod::Copper, Amphipod::Bronze, Amphipod::Desert],
        [Amphipod::Amber, Amphipod::Bronze, Amphipod::Amber, Amphipod::Amber],
        [Amphipod::Desert, Amphipod::Amber, Amphipod::Copper, Amphipod::Bronze],
    );
    let lowest_cost = lowest_cost_ordering(burrow).expect("found no ordering");
    println!("The lowest cost to order the amphipods is {}", lowest_cost)
}
