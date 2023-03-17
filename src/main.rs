use std::fs;
use std::io;

use clap::Parser;

use peppi::model::game::Game;
use peppi::model::metadata::Player;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    directory: String,

    #[arg(short, long)]
    connect_code: String,
}

fn find_player_with_connect_code<'a>(
    connect_code: &String,
    players: &'a Vec<Player>,
) -> Option<&'a Player> {
    for player in players {
        if let Some(netplay_data) = &player.netplay {
            if netplay_data.code == connect_code.to_owned() {
                return Some(player);
            }
        }
    }

    return None;
}

fn does_game_feature_connect_code(game: &Game, connect_code: &String) -> bool {
    if let Some(players) = &game.metadata.players {
        if find_player_with_connect_code(connect_code, players).is_some() {
            return true;
        }
    }

    return false;
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
                .expect("Invalid path");

            let file = fs::File::open(file_name).expect("File could not be read");
            let mut buffer = io::BufReader::new(file);
            let game = peppi::game(&mut buffer, None, None);

            if let Ok(game) = game {
                if does_game_feature_connect_code(&game, &connect_code) {
                    num_games_with_connect_code += 1;
                }
            }
        }
    }

    println!(
        "Number of games featuring specified connect-code: {}",
        num_games_with_connect_code
    );
}
