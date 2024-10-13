# Start_gaming

Just for fun

# Warming

**_This function is only available on Windows._**

# Documentation

https://docs.rs/game_start_macro/0.1.0/game_start_macro/

# Inspiration

from:

> https://github.com/Charley-xiao/nogenshin
> and **Rewirite It In Rust**

## How to use

add this:

```Rust
[dependencies]
start_gaming = "0.1.0"
game_start_macro = "0.1.0"
```

to your workspace.

To download the dependencies for your Rust project, run the following command in the terminal:

```powershell
cargo build
```

This command will read the _Cargo.toml_ file and download and compile all the listed dependencies.

# Example Code

```Rust
use game_start_macro::start_game;

#[start_game(WutheringWaves)]
fn main() {
    println!("Hello, world!");
    add(1, 1);
    panic!("This is a panic!");
}

#[start_game(Genshin)]
fn add(a: i32, b: i32) -> i32 {
    a + b
}

```

If the method triggers panic, then he will open Genshin Impact or Wuthering Waves.
Just aim to relax you.

# Errors

      This function will return an error if the game is not found in the registry.

# Panics

      This function will return an PANIC if the game executable is not found.

# Safety

      This function is not marked as unsafe, but it is not safe to use in a concurrent environment.
      It is just for fun.
