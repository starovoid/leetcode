impl Solution {
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        if n == 1 {
            return true;
        }

        let mut graph: Vec<Vec<usize>> = (0..n).map(|_| vec![]).collect();
        for edge in edges.into_iter() {
            graph[edge[0] as usize].push(edge[1] as usize);
            graph[edge[1] as usize].push(edge[0] as usize);
        }

        let mut stack = vec![source as usize];
        let mut used = vec![false; n as usize];
        while let Some(node) = stack.pop() {
            for &v in graph[node].iter() {
                if v == destination as usize {
                    return true;
                }
                if !used[v] {
                    stack.push(v);
                    used[v] = true;
                }
            }
        }

        false
    }
}
