use std::{cmp::Ordering, collections::BinaryHeap};

// Dijkstra solution
impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let n = n as usize;
        let k = k as usize - 1;
        let mut graph: Vec<Vec<(u8, i32)>> = vec![vec![]; n];
        for edge in times.into_iter() {
            graph[edge[0] as usize - 1].push((edge[1] as u8 - 1, edge[2]));
        }

        let mut visit_next: BinaryHeap<MinScored<i32, u8>> = BinaryHeap::new();
        let mut delays: Vec<Option<i32>> = vec![None; n];
        let mut visit: Vec<bool> = vec![false; n];
        delays[k] = Some(0);
        visit_next.push(MinScored(0, k as u8));

        while let Some(MinScored(node_delay, node)) = visit_next.pop() {
            if visit[node as usize] {
                continue;
            }
            visit[node as usize] = true;

            for &(n, edge_delay) in graph[node as usize].iter()
                    .filter(|&&(n, _)| !visit[n as usize]) {
                let new_dist = node_delay + edge_delay;
                match delays[n as usize] {
                    Some(d) => {
                        if new_dist < d {
                            visit_next.push(MinScored(new_dist, n));
                            delays[n as usize] = Some(new_dist);
                        }
                    }
                    None => {
                        visit_next.push(MinScored(new_dist, n));
                        delays[n as usize] = Some(new_dist);
                    }
                }
            }
        }

        let mut max_delay = 0;
        for d in delays.into_iter() {
            match d {
                None => return -1,
                Some(d) => max_delay = max_delay.max(d),
            }
        }

        max_delay
    }
}

/// MinScored taken from my favorite "petgraph" crate.
#[derive(Copy, Clone, Debug)]
pub struct MinScored<K, T>(pub K, pub T);

impl<K: PartialOrd, T> PartialEq for MinScored<K, T> {
    #[inline]
    fn eq(&self, other: &MinScored<K, T>) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl<K: PartialOrd, T> Eq for MinScored<K, T> {}

impl<K: PartialOrd, T> PartialOrd for MinScored<K, T> {
    #[inline]
    fn partial_cmp(&self, other: &MinScored<K, T>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<K: PartialOrd, T> Ord for MinScored<K, T> {
    #[inline]
    fn cmp(&self, other: &MinScored<K, T>) -> Ordering {
        let a = &self.0;
        let b = &other.0;
        if a == b {
            Ordering::Equal
        } else if a < b {
            Ordering::Greater
        } else if a > b {
            Ordering::Less
        } else if a.ne(a) && b.ne(b) {
            // these are the NaN cases
            Ordering::Equal
        } else if a.ne(a) {
            // Order NaN less, so that it is last in the MinScore order
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}
