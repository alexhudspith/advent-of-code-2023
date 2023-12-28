#![allow(clippy::redundant_field_names)]

use std::collections::{HashMap, VecDeque};
use std::iter;
use std::ops::{Add, AddAssign, Mul, Sub};

use itertools::Itertools;

pub mod parse;

type CommsModuleId = usize;

type Pulse = bool;
const LOW: bool = false;
const HIGH: bool = true;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum ControlFlow {
    Break,
    Continue
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum CommsModuleType {
    Output,
    Broadcast,
    FlipFlop,
    Conjunction,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum CommsModuleState {
    Output,
    Broadcast,
    FlipFlop(Pulse),
    Conjunction(Vec<Pulse>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CommsModule {
    name: String,
    state: CommsModuleState,
    incoming: Vec<CommsModuleId>,
    outgoing: Vec<CommsModuleId>,
}

impl CommsModule {
    pub fn new(name: &str, ty: CommsModuleType) -> Self {
        use CommsModuleType as C;
        use CommsModuleState as S;
        let state = match ty {
            C::Output => S::Output,
            C::Broadcast => S::Broadcast,
            C::FlipFlop => S::FlipFlop(LOW),
            C::Conjunction => S::Conjunction(Vec::new()),
        };

        Self {
            name: name.to_owned(),
            state: state,
            incoming: Vec::new(),
            outgoing: Vec::new()
        }
    }

    pub fn reset(&mut self) {
        use CommsModuleState as S;
        self.state = match self.state {
            S::FlipFlop(_) => S::FlipFlop(LOW),
            S::Conjunction(_) => S::Conjunction(vec![LOW; self.incoming.len()]),
            S::Output => S::Output,
            S::Broadcast => S::Broadcast,
        };
    }

    pub fn add_incoming(&mut self, id: CommsModuleId) {
        self.incoming.push(id);
        if let CommsModuleState::Conjunction(v) = &mut self.state {
            v.push(LOW);
        }
    }

    pub fn add_outgoing(&mut self, id: CommsModuleId) {
        self.outgoing.push(id);
    }

    pub fn receive(&mut self, sender: CommsModuleId, pulse: Pulse) -> Option<Pulse> {
        use CommsModuleState as S;
        match self.state {
            S::Output => None,
            S::Broadcast => {
                Some(pulse)
            }
            S::FlipFlop(ref mut state) => {
                if pulse == HIGH {
                    None
                } else {
                    *state = !*state;
                    Some(*state)
                }
            }
            S::Conjunction(ref mut state) => {
                let ix = self.incoming.iter().position(|&m| m == sender).unwrap();
                state[ix] = pulse;
                Some(!state.iter().all(|&s| s))
            }
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct LowHighCount {
    pub low: u64,
    pub high: u64,
}

impl Add for LowHighCount {
    type Output = LowHighCount;

    fn add(self, rhs: Self) -> Self::Output {
        LowHighCount { low: self.low + rhs.low, high: self.high + rhs.high }
    }
}

impl AddAssign for LowHighCount {
    fn add_assign(&mut self, rhs: Self) {
        self.low += rhs.low;
        self.high += rhs.high;
    }
}

impl Sub for LowHighCount {
    type Output = LowHighCount;

    fn sub(self, rhs: Self) -> Self::Output {
        LowHighCount { low: self.low - rhs.low, high: self.high - rhs.high }
    }
}

impl Mul<u64> for LowHighCount {
    type Output = LowHighCount;

    fn mul(self, rhs: u64) -> Self::Output {
        LowHighCount { low: self.low * rhs, high: self.high * rhs }
    }
}

#[derive(Debug)]
pub struct CommsSystem {
    comms: Vec<CommsModule>,
    index: HashMap<String, CommsModuleId>,
    pending: VecDeque<(CommsModuleId, Pulse)>,
}

impl CommsSystem {
    fn new() -> Self {
        Self { comms: Vec::new(), index: HashMap::new(), pending: VecDeque::new() }
    }

    fn reset(&mut self) {
        self.pending.clear();
        for comm in &mut self.comms {
            comm.reset()
        }
    }

    fn add(&mut self, comms: CommsModule) -> CommsModuleId {
        let id = self.comms.len();
        self.index.insert(comms.name.to_owned(), id);
        self.comms.push(comms);
        id
    }

    fn connect(&mut self, comms_name1: &str, comms_name2: &str) {
        let id1 = self.index[comms_name1];
        let id2 = self.index[comms_name2];
        self.comms[id1].add_outgoing(id2);
        self.comms[id2].add_incoming(id1);
    }

    fn broadcast_module(&self) -> usize {
        self.index["broadcaster"]
    }

    fn fire(&mut self, sender_id: CommsModuleId, receiver_id: CommsModuleId, pulse: Pulse) {
        let receiver = &mut self.comms[receiver_id];
        let output = receiver.receive(sender_id, pulse);
        if let Some(pulse_out) = output {
            self.pending.push_back((receiver_id, pulse_out));
        }
    }

    pub fn push_button(&mut self) -> LowHighCount {
        let mut total = LowHighCount { low: 1, high: 0 };

        self.pending.push_back((self.broadcast_module(), LOW));
        while let Some((sender_id, pulse_in)) = self.pending.pop_front() {
            let receiver_ids = self.comms[sender_id].outgoing.clone();
            for receiver_id in receiver_ids {
                if pulse_in {
                    total.high += 1;
                } else {
                    total.low += 1;
                }
                self.fire(sender_id, receiver_id, pulse_in);
            }
        }

        total
    }

    pub fn push_button_and_wait_until<F>(&mut self, mut until_test: F) -> ControlFlow
        where F: FnMut(CommsModuleId, CommsModuleId, Pulse) -> bool
    {
        self.pending.push_back((self.broadcast_module(), LOW));

        while let Some((sender_id, pulse_in)) = self.pending.pop_front() {
            let receiver_ids = self.comms[sender_id].outgoing.clone();
            for receiver_id in receiver_ids {
                if until_test(sender_id, receiver_id, pulse_in) {
                    return ControlFlow::Break;
                }

                self.fire(sender_id, receiver_id, pulse_in);
            }
        }

        ControlFlow::Continue
    }

    fn state(&mut self) -> Vec<CommsModuleState> {
        self.comms.iter().map(|c| c.state.clone()).collect_vec()
    }

    pub fn run_part1(&mut self, button_pushes: u64) -> LowHighCount {
        let mut cycle_end_lh = LowHighCount::default();

        let mut states = HashMap::new();
        states.insert(self.state(), (0, cycle_end_lh));
        let (mut cycle_start, mut cycle_end) = (0, button_pushes);
        let mut cycle_start_lh = LowHighCount::default();
        for j in 0..button_pushes {
            cycle_end_lh += self.push_button();
            let state = self.state();
            if let Some((i, lh)) = states.insert(state, (j, cycle_end_lh)) {
                (cycle_start, cycle_end, cycle_start_lh) = (i, j, lh);
                break;
            }
        };

        let cycle_len = cycle_end - cycle_start + 1;
        let (cycles, post_cycle_inc_lh) = if cycle_len == 1 {
            (button_pushes, LowHighCount::default())
        } else {
            let steps_in_cycle = button_pushes - cycle_start;
            let cycles = steps_in_cycle / cycle_len;
            let post_cycle = steps_in_cycle % cycle_len;
            let post_cycle_inc_lh = if post_cycle == 0 {
                LowHighCount::default()
            } else {
                states.values()
                    .find(|(i, ..)| *i == cycle_start + post_cycle - 1)
                    .map(|(_, lh)| *lh)
                    .unwrap()
                    - cycle_start_lh
            };
            (cycles, post_cycle_inc_lh)
        };

        let cycle_inc_lh = cycle_end_lh - cycle_start_lh;
        cycle_inc_lh * cycles + cycle_start_lh + post_cycle_inc_lh
    }

    pub fn run_part2(&mut self) -> u64 {
        let rx = self.index["rx"];
        let rx_in = self.comms[rx].incoming.iter()
            .copied()
            .exactly_one()
            .expect("Expect exactly one node to rx");

        let conjunctions = self.comms[rx_in].incoming.clone();

        conjunctions.into_iter().map(|comm| {
            assert!(matches!(self.comms[comm].state, CommsModuleState::Conjunction(_)));
            self.reset();
            iter::repeat(()).map(|_| {
                self.push_button_and_wait_until(|sender_id, receiver_id, pulse| {
                    sender_id == comm && receiver_id == rx_in && pulse == HIGH
                })
            })
            .take_while_inclusive(|&outcome| outcome == ControlFlow::Continue)
            .count() as u64
        })
        .reduce(num::integer::lcm)
        .expect("No nodes to rx")
    }
}
