use std::io;
use std::hash::Hash;

fn main() {
    println!("Hello, world!");
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
    Underground,
    SeaLevel,
    Flying,
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
    Altar{flux:Flux},
    Lair{entity:Entity},
    Ruin{race:Race},
    Graveyard{race:Race},
}

struct Entity {
    name: char,

}
struct Race {
    name: String,
    alignment: Alignment,
    favourite_biome: Biome,
}

enum TownSize {
    Hamlet,
    Town,
    City,
    Metropolis,
}

struct Alignment {
    flux : Flux,
    intensity : u8,
}
enum Flux {
    Genesis,
    Change,
    Destruction,
}

