# ElixirKit.PubSub demo

Docs:

- [ElixirKit](https://elixirkit.hexdocs.pm/readme.html)
- [ElixirKit.PubSub](https://elixirkit.hexdocs.pm/ElixirKit.PubSub.html)

Run Elixir from RustTauri apps and exchange messages over **PubSub**.

The docs describe `ElixirKit.PubSub` as a Publish/Subscribe server to exchange messages between
Elixir and native code. PubSub is designed to be started alongside `elixirkit::PubSub` from a native
app.

So there are two corresponding PubSub components:

- `elixirkit::PubSub` on the Rust/native side
- `ElixirKit.PubSub` on the Elixir side

On the Rust side, use `elixirkit::elixir` to start Elixir and `elixirkit::PubSub` to exchange
messages:

```rust
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
```

On the Elixir side, start `ElixirKit.PubSub` under a supervision tree. Use
`ElixirKit.PubSub.subscribe/1` to listen to messages and `ElixirKit.PubSub.broadcast/2` to send
messages to the Rust side:

```elixir
Mix.install([
  :elixirkit
])

children = [
  {ElixirKit.PubSub, connect: System.fetch_env!("ELIXIRKIT_PUBSUB"), on_exit: &System.stop/0}
]

{:ok, _supervisor} =
  Supervisor.start_link(children, strategy: :one_for_one)

IO.puts("[elixir] PubSub connected")

# Subscribe
ElixirKit.PubSub.subscribe("hello")

IO.puts("[elixir] sending message to Rust")

ElixirKit.PubSub.broadcast(
  "hello",
  "Hello from Elixir!"
)

receive do
  message ->
    IO.puts("[elixir] received from Rust: #{inspect(message)}")
end
```

## Run

Run the app with `cargo run`:

```bash
cargo run
   Compiling hello-elixirkit v0.1.0 (/home/scossar/projects/rust/hello-elixirkit)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/hello-elixirkit`
[rust] PubSub listening at tcp://127.0.0.1:34121
[rust] starting Elixir
[elixir] PubSub connected
[elixir] sending message to Rust
[rust] received from Elixir: Hello from Elixir!
[rust] sending reply
[elixir] received from Rust: "hello from Rust!"
[rust] Elixir exited with exit status: 0
```
