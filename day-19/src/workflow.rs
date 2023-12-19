#![allow(clippy::redundant_field_names)]

use std::collections::HashMap;
use crate::parts::{Category, Part, RangedPart};
use crate::range::Range;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Target {
    Accept,
    Reject,
    Workflow(String),
    Pass,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RangedPartTarget {
    part: RangedPart,
    target: Target,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Op {
    LessThan,
    GreaterThan,
    True,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rule {
    category: Category,
    op: Op,
    value: u64,
    target: Target,
}

impl Rule {
    pub fn new(category: Category, op: Op, value: u64, target: Target) -> Self {
        Self { category, op, value, target }
    }

    pub fn fallback(target: Target) -> Self {
        Self {
            category: Category::X,
            op: Op::True,
            value: 0,
            target: target,
        }
    }

    pub fn apply(&self, part: &Part) -> &Target {
        let v = part[&self.category];
        let target = &self.target;
        match self.op {
            Op::LessThan if v < self.value => target,
            Op::GreaterThan if v > self.value => target,
            Op::True => target,
            _ => &Target::Pass,
        }
    }

    fn part_with_range(&self, part: &RangedPart, range: Range) -> RangedPart {
        let mut p = *part;
        p[self.category] = range;
        p
    }

    pub fn split(&self, part: &RangedPart) -> (RangedPartTarget, RangedPartTarget) {
        let range = part[&self.category];

        let split_value = match self.op {
            Op::LessThan => self.value,
            Op::GreaterThan => self.value + 1,
            Op::True => range.end(),
        };

        let (a, b) = range.split_at(split_value);
        let mut pa = self.part_with_range(part, a);
        let mut pb = self.part_with_range(part, b);
        if self.op == Op::GreaterThan {
            (pa, pb) = (pb, pa)
        }

        (
            RangedPartTarget { part: pa, target: self.target.clone() },
            RangedPartTarget { part: pb, target: Target::Pass },
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl Workflow {
    pub fn new(name: String, rules: Vec<Rule>) -> Self {
        Self { name, rules }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn apply(&self, part: &Part) -> &Target {
        self.rules.iter()
            .map(|rule| rule.apply(part))
            .find(|&target| target != &Target::Pass)
            .unwrap_or(&Target::Pass)
    }

    pub fn split(&self, part: &RangedPart) -> Vec<RangedPartTarget> {
        let mut targets: Vec<RangedPartTarget> = Vec::new();

        let mut rule_target_stack: Vec<(usize, RangedPartTarget)> = Vec::new();
        rule_target_stack.push((0, RangedPartTarget { part: *part, target: Target::Pass }));

        let mut handle = |t: &RangedPartTarget| {
            if t.part.is_empty() {
                return false;
            }

            if t.target == Target::Pass {
                true
            } else {
                targets.push(t.clone());
                false
            }
        };

        while let Some((rule_ix, target)) = rule_target_stack.pop() {
            let (a, b) = self.rules[rule_ix].split(&target.part);
            if handle(&a) {
                rule_target_stack.push((rule_ix + 1, a));
            }
            if handle(&b) {
                rule_target_stack.push((rule_ix + 1, b));
            }
        }

        targets
    }
}

#[derive(Debug, Clone)]
pub struct PartsSystem {
    pub(crate) workflows: Vec<Workflow>,
    pub(crate) index: HashMap<String, usize>,
}

impl PartsSystem {
    const MAX_STEPS: u64 = 1000;

    pub fn new(workflows: Vec<Workflow>) -> Self {
        let index = workflows.iter()
            .enumerate()
            .map(|(i, w)| (w.name().to_owned(), i))
            .collect();

        Self { workflows, index }
    }

    pub fn value(&self, part: &Part) -> u64 {
        let mut ix = self.index["in"];
        for _ in 0..Self::MAX_STEPS {
            let workflow = &self.workflows[ix];
            let target = workflow.apply(part);
            match target {
                Target::Accept => {
                    return part.sum();
                },
                Target::Reject => {
                    return 0;
                },
                Target::Workflow(w) => {
                    ix = self.index[w];
                },
                Target::Pass => {
                    ix += 1;
                }
            }
        }

        panic!("Exceeded max {} steps", Self::MAX_STEPS)
    }

    pub fn combinations(&self, part: &RangedPart) -> u64 {
        let mut sum = 0;
        let mut stack: Vec<(usize, RangedPart)> = Vec::new();
        stack.push((self.index["in"], *part));

        let mut steps = 0;
        while let Some((workflow_ix, part)) = stack.pop() {
            let workflow = &self.workflows[workflow_ix];
            let ranged_targets = workflow.split(&part);
            for target in ranged_targets {
                match target.target {
                    Target::Accept => {
                        sum += target.part.product();
                    },
                    Target::Reject => {
                    },
                    Target::Workflow(w) => {
                        stack.push((self.index[&w], target.part));
                    },
                    Target::Pass => {
                        stack.push((workflow_ix + 1, part));
                    }
                }
            }

            steps += 1;
            if steps > Self::MAX_STEPS {
                panic!("Exceeded max {} steps", Self::MAX_STEPS);
            }
        }

        sum
    }
}
