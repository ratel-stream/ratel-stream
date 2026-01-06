use ratel_stream::run;

#[monoio::main]
async fn main() {
    run().await;
}
