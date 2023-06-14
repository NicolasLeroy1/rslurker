use std::io;
use std::hash::Hash;

fn main() {
    let mut world = World{name: String::from("Altiria"), age: 0, continents: Vec::new()};
    world.continents.push(Continent{name: String::from("SeaLevelContinent"), regions: Vec::new(), height: ContinentHeight::SeaLevel});
    world.continents.push(Continent{name: String::from("UndergroundContinent"), regions: Vec::new(), height: ContinentHeight::Underground});
    world.continents.push(Continent{name: String::from("FlyingContinent"), regions: Vec::new(), height: ContinentHeight::Flying});  
    world.continents[0].regions.push(Region{name: String::from("Region1"), locations: Vec::new(), biome: Biome{topography: Topography::Coastal, climate: Climate::Temperate}});
    world.continents[0].regions.push(Region{name: String::from("Region2"), locations: Vec::new(), biome: Biome{topography: Topography::Plains, climate: Climate::Warm}});


    println!("In the world of {} there are {} continents", world.name, world.continents.len());

    for continent in &world.continents {
        println!("The continent of {} has {} regions", continent.name, continent.regions.len());
    }

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

enum Race {
    Elves,
    Dwarves,
    Orcs,
    Avians,
}

enum TownSize {
    Hamlet,
    Town,
    City,
    Metropolis,
}


