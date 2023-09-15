// `tokio::test` is the testing equivalent of `tokio::main`.
// It also spares you from having to specify the `#[test]` attribute.
//
// You can inspect what code gets generated using 
// `cargo expand --test health_check` (<- name of the test file)

#[tokio::test]
async fn health_check_works(){
    // Arrange
    spawn_app();
    // we need to bring in "reqwest"
    // to perform HTTP requests against our application.
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// Launch out application in the background ~somehow~
fn spawn_app()-> String{
    let listenenr = std::net::TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");

    let port = listener.local_addr().unwrap().port();
    // writeing below !!!!!!!
    let server = zero2prod::run("127.0.0.1:0").expect("Failed to bind address.");

    let _ = tokio::spawn(server);
}