#[tokio::test]
async fn health_check_works() {
    // No .await, no .expect
    let addr = spawn_app();
    // We need to bring in `reqwest`
    // to perform HTTP requests against our application
    let client = reqwest::Client::new();
    // The really interesting things happened here, the .await here 
    // caused the `tokio` runtime to asynchronized to poll both client's
    // send event and the server's running event, but we only wait the 
    // `send` to finish. Once the `send` is completed, we will get a 
    // response from the server, and we do all kinds of checks. Whether
    // the checks are passed or failed, the test is over, it will close
    // along with the *server* running behind it.
    let response = client
        .get(format!("{}/health_check", addr))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

 // No .await call, therefore no need for `spawn_app` to be asyn cnow.
 // We are also running tests, so it is not worth it to propagate errors:
 // if we fail to perform the required setup we can just panic and crash
 // all the things.
fn spawn_app() -> String {
    let listener = std::net::TcpListener::bind("localhost:0")
        .expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = newsletter::run(listener).expect("Failed to bind address");
    // Launch the server as a background task
    // tokio::spawn returns a handle to the spawned future,
    // but we have no use for it here, hence the non-binding let
    let _ = tokio::spawn(server);
    format!("http://localhost:{}", port)
}