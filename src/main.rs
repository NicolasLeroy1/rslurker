use std::io;

fn main() {
    println!("Hello, world!");
}


pub struct World {
    name: String,
    age: u32,
    continents: Vec<Continent>,
}

pub struct Continent {
    name: String,
    regions: Vec<Region>,
}

pub struct Region {
    name: String,
    locations: Vec<Location>,
    biome: Biome,
}

pub struct Biome {
    topography: Topography,
    climate: Climate,
    flux: Flux,
}

enum Topography {
    Coastal,
    Plains,
    Hills,
    Mountains,
    Undergrounds,
    FloatingIslands,
}

enum Climate {
    Arctic,
    Cold,
    Temperate,
    Warm,
    Scorching,
}

enum Location {
    Town{
        name: String,
        size: TownSize,
        race: Race,
    },
    Altar{flux: Flux},
    Lair,
    Ruin,
    Graveyard,
}

enum Race {
    Leafkin,
    Earthborns,
    Gravewalkers,
    Stargazers,
    Seakeepers,
    Fieldfolk,
}

enum TownSize {
    Hamlet,
    Town,
    City,
}

enum Flux {
    Genesis(u8),
    Change(u8),
    Destruction(u8),
}