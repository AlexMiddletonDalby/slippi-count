mod to_string;

use to_string::character_to_string;
use to_string::stage_to_string;

use std::fs;
use std::io;

fn main() {
    let file = fs::File::open("src/test-slippi-files/Day 3-Game_20210718T091908.slp")
        .expect("File could not be read");

    let mut buffer = io::BufReader::new(file);
    let game = peppi::game(&mut buffer, None, None).expect("File could not be parsed");

    println!("Stage: {}", stage_to_string(game.start.stage));

    let char1 = game.start.players[0].character;
    println!("Character 1: {}", character_to_string(char1));

    let char2 = game.start.players[1].character;
    println!("Character 2: {}", character_to_string(char2));
}
