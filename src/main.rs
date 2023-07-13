use axum_assessment_headers::run;

#[tokio::main]
async fn main() {
    match run().await {
        Ok(_) => println!("Server exited okay"),
        Err(error) => eprintln!("Server exited with error: {error}"),
    };
}
