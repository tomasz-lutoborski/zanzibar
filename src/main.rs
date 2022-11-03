mod player;
mod setup;
mod game;

use std::collections::HashMap;
use crate::game::run;
use crate::player::{
    PlayerType,
    Player
};
use crate::setup::{get_players, print_welcome};

fn main() {
    print_welcome();

    let players = get_players();

    run(players);
}
