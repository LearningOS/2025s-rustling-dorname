/*
    bfs
    This problem requires you to implement a basic BFS algorithm
*/

use std::collections::VecDeque;

// Define a graph
struct Graph {
    adj: Vec<Vec<usize>>,
}

impl Graph {
    // Create a new graph with n vertices
    fn new(n: usize) -> Self {
        Graph {
            adj: vec![vec![]; n],
        }
    }

    // Add an edge to the graph
    fn add_edge(&mut self, src: usize, dest: usize) {
        self.adj[src].push(dest);
        self.adj[dest].push(src);
    }

    // Perform a breadth-first search on the graph, return the order of visited nodes
    fn bfs_with_return(&self, start: usize) -> Vec<usize> {

        let mut visit_order = vec![];
        // fn dfs(graph: &Graph, visit_order: &mut Vec<usize>, start: usize) {
        //     // 1、如果节点已经访问过，则跳过
        //     if visit_order.contains(&start) {
        //         return;
        //     }
        //     // 2、将节点加入访问顺序
        //     visit_order.push(start);
        //     // 3、依次访问节点可访问的节点
        //     for neighbor in &graph.adj[start] {
        //         dfs(graph, visit_order, *neighbor);
        //     }
        // }
        fn bfs(graph: &Graph, visit_order: &mut Vec<usize>, start: usize) {
            //1、初始化队列
            let mut queue = VecDeque::new();
            queue.push_back(start);
            //2、依次出队
            while let Some(node) = queue.pop_front() {
                // 3、如果节点已经访问过，则跳过
                if visit_order.contains(&node) {
                    continue;
                }
                // 4、将节点加入访问顺序
                visit_order.push(node);
                // 5、将节点可访问的节点加入队列
                for neighbor in &graph.adj[node] {
                    queue.push_back(*neighbor);
                }
            }
        }
        bfs(&self, &mut visit_order, start);
        // dfs(&self, &mut visit_order, start); dfs 最终的访问顺序和测试bfs的顺序是不一样的
        visit_order
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs_all_nodes_visited() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(0, 4);
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(1, 4);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 4, 2, 3]);
    }

    #[test]
    fn test_bfs_different_start() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        let visited_order = graph.bfs_with_return(2);
        assert_eq!(visited_order, vec![2, 1, 0]);
    }

    #[test]
    fn test_bfs_with_cycle() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 0);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 2]);
    }

    #[test]
    fn test_bfs_single_node() {
        let mut graph = Graph::new(1);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0]);
    }
}
