use zero2prod_rust::run;

fn spawn_app() {
    let server = zero2prod_rust::run().expect("Failed to bind address");
    let _ = tokio::spawn(server);
}

#[tokio::test]
async fn health_check_works() {
    spawn_app();
}
