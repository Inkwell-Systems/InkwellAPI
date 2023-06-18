use inkwell_api::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let server = run().unwrap();
    server.await
}
