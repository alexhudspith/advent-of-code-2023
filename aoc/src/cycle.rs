use std::hash::Hash;
use indexmap::IndexMap;
use indexmap::map::Entry;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct NoCycle<T> {
    pub target: (usize, T),
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Cycle<T> {
    pub start: (usize, T),
    pub end: (usize, T),
    pub target_equiv: (usize, T),
    pub target: usize,
}

impl<T> Cycle<T> {
    pub fn cycle_len(&self) -> usize {
        self.end.0 - self.start.0
    }

    pub fn complete_cycles(&self) -> usize {
        (self.target - self.start.0) / self.cycle_len()
    }
}

fn swap_remove_or_insert<K: Eq + Hash, V>(map: &mut IndexMap<K, V>, key: K, value: V) -> Option<(usize, V, V)> {
    match map.entry(key) {
        Entry::Occupied(e) => {
            let index = e.index();
            let (_, existing_val) = map.swap_remove_index(index).unwrap();
            Some((index, existing_val, value))
        }
        Entry::Vacant(e) => {
            e.insert(value);
            None
        }
    }
}

pub fn find_in_cycle<I, K, V>(iter: I, target: usize) -> Result<Cycle<V>, NoCycle<V>>
    where
        I: IntoIterator<Item=(K, V)>,
        K: Eq + Hash,
        V: Clone,
{
    let mut seen: IndexMap<K, V> = IndexMap::new();
    let (cycle_start, cycle_start_val, cycle_end, cycle_end_val) = iter.into_iter()
        .take(target + 1)
        .enumerate()
        .filter_map(|(j, (key, val_j))|
            swap_remove_or_insert(&mut seen, key, val_j)
                .map(|(i, val_i, val_j)| (i, val_i, j, val_j))
        )
        .next()
        .ok_or_else(|| {
            let (_, val) = seen.swap_remove_index(target).expect("Iterator too short for target index");
            NoCycle { target: (target, val) }
        })?;

    let cycle_len = cycle_end - cycle_start;
    let target_equiv = ((target - cycle_start) % cycle_len) + cycle_start;
    eprintln!("Cycle at {cycle_start:?} -> {cycle_end:?}, need {target_equiv:?}");
    assert_eq!(cycle_end, seen.len() + 1);
    assert!(target_equiv < cycle_end);

    let target_equiv_val = if target_equiv == cycle_start {
        cycle_start_val.clone()
    } else {
        // If target_equiv was at the end, it was swap_removed with cycle_start
        let seen_ix = if target_equiv == seen.len() { cycle_start } else { target_equiv };
        seen.swap_remove_index(seen_ix).unwrap().1
    };

    Ok(Cycle {
        start: (cycle_start, cycle_start_val),
        end: (cycle_end, cycle_end_val),
        target_equiv: (target_equiv, target_equiv_val),
        target: target,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: [(i32, char); 10] = [
        (0, 'a'), (1, 'b'), (2, 'c'), (3, 'd'), (0, 'e'), (1, 'f'), (2, 'g'), (3, 'h'), (0, 'i'), (1, 'j')
    ];

    const OFFSET_TEST: [(i32, char); 10] = [
        (-2, 'z'), (-1, 'y'), (0, 'a'), (1, 'b'), (2, 'c'), (3, 'd'), (0, 'e'), (1, 'f'), (2, 'g'), (3, 'h')
    ];

    #[test]
    #[should_panic]
    fn empty() {
        let _ = find_in_cycle(Vec::<(i32, i32)>::new(), 0);
    }

    #[test]
    #[should_panic]
    fn too_short() {
        let _ = find_in_cycle(vec![(0, 0)], 1);
    }

    #[test]
    fn no_cycle_0() {
        let actual = find_in_cycle(TEST, 0);
        let expected = Err(NoCycle { target: (0, 'a') });
        assert_eq!(actual, expected);
    }

    #[test]
    fn no_cycle_3() {
        let actual = find_in_cycle(TEST, 3);
        let expected = Err(NoCycle { target: (3, 'd') });
        assert_eq!(actual, expected);
    }

    #[test]
    fn cycle_4_4() {
        let actual = find_in_cycle(TEST, 4);
        let expected = Ok(Cycle { start: (0, 'a'), end: (4, 'e'), target_equiv: (0, 'a'), target: 4 });
        assert_eq!(actual, expected);
    }

    #[test]
    fn cycle_4_5() {
        let actual = find_in_cycle(TEST, 5);
        let expected = Ok(Cycle { start: (0, 'a'), end: (4, 'e'), target_equiv: (1, 'b'), target: 5 });
        assert_eq!(actual, expected);
    }

    #[test]
    fn cycle_4_7() {
        let actual = find_in_cycle(TEST, 7);
        let expected = Ok(Cycle { start: (0, 'a'), end: (4, 'e'), target_equiv: (3, 'd'), target: 7 });
        assert_eq!(actual, expected);
    }

    #[test]
    fn cycle_4_8() {
        let actual = find_in_cycle(TEST, 8);
        let expected = Ok(Cycle { start: (0, 'a'), end: (4, 'e'), target_equiv: (0, 'a'), target: 8 });
        assert_eq!(actual, expected);
    }

    #[test]
    fn offset_cycle_4_6() {
        let actual = find_in_cycle(OFFSET_TEST, 6);
        let expected = Ok(Cycle { start: (2, 'a'), end: (6, 'e'), target_equiv: (2, 'a'), target: 6 });
        assert_eq!(actual, expected);
    }

    #[test]
    fn offset_cycle_4_7() {
        let actual = find_in_cycle(OFFSET_TEST, 7);
        let expected = Ok(Cycle { start: (2, 'a'), end: (6, 'e'), target_equiv: (3, 'b'), target: 7 });
        assert_eq!(actual, expected);
    }

    #[test]
    fn offset_cycle_4_8() {
        let actual = find_in_cycle(OFFSET_TEST, 8);
        let expected = Ok(Cycle { start: (2, 'a'), end: (6, 'e'), target_equiv: (4, 'c'), target: 8 });
        assert_eq!(actual, expected);
    }

    #[test]
    fn offset_cycle_4_9() {
        let actual = find_in_cycle(OFFSET_TEST, 9);
        let expected = Ok(Cycle { start: (2, 'a'), end: (6, 'e'), target_equiv: (5, 'd'), target: 9 });
        assert_eq!(actual, expected);
    }
}
