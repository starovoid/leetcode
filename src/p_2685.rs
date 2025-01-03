impl Solution {
    pub fn count_complete_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut graph: Vec<Vec<usize>> = vec![Vec::new(); n];
        for edge in edges {
            graph[edge[0] as usize].push(edge[1] as usize);
            graph[edge[1] as usize].push(edge[0] as usize);
        }

        let mut answer = 0;
        let mut visit = vec![false; n];
        let mut stack = Vec::new();

        // DFS on each cc
        for start in 0..n {
            if visit[start] {
                continue;
            }

            let mut nodes_in_cc = 0;
            let mut edges_in_cc = 0;
            stack.push(start);

            while let Some(v) = stack.pop() {
                visit[v] = true;
                nodes_in_cc += 1;
                edges_in_cc += graph[v].len();
                for &u in graph[v].iter() {
                    if !visit[u] {
                        visit[u] = true;
                        stack.push(u);
                    }
                }
            }

            // Сheck completeness. The edges were counted twice.
            if edges_in_cc / 2 == nodes_in_cc * (nodes_in_cc - 1) / 2 {
                answer += 1;
            }
        }

        answer
    }
}
