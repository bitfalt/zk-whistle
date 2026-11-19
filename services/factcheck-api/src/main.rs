#[tokio::main]
async fn main() {
    api::serve(zk::Scope::FactCheck, 8081).await;
}
