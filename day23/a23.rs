// Not mine - all credit goes to: https://github.com/prscoelho/aoc2021/blob/main/a23.rs

use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
};

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
struct Room {
    size: usize,
    amphipod: u8,
    room_amphipods: Vec<u8>,
    open: bool,
    completed: bool,
}

impl Room {
    fn new(amphipod: u8, room: Vec<u8>) -> Self {
        Self {
            size: room.len(),
            amphipod,
            room_amphipods: room,
            open: false,
            completed: false,
        }
    }

    fn complete(&mut self) {
        while let Some(&amphi) = self.room_amphipods.first() {
            if amphi == self.amphipod {
                self.room_amphipods.remove(0);
                self.size -= 1;
                if self.size == 0 {
                    self.completed = true;
                }
            } else {
                break;
            }
        }
    }

    fn add_amphipod(&mut self) -> usize {
        assert!(self.open);
        assert!(!self.completed);
        assert!(self.size > 0);
        // cost: 3 if size is 4, 2 if size is 3, 1 if size is 2, 0 if size is 1
        let cost = self.size - 1;
        self.size -= 1;
        if self.size == 0 {
            self.completed = true;
        }
        cost
    }

    fn peek_amphipod(&self) -> Option<u8> {
        self.room_amphipods.last().cloned()
    }

    fn pop_amphipod(&mut self) -> (u8, usize) {
        assert!(!self.room_amphipods.is_empty());

        let cost = self
            .room_amphipods
            .iter()
            .rev()
            .skip(1)
            .map(|&n| 10usize.pow(n as u32))
            .sum();

        let amphipod = self.room_amphipods.pop().unwrap();

        if self.room_amphipods.is_empty() {
            self.open = true;
        }

        (amphipod, cost)
    }
}

#[derive(Hash, PartialEq, Eq, Clone)]
struct World {
    rooms: [Room; 4],
    hallway: [u8; 7],
}

type HallwayCache = HashMap<(Node, Vec<bool>), HashSet<Node>>;

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // compare other with self (reverse order)
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, PartialEq, Eq)]
struct State {
    cost: usize,
    world: World,
}

impl State {
    // advance all amphipods that can move towards their room
    // there is no suboptimal path here, so we can just move them all without branching
    fn advance(&mut self, graph: &Graph, cache: &mut HallwayCache, pairs: &Pairs) {
        'outer: loop {
            let hallway_passable: Vec<bool> =
                self.world.hallway.iter().map(|&v| v == u8::MAX).collect();
            for (room_idx, room) in self.world.rooms.iter_mut().enumerate() {
                if room.open {
                    let idxes: Vec<usize> = self
                        .world
                        .hallway
                        .iter()
                        .enumerate()
                        .filter(|(_, amphipod)| room.amphipod == **amphipod)
                        .map(|(idx, _)| idx)
                        .collect();

                    for location in idxes {
                        let hallway_node = Node::Hallway(location);
                        let room_node = Node::Room(room_idx);
                        let set = reachable_nodes(hallway_node, &hallway_passable, graph, cache);

                        if set.contains(&room_node) {
                            // move amphipod in location_idx into room_idx
                            let room_movement_cost = room.add_amphipod();
                            let hallway_movement_cost =
                                pairs.get(&(hallway_node, room_node)).unwrap();
                            let added_cost = (hallway_movement_cost + room_movement_cost)
                                * 10usize.pow(room_idx as u32);
                            self.cost += added_cost;
                            self.world.hallway[location] = u8::MAX;

                            continue 'outer;
                        }
                    }
                }
            }
            break;
        }
    }

    fn branch(&self, graph: &Graph, cache: &mut HallwayCache, pairs: &Pairs) -> Vec<State> {
        let mut result = Vec::new();

        let hallways: Vec<bool> = self.world.hallway.iter().map(|&v| v == u8::MAX).collect();

        for (room_idx, room) in self.world.rooms.iter().enumerate() {
            if let Some(amphipod) = room.peek_amphipod() {
                let room_node = Node::Room(room_idx);

                for node in reachable_nodes(room_node, &hallways, graph, cache) {
                    if let Node::Hallway(hallway_idx) = node {
                        let mut new_state = self.clone();
                        let movement_cost = pairs[&(room_node, node)];
                        let (_, pop_cost) = new_state.world.rooms[room_idx].pop_amphipod();
                        new_state.cost += pop_cost + movement_cost * 10usize.pow(amphipod as u32);
                        new_state.world.hallway[hallway_idx] = amphipod;
                        new_state.advance(graph, cache, pairs);
                        result.push(new_state);
                    }
                }
            }
        }

        result
    }
}

impl World {
    fn new(starting_rooms: [Room; 4]) -> Self {
        Self {
            rooms: starting_rooms,
            hallway: [u8::MAX; 7],
        }
    }
}

