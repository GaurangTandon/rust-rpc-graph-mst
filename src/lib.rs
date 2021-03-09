#[tarpc::service]
pub trait GraphicalWorld {
    /// Creates a new graph
    async fn add_graph(name: String, n: u32);
    /// Add one edge to existing graph
    async fn add_edge(name: String, u: u32, v: u32, w: u32);
    /// Gets MST weight for existing graph
    async fn get_mst(name: String) -> i32;
    /// Clears graph for easy debugging
    async fn clear_graph(name: String);
}

use std::collections::BTreeSet;
type Edge = (u32, u32, u32);
type EdgeSet = BTreeSet<Edge>;

pub struct DSU {
    parents: Vec<usize>,
    sizes: Vec<usize>,
    n: usize,
}

impl DSU {
    pub fn new(n: usize) -> Self {
        let parents: Vec<usize> = (0..=n).collect();
        let sizes = vec![1; n as usize + 1];

        Self { parents, sizes, n }
    }

    fn find(&mut self, node: usize) -> usize {
        if self.parents[node] != node {
            self.parents[node] = self.find(self.parents[node]);
        }

        return self.parents[node];
    }

    fn merge(&mut self, a: usize, b: usize) -> bool {
        let mut pa = self.find(a);
        let mut pb = self.find(b);

        if pa == pb {
            return false;
        }

        let sa = self.sizes[pa];
        let sb = self.sizes[pb];
        if sa < sb {
            let x = pa;
            pa = pb;
            pb = x;
        }

        self.sizes[pa] += self.sizes[pb];
        self.parents[pb] = pa;

        true
    }

    fn is_connected(&mut self) -> bool {
        let parent = self.find(1);

        for i in 2..=self.n {
            if self.find(i) != parent {
                return false;
            }
        }

        true
    }

    pub fn get_mst(&mut self, edges: &EdgeSet) -> i32 {
        let mut weight = 0;

        for (w, u, v) in edges {
            if self.merge(*u as usize, *v as usize) {
                weight += *w;
            }
        }

        if !self.is_connected() { -1 } else { weight as i32 }
    }
}

