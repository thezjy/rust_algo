// https://leetcode.com/problems/course-schedule/

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
    }

    pub fn vertice_count(&self) -> usize {
        self.adj_list.len()
    }

    pub fn adj(&self, v: usize) -> impl Iterator<Item = &usize> {
        self.adj_list[v].iter()
    }
}

struct Cycle {
    marked: Vec<bool>,
    on_stack: Vec<bool>,
    exists: bool,
}

impl Cycle {
    fn new(g: &Graph) -> Self {
        let vertice_count = g.vertice_count();

        let mut cycle = Cycle {
            marked: vec![false; vertice_count],
            on_stack: vec![false; vertice_count],
            exists: false,
        };

        for v in 0..vertice_count {
            if cycle.exists {
                break;
            } else if !cycle.marked[v] {
                cycle.search(&g, v);
            }
        }

        cycle
    }

    fn search(&mut self, g: &Graph, s: usize) {
        self.marked[s] = true;
        self.on_stack[s] = true;

        g.adj(s).for_each(|&v| {
            if self.on_stack[v] {
                self.exists = true;
                return;
            } else if !self.marked[v] {
                self.search(g, v);
            }
        });

        self.on_stack[s] = false;
    }
}

pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    let mut g = Graph::new(num_courses as usize);

    prerequisites.iter().for_each(|edge| {
        g.add_edge(edge[1] as usize, edge[0] as usize);
    });

    let cycle = Cycle::new(&g);

    !cycle.exists
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        assert!(can_finish(2, vec![vec![1, 0]]));
    }
}
