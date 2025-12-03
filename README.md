# hardcore-snake
## Installing
Download the appropriate binary from the Releases page.

**Note**: you need to put the snake configuration file `config.toml` next to the game binary file.

You can find that file in the repo's root.

### Config
Example of configuration file `config.toml`:

```toml
# Example of snake configuration
width = 50
height = 25
pause_time = 80  # pause time between tacts; the less the value is – the faster snake is
enable_maze = true  # whether to build maze or play on a free board
maze_freedom = 0.8  # how much of a perfectly build maze should be destroyed for the game to be playable
```

**Note**: every entry from this example must be present in the config, otherwise the game won't run.

## Building
You'll need to have Rust and Cargo installed.

Clone the repo, and in the root of the repo, run:
```sh
cargo run --release
```

This will build a `release` version of binary in `target/release/`.
