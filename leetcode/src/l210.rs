// https://leetcode.com/problems/course-schedule-ii/

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

struct TopoOrder {
    order: Vec<i32>,
    marked: Vec<bool>,
}

impl TopoOrder {
    pub fn new(g: &Graph) -> Self {
        let vertice_count = g.vertice_count();

        let mut to = TopoOrder {
            order: Vec::with_capacity(vertice_count),
            marked: vec![false; vertice_count],
        };

        (0..vertice_count).for_each(|v| {
            if !to.marked[v] {
                to.search(g, v);
            }
        });

        to.order.reverse();

        to
    }

    fn search(&mut self, g: &Graph, s: usize) {
        self.marked[s] = true;

        g.adj(s).for_each(|&v| {
            if !self.marked[v] {
                self.search(g, v);
            }
        });

        self.order.push(s as i32);
    }
}

pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
    let mut g = Graph::new(num_courses as usize);

    prerequisites.iter().for_each(|edge| {
        g.add_edge(edge[1] as usize, edge[0] as usize);
    });

    let cycle = Cycle::new(&g);

    if cycle.exists {
        return vec![];
    } else {
        TopoOrder::new(&g).order
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(find_order(2, vec![vec![1, 0]]), vec![0, 1]);
    }
}
