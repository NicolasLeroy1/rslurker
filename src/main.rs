use std::io;
use std::collections::HashMap;

fn main() {
 let mut game_state = GameState::Start;
}


enum GameState {
    Start,
    SaveAndLoad{world: World, player: Player},
    WorldAwait{world: World, player: Player},
    WorldSimulation{world: World, player: Player},
}

struct Player {
    name: String,
}

struct Pawn {
    name: String,
    pawn_type: PawnType,
    traits: Vec<PawnTrait>,
}

enum PawnTrait {
}

enum PawnType {
    Summon{skill : HashMap<ActionType,u8>},
    Creature(Creature),
    Follower{tribe:Tribe, action_type:ActionType},
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

enum ContinentHeight {
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

enum Topography {
    Coastal,
    Plains,
    Hills,
    Mountains,
}

enum Climate {
    Arctic,
    Temperate,
    Scorching,
}

enum Location {
    Town{
        name: String,
        size: TownSize,
        tribe: Tribe,
    },
    Church{god:God},
    Lair{creature : Creature},
    Ruin{tribe:Tribe},
}
enum Creature {
    Dragon,
    Spider,
    Bear,
    Wolf,
    Snake,
    Troll,
    Spirit,
}

struct God{
    name: String,
}

enum Race{
    Elf,
    Orc,
    Naga,
    Atlantean,
    Angel,
    Demon,
    Dwarf,
    Goblin,
}

struct Tribe {
    name : String,
    race : Race,
    culture : Culture,
}

enum Culture {
    Bellicist,
    Mercantile,
    Industrious,
    Deceptive,
}

enum ActionType{
    Charm,
    Fight,
    Work,
    Investigate,
}

struct PlayerRessources{
    Influence : i64,
    Secrets : i64,
    Materials : i64,
}

enum TownSize {
    Hamlet,
    Town,
    City,
    Metropolis,
}


