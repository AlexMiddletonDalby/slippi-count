use std::fs;
use std::io;
use std::time::Duration;

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
    connect_code: &str,
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

fn does_game_feature_connect_code(game: &Game, connect_code: &str) -> bool {
    if let Some(players) = &game.metadata.players {
        if find_player_with_connect_code(connect_code, players).is_some() {
            return true;
        }
    }

    return false;
}

fn does_game_feature_any_connect_code(game: &Game, connect_codes: &Vec<&str>) -> bool {
    for code in connect_codes {
        if does_game_feature_connect_code(&game, code) {
            return true;
        }
    }

    return false;
}

fn get_game_duration(game: &Game) -> Duration {
    if let Some(length_in_frames) = game.metadata.duration {
        let duration = Duration::from_secs((length_in_frames / 60) as u64);

        return duration;
    }

    return Duration::from_secs(0);
}

fn main() {
    let args = Args::parse();

    let directory = args.directory;
    let connect_codes: Vec<&str> = args.connect_code.split(",").collect();

    let files = fs::read_dir(directory).expect("Files could not be read from given directory");

    let mut total_game_time = Duration::from_secs(0);

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
                if does_game_feature_any_connect_code(&game, &connect_codes) {
                    total_game_time += get_game_duration(&game);
                }
            }
        }
    }

    let secs = total_game_time.as_secs();
    let mins = secs as f32 / 60.0;
    let hours = mins / 60.0;
    let days = hours / 24.0;
    let weeks = days / 7.0;

    println!("You've played {:?} seconds of Melee, that's...", secs);
    println!("{:.2} minutes", mins);
    println!("{:.2} hours", hours);
    println!("{:.2} days", days);
    println!("or {:.2} weeks!", weeks);
    println!("Boy, that's a lot of Melee!")
}
