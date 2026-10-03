fn main() {
    // Rust opens a local TCP PubSub endpoint.
    // Port 0 means "pick an available port".
    let pubsub = elixirkit::PubSub::listen("tcp://127.0.0.1:0").expect("failed to start PubSub");

    println!("[rust] PubSub listening at {}", pubsub.url());

    let pubsub_for_reply = pubsub.clone();

    // Subscribe before starting E
    pubsub.subscribe("hello", move |message| {
        println!(
            "[rust] received from Elixir: {}",
            String::from_utf8_lossy(message)
        );

        println!("[rust] sending reply");

        pubsub_for_reply
            .broadcast("hello", b"hello from Rust!")
            .expect("failed to broadcast");
    });

    println!("[rust] starting Elixir");

    let status = elixirkit::elixir(&["hello.exs"])
        .env("ELIXIRKIT_PUBSUB", pubsub.url())
        .status()
        .expect("failed to start Elixir");

    println!("[rust] Elixir exited with {status}")
}
