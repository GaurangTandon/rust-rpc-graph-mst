#[tarpc::service]
pub trait World {
    /// Returns a greeting for name.
    async fn hello(name: String) -> String;
}