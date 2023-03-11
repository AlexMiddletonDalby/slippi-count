use peppi::model::enums::character;
use peppi::model::enums::stage;

pub fn stage_to_string(stage: stage::Stage) -> String {
    match stage {
        stage::Stage::FOUNTAIN_OF_DREAMS => "Fountain of Dreams".to_owned(),
        stage::Stage::BATTLEFIELD => "Battlefield".to_owned(),
        stage::Stage::POKEMON_STADIUM => "Pokémon Stadium".to_owned(),
        stage::Stage::YOSHIS_STORY => "Yoshi's Story".to_owned(),
        stage::Stage::DREAM_LAND_N64 => "Dream Land 64".to_owned(),
        stage::Stage::FINAL_DESTINATION => "Final Destination".to_owned(),
        _ => "Illegal stage".to_owned(),
    }
}

pub fn character_to_string(character: character::External) -> String {
    match character {
        character::External::CAPTAIN_FALCON => "Captain Falcon".to_owned(),
        character::External::DONKEY_KONG => "DK".to_owned(),
        character::External::FOX => "Fox".to_owned(),
        character::External::GAME_AND_WATCH => "Mr. Game & Watch".to_owned(),
        character::External::KIRBY => "Kirby".to_owned(),
        character::External::BOWSER => "Bowser".to_owned(),
        character::External::LINK => "Link".to_owned(),
        character::External::LUIGI => "Luigi".to_owned(),
        character::External::MARIO => "Mario".to_owned(),
        character::External::MARTH => "Marth".to_owned(),
        character::External::MEWTWO => "Mewtwo".to_owned(),
        character::External::NESS => "Ness".to_owned(),
        character::External::PEACH => "Pichu".to_owned(),
        character::External::PIKACHU => "Pikachu".to_owned(),
        character::External::ICE_CLIMBERS => "Ice Climbers".to_owned(),
        character::External::JIGGLYPUFF => "Jigglypuff".to_owned(),
        character::External::SAMUS => "Samus".to_owned(),
        character::External::YOSHI => "Yoshi".to_owned(),
        character::External::ZELDA => "Zelda".to_owned(),
        character::External::SHEIK => "Sheik".to_owned(),
        character::External::FALCO => "Falco".to_owned(),
        character::External::YOUNG_LINK => "Young Link".to_owned(),
        character::External::DR_MARIO => "Dr. Mario".to_owned(),
        character::External::ROY => "Roy".to_owned(),
        character::External::PICHU => "Pichu".to_owned(),
        character::External::GANONDORF => "Ganondorf".to_owned(),
        _ => "Unrecognised character".to_owned(),
    }
}
