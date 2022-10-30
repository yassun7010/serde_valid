#[tokio::main]
async fn main() -> Result<(), crate::Error> {
    load_env();

    aide::gen::on_error(|error| {
        println!("{error}");
    });

    aide::gen::extract_schemas(true);

    let state = AppState::default().await;

    let mut api = OpenApi {
        // Swagger UI does not support v3.1.0 OpenAPI.
        openapi: "3.0.0",
        ..Default::default()
    };

    let app = api::router::make(&mut api, state).layer(Extension(Arc::new(api)));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    println!("OpenAPI docs are accessible at http://{}/docs", &addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