// bfs search for reachable hallways and rooms according to current hallway layout
// cache the results by key = start, hallway
// we're not concerned with the movement cost, since the combinations are low
// we can precompute all cost pairs (Node -> Node)
fn reachable_nodes(
    start: Node,
    hallways: &[bool],
    graph: &Graph,
    cache: &mut HallwayCache,
) -> HashSet<Node> {
    let key = (start, hallways.to_owned());
    if let Some(v) = cache.get(&key) {
        return v.clone();
    }
    let mut seen = HashSet::new();
    seen.insert(start);

    let mut queue = vec![start];

    while !queue.is_empty() {
        let position = queue.pop().unwrap();

        for &neighbour in graph[&position].keys() {
            if let Node::Hallway(idx) = neighbour {
                // is passable and hasn't been seen yet
                if hallways[idx] && seen.insert(neighbour) {
                    queue.push(neighbour);
                }
            } else {
                // is room, so we don't want to branch but we want to add it to reachable
                seen.insert(neighbour);
            }
        }
    }

    seen.remove(&start);

    cache.insert(key, seen.clone());
    seen
}

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
enum Node {
    Hallway(usize),
    Room(usize),
}

type Graph = HashMap<Node, HashMap<Node, usize>>;
type Pairs = HashMap<(Node, Node), usize>;

fn build_pairs(graph: &Graph) -> Pairs {
    let mut pairs = HashMap::new();

    for start in graph.keys().cloned() {
        let mut queue = VecDeque::new();
        queue.push_back((start, 0));
        let mut seen = HashSet::new();

        while !queue.is_empty() {
            let (current, cost) = queue.pop_front().unwrap();

            pairs.insert((start, current), cost);

            for (&neighbour, &added_cost) in graph.get(&current).unwrap() {
                if seen.insert(neighbour) {
                    queue.push_front((neighbour, cost + added_cost));
                }
            }
        }
    }

    pairs
}

/* Graph
 #############
│#01.2.3.4.56#
│###0#1#2#3###
 ###A#B#C#D###
 ############# */

fn build_graph() -> HashMap<Node, HashMap<Node, usize>> {
    let mut graph = HashMap::new();
    for i in 0..7 {
        graph.insert(Node::Hallway(i), HashMap::new());
    }

    for i in 2..5 {
        let node = Node::Hallway(i);
        graph
            .get_mut(&node)
            .unwrap()
            .insert(Node::Hallway(i - 1), 2);
        graph
            .get_mut(&node)
            .unwrap()
            .insert(Node::Hallway(i + 1), 2);
    }

    for i in 0..4 {
        let room = Node::Room(i);

        graph.insert(room, HashMap::new());

        for n in 1..3 {
            let hallway = Node::Hallway(i + n);

            graph.get_mut(&hallway).unwrap().insert(room, 2);
            graph.get_mut(&room).unwrap().insert(hallway, 2);
        }
    }

    graph
        .get_mut(&Node::Hallway(0))
        .unwrap()
        .insert(Node::Hallway(1), 1);
    graph
        .get_mut(&Node::Hallway(1))
        .unwrap()
        .insert(Node::Hallway(0), 1);

    graph
        .get_mut(&Node::Hallway(5))
        .unwrap()
        .insert(Node::Hallway(6), 1);
    graph
        .get_mut(&Node::Hallway(6))
        .unwrap()
        .insert(Node::Hallway(5), 1);

    graph
        .get_mut(&Node::Hallway(1))
        .unwrap()
        .insert(Node::Hallway(2), 2);
    graph
        .get_mut(&Node::Hallway(5))
        .unwrap()
        .insert(Node::Hallway(4), 2);

    graph
}

fn dijkstra(mut world: World) -> usize {
    for room in world.rooms.iter_mut() {
        room.complete();
    }
    let graph = build_graph();
    let pairs = build_pairs(&graph);
    let mut cache = HallwayCache::new();

    let mut heap = BinaryHeap::new();
    heap.push(State { cost: 0, world });

    let mut seen = HashMap::new();

    while let Some(state) = heap.pop() {
        if state.world.rooms.iter().all(|room| room.completed) {
            return state.cost;
        }

        if state.cost > seen.get(&state.world).cloned().unwrap_or(usize::MAX) {
            continue;
        }

        for new_state in state.branch(&graph, &mut cache, &pairs) {
            let entry = seen.entry(new_state.world.clone()).or_insert(usize::MAX);
            if new_state.cost < *entry {
                *entry = new_state.cost;
                heap.push(new_state);
            }
        }
    }

    // result should be Option<usize>, but we know our input is solvable, so this should not be reachable
    usize::MAX
}

fn main() {
    let rooms = [
        Room::new(0, vec![3, 3]),
        Room::new(1, vec![0, 1]),
        Room::new(2, vec![1, 2]),
        Room::new(3, vec![0, 2]),
    ];

    let world = World::new(rooms);
    println!("{}", dijkstra(world));

    // #D#C#B#A#
    // #D#B#A#C#
    let rooms = [
        Room::new(0, vec![3, 3, 3, 3]),
        Room::new(1, vec![0, 1, 2, 1]),
        Room::new(2, vec![1, 0, 1, 2]),
        Room::new(3, vec![0, 2, 0, 2]),
    ];

    let world = World::new(rooms);
    println!("{}", dijkstra(world));
}
