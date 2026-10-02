Mix.install([
  :elixirkit
])

# alias Mix.PubSub

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
