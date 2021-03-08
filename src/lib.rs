#[tarpc::service]
pub trait GraphicalWorld {
    /// Creates a new graph
    async fn add_graph(name: String, n: u32);
    /// Add one edge to existing graph
    async fn add_edge(name: String, u: u32, v: u32, w: u32);
    /// Gets MST weight for existing graph
    async fn get_mst(name: String) -> i32;
}