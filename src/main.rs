use std::fs;
use std::io;
use std::sync::mpsc::channel;
use std::thread::available_parallelism;
use std::time::Duration;

use clap::Parser;
use threadpool::ThreadPool;

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
    let files = fs::read_dir(&directory).expect("Files could not be read from given directory");

    let mut total_game_time = Duration::from_secs(0);

    let threadpool = ThreadPool::new(usize::from(available_parallelism().unwrap()));

    let (sender, reciever) = channel();
    for file in files {
        let sender = sender.clone();
        let connect_code_arg = args.connect_code.clone();

        threadpool.execute(move || {
            let connect_codes: Vec<&str> = connect_code_arg.split(",").collect();

            if let Ok(file) = file {
                let file_name = file
                    .path()
                    .into_os_string()
                    .into_string()
                    .unwrap_or("".to_string());

                let file = fs::File::open(file_name).expect("File could not be read");
                let mut buffer = io::BufReader::new(file);
                let game = peppi::game(&mut buffer, None, None);

                if let Ok(game) = game {
                    if does_game_feature_any_connect_code(&game, &connect_codes) {
                        sender
                            .send(get_game_duration(&game).as_secs() as u32)
                            .unwrap()
                    }
                }
            }
        });
    }
    drop(sender);

    for message in reciever.iter() {
        total_game_time += Duration::from_secs(message as u64);
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
    println!("Boy, that's a lot of Melee!");
}
