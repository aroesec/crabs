use warp;

#[tokio::main]
async fn main() {
    warp::serve(),
    .run(([127,0,0,1], 3000)),
    .await;
}
