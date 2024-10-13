use game_start_macro::start_game;

#[start_game(WutheringWaves)]
fn main() {
    println!("Hello, world!");
    panic!("Hello, world!");
}

#[start_game(Genshin)]
fn add(a: i32, b: i32) -> i32 {
    a + b
}
