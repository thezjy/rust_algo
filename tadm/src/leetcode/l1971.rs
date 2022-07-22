// https://leetcode.com/problems/find-if-path-exists-in-graph/

#[derive(Debug)]
struct WeightedUnionFind {
    id: Vec<usize>,
    size: Vec<usize>,
}

impl WeightedUnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            id: (0..=n).collect(),
            size: vec![1; n],
        }
    }

    fn root(&self, i: usize) -> usize {
        let mut root = i;
        while root != self.id[root] {
            root = self.id[root]
        }

        // flatten the tree
        // while i != root {
        //     self.id[i] = root;
        //     i = self.id[root];
        // }

        root
    }

    pub fn union(&mut self, i: usize, j: usize) {
        let i_root = self.root(i);
        let j_root = self.root(j);

        if i_root != j_root {
            let i_size = self.size[i_root];
            let j_size = self.size[j_root];

            let total_size = i_size + j_size;

            if i_size > j_size {
                self.id[j_root] = i_root;
                self.size[i_root] = total_size;
            } else {
                self.id[i_root] = j_root;
                self.size[j_root] = total_size;
            }
        }
    }

    pub fn connected(&self, i: usize, j: usize) -> bool {
        self.root(i) == self.root(j)
    }
}

pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
    let mut wuf = WeightedUnionFind::new(n as usize);

    edges.iter().for_each(|pair| {
        wuf.union(pair[0] as usize, pair[1] as usize);
    });

    wuf.connected(source as usize, destination as usize)
}

struct Graph {
    adj_list: Vec<Vec<usize>>,
}

impl Graph {
    pub fn new(n: usize) -> Self {
        Self {
            adj_list: vec![vec![]; n],
        }
    }

    pub fn add_edge(&mut self, v: usize, w: usize) {
        self.adj_list[v].push(w);
        self.adj_list[w].push(v);
    }

    pub fn vertice_count(&self) -> usize {
        self.adj_list.len()
    }

    pub fn adj(&self, v: usize) -> impl Iterator<Item = &usize> {
        self.adj_list[v].iter()
    }
}

struct DFS {
    marked: Vec<bool>,
    id: Vec<usize>,
    component_count: usize,
}

impl DFS {
    pub fn new(g: &Graph) -> Self {
        let vertice_count = g.vertice_count();
        let mut dfs = Self {
            marked: vec![false; vertice_count],
            id: (0..vertice_count).collect(),
            component_count: 0,
        };

        (0..vertice_count).for_each(|v| {
            if !dfs.marked[v] {
                dfs.search(g, v);
                dfs.component_count += 1;
            }
        });

        dfs
    }

    pub fn connected(&self, u: usize, w: usize) -> bool {
        self.id[u] == self.id[w]
    }

    fn search(&mut self, g: &Graph, v: usize) {
        self.marked[v] = true;
        self.id[v] = self.component_count;
        g.adj(v).for_each(|&w| {
            if !self.marked[w] {
                self.search(g, w);
            }
        });
    }
}

pub fn valid_path_dfs(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
    let mut g = Graph::new(n as usize);

    edges.iter().for_each(|pair| {
        g.add_edge(pair[0] as usize, pair[1] as usize);
    });

    let dfs = DFS::new(&g);

    dfs.connected(destination as usize, source as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        assert!(valid_path(
            3,
            vec![vec![0, 1], vec![1, 2], vec![2, 0]],
            0,
            2
        ));
    }

    #[test]
    fn t2() {
        assert!(!valid_path(
            6,
            vec![vec![0, 1], vec![0, 2], vec![3, 5], vec![5, 4], vec![4, 3]],
            0,
            5
        ));
    }

    #[test]
    fn t1_dfs() {
        assert!(valid_path_dfs(
            3,
            vec![vec![0, 1], vec![1, 2], vec![2, 0]],
            0,
            2
        ));
    }

    #[test]
    fn t2_dfs() {
        assert!(!valid_path_dfs(
            6,
            vec![vec![0, 1], vec![0, 2], vec![3, 5], vec![5, 4], vec![4, 3]],
            0,
            5
        ));
    }
}
