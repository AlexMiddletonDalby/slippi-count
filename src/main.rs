use std::fs;
use std::io;

use clap::Parser;

use peppi::model::frame::Frame;
use peppi::model::game::Frames;
use peppi::model::game::Player;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    directory: String,

    #[arg(short, long)]
    connect_code: String,
}

fn get_player_index_for_connect_code(connect_code: String, players: &Vec<Player>) -> Option<usize> {
    for (index, player) in players.iter().enumerate() {
        if let Some(netplay_data) = &player.netplay {
            if netplay_data.code == connect_code {
                return Some(index);
            }
        }
    }

    return None;
}

fn does_game_feature_connect_code(file: String, connect_code: &String) -> bool {
    let file = fs::File::open(file).expect("File could not be read");

    let mut buffer = io::BufReader::new(file);
    let game = peppi::game(&mut buffer, None, None).expect("File could not be parsed");

    let found_player_index =
        get_player_index_for_connect_code(connect_code.to_owned(), &game.start.players);

    return found_player_index.is_some();
}

fn main() {
    let args = Args::parse();

    let directory = args.directory;
    let connect_code = args.connect_code;

    let files = fs::read_dir(directory).expect("Files could not be read from given directory");

    let mut num_games_with_connect_code: i32 = 0;
    for file in files {
        if let Ok(file) = file {
            let file_name = file
                .path()
                .into_os_string()
                .into_string()
                .expect("Invlaid path");

            if does_game_feature_connect_code(file_name.to_owned(), &connect_code) {
                num_games_with_connect_code += 1;
            }
        }
    }

    println!(
        "Number of games featuring specified connect-code: {}",
        num_games_with_connect_code
    );
}
