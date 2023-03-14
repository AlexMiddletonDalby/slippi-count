use std::fs;
use std::io;

use peppi::model::frame::Frame;
use peppi::model::game::Frames;
use peppi::model::game::Player;

fn count_lcancels(frames: Vec<Frame<2>>, player_index: usize) -> i32 {
    let mut l_cancel_count: i32 = 0;

    for curent_frame in frames {
        let frame_data = curent_frame.ports[player_index].leader;

        let l_cancel_data = frame_data.post.l_cancel;
        if let Some(l_cancel) = l_cancel_data {
            if let Some(l_cancel_successful) = l_cancel {
                if l_cancel_successful {
                    l_cancel_count += 1;
                }
            }
        }
    }

    return l_cancel_count;
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

fn main() {
    let connect_code = "MD#1"; //this should be specified by the user

    //Todo: Find all files in directory and do this for each
    let file = fs::File::open("src/test-slippi-files/Day 3-Game_20210718T092212.slp")
        .expect("File could not be read");

    let mut buffer = io::BufReader::new(file);
    let game = peppi::game(&mut buffer, None, None).expect("File could not be parsed");

    let found_player_index =
        get_player_index_for_connect_code(connect_code.to_owned(), &game.start.players);

    let mut total_l_cancels: i32 = 0;

    if let Some(player_index) = found_player_index {
        println!("Found player with index {}", player_index);

        let frames = game.frames;

        let mut lcancel_count: i32 = 0;
        match frames {
            Frames::P2(data) => lcancel_count = count_lcancels(data, player_index),
            _ => println!("Ignored because wrong number of players"),
        }

        println!("Number of successful l-cancels in game: {}", lcancel_count);
        total_l_cancels += lcancel_count;
    } else {
        println!("Player with specified connect code not found. Skipping...")
    }

    println!("Total number of successful l-cancels: {}", total_l_cancels);
}
