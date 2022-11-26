use csv::*;
use serde::*;
use std::collections::*;

#[derive(Debug, Deserialize)]
struct TypeEfficacy {
    #[serde(rename = "damage_type_id")]
    damage: u64,
    #[serde(rename = "target_type_id")]
    target: u64,
    #[serde(rename = "damage_factor")]
    factor: u64,
}

#[derive(Debug, Deserialize)]
struct Move {
    id: u64,
    identifier: String,
    type_id: u64,
    power: Option<u64>,
}

fn process_type_efficacy() {
    let file = include_str!("type_efficacy.csv");

    let mut rdr = Reader::from_reader(file.as_bytes());

    let type_efficacy: Vec<TypeEfficacy> = rdr
        .deserialize::<TypeEfficacy>()
        .map(|element| element.unwrap())
        .collect();

    let mut tree: HashMap<u64, HashMap<u64, bool>> = HashMap::new();

    for element in &type_efficacy {
        if tree.get(&element.damage).is_none() {
            tree.insert(element.damage, HashMap::default());
        }

        tree.get_mut(&element.damage)
            .map(|val| val.insert(element.target, element.factor > 100));
    }

    dbg!(tree);
}

fn process_moves() {
    let file = include_str!("moves.csv");

    let mut rdr = Reader::from_reader(file.as_bytes());

    let moves: Vec<Result<Move>> = rdr.deserialize::<Move>().collect();

    let attack_moves: Vec<Result<Move>> = moves
        .into_iter()
        .filter(|move_| move_.as_ref().unwrap().power.is_some())
        .collect();

    dbg!(attack_moves.len());
}

fn main() {
    process_type_efficacy();
    // process_moves();
}
