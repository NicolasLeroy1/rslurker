use std::io;
use std::hash::Hash;

fn main() {

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
    Church{god:God},
    PrimalAltar,
    Ruin{race:Race},
}

struct God{
    name: String,
}
struct Entity {
    name: char,

}
struct Race {
    name: String,
    favourite_biome: Biome,
}

enum TownSize {
    Hamlet,
    Town,
    City,
    Metropolis,
}



enum MaterialType{
    Wood,
    Stone,
    Metal,
    Food,
    MagicStone,
}

enum MaterialTier{
    Poor,
    Basic,
    Fine,
    Exquisite,
}

