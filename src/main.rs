use std::collections::HashMap;
use std::io;

fn main() {
    let mut game = Game::Start;
    run(&mut game);
}

pub fn run(game: &mut Game) {
    loop {
        match game {
            Game::Start => {
                // Handle start state
            }
            Game::Loading {
                player,
                config,
                filepath,
            } => {
                // Handle loading state
            }
            Game::PlayerTurn {
                world,
                player,
                config,
            } => {
                // Handle player turn
            }
            Game::Simulation {
                world,
                player,
                config,
            } => {
                // Handle simulation
            }
        }
    }
}

pub enum Game {
    Start,
    Loading {
        player: Player,
        config: Config,
        filepath: String,
    },
    PlayerTurn {
        world: World,
        player: Player,
        config: Config,
    },
    Simulation {
        world: World,
        player: Player,
        config: Config,
    },
}

pub struct Config {
    seed: u64,
}

pub struct Player {
    name: String,
}

pub struct Pawn {
    name: String,
    pawn_type: PawnType,
    traits: Vec<PawnTrait>,
}

pub enum PawnTrait {}

pub enum PawnType {
    Summon {
        skill: HashMap<ActionType, u8>,
    },
    Creature(Creature),
    Follower {
        tribe: Tribe,
        action_type: ActionType,
    },
}
pub struct World {
    name: String,
    age: u64,
    continents: Vec<Continent>,
}

pub struct Continent {
    name: String,
    regions: Vec<Region>,
    height: ContinentHeight,
}

pub enum ContinentHeight {
    Undergound,
    Abyssal,
    Celestial,
    Ground,
}

pub struct Region {
    name: String,
    locations: Vec<Location>,
    biome: Biome,
}

pub struct Biome {
    topography: Topography,
    climate: Climate,
}

pub enum Topography {
    Coastal,
    Plains,
    Hills,
    Mountains,
}

pub enum Climate {
    Arctic,
    Temperate,
    Scorching,
}

enum Location {
    Town {
        name: String,
        size: TownSize,
        tribe: Tribe,
    },
    Church {
        god: God,
    },
    Lair {
        creature: Creature,
    },
    Ruin {
        tribe: Tribe,
    },
}
pub enum Creature {
    Dragon,
    Spider,
    Bear,
    Wolf,
    Snake,
    Troll,
    Spirit,
}

pub struct God {
    name: String,
}

pub enum Race {
    Elf,
    Orc,
    Naga,
    Atlantean,
    Angel,
    Demon,
    Dwarf,
    Goblin,
}

pub struct Tribe {
    name: String,
    race: Race,
    culture: Culture,
}

pub enum Culture {
    Mercantile,
    Bellicist,
    Industrious,
    Deceptive,
}

pub enum ActionType {
    Charm,
    Fight,
    Work,
    Investigate,
}

pub struct PlayerRessources {
    influence: i64,
    secrets: i64,
    materials: i64,
}

pub enum TownSize {
    Hamlet,
    Town,
    City,
    Metropolis,
}
