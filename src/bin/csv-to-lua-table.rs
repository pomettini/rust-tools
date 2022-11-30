#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

use csv::Reader;
use itertools::Itertools;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Write;
use std::result::Result;

#[derive(Debug, Deserialize)]
struct Pokemon {
    id: u64,
    #[serde(rename = "identifier")]
    name: String,
}

#[derive(Debug, Deserialize)]
struct PokemonType {
    #[serde(rename = "pokemon_id")]
    id: u64,
    type_id: u64,
    slot: u64,
}

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
    type_id: u64,
    power: Option<u64>,
}

#[derive(Debug, Deserialize)]
struct MoveName {
    move_id: u64,
    #[serde(rename = "local_language_id")]
    language: u64,
    name: String,
}

fn process_pokemon_names() -> String {
    let file = include_str!("../../csv/pokemon.csv");

    let mut reader = Reader::from_reader(file.as_bytes());

    let pokemon_names = reader.deserialize::<Pokemon>().map(Result::unwrap);

    pokemon_names
        .into_iter()
        .fold(String::new(), |mut acc, value| {
            acc.push_str(&format!(
                "[{}] = \"{}\", \n",
                value.id,
                // First letter is capitalized
                value.name[0..1].to_uppercase() + &value.name[1..]
            ));
            acc
        })
}

fn process_type_efficacy() -> String {
    let file = include_str!("../../csv/type_efficacy.csv");

    let mut reader = Reader::from_reader(file.as_bytes());

    let type_efficacy: Vec<TypeEfficacy> = reader
        .deserialize::<TypeEfficacy>()
        .map(Result::unwrap)
        .collect();

    let mut tree: BTreeMap<u64, BTreeMap<u64, bool>> = BTreeMap::new();

    for efficacy in &type_efficacy {
        if tree.get(&efficacy.damage).is_none() {
            tree.insert(efficacy.damage, BTreeMap::default());
        }

        tree.get_mut(&efficacy.damage)
            .map(|val| val.insert(efficacy.target, efficacy.factor > 100));
    }

    tree.into_iter().fold(String::new(), |mut acc, value| {
        let values = value.1.into_iter().fold(String::new(), |mut acc, val| {
            acc.push_str(&format!("\t[{}] = {}, \n", val.0, val.1));
            acc
        });
        acc.push_str(&format!("[{}] = {{\n{values}}}, \n", value.0));
        acc
    })
}

fn process_moves() -> String {
    let file = include_str!("../../csv/moves.csv");

    let mut reader = Reader::from_reader(file.as_bytes());

    let moves = reader
        .deserialize::<Move>()
        .map(std::result::Result::unwrap);

    let attack_moves: Vec<Move> = moves
        .into_iter()
        .filter(|move_| move_.power.is_some())
        .collect();

    let mut tree: BTreeMap<u64, Vec<u64>> = BTreeMap::new();

    for move_ in &attack_moves {
        if tree.get(&move_.type_id).is_none() {
            tree.insert(move_.type_id, Vec::default());
        }

        if let Some(val) = tree.get_mut(&move_.type_id) {
            val.push(move_.id);
        }
    }

    tree.into_iter().fold(String::new(), |mut acc, value| {
        acc.push_str(&format!(
            "[{}] = {{ {} }}, \n",
            value.0,
            value.1.iter().join(", ")
        ));
        acc
    })
}

fn process_move_names() -> String {
    let file = include_str!("../../csv/move_names.csv");

    let mut reader = Reader::from_reader(file.as_bytes());

    let moves = reader.deserialize::<MoveName>().map(Result::unwrap);

    let move_names: BTreeMap<u64, String> = moves
        .into_iter()
        .filter(|move_| move_.language == 8)
        .fold(BTreeMap::new(), |mut acc, value| {
            acc.entry(value.move_id).or_insert(value.name);
            acc
        });

    move_names
        .into_iter()
        .fold(String::new(), |mut acc, value| {
            acc.push_str(&format!("[{}] = \"{}\", \n", value.0, &value.1));
            acc
        })
}

fn write_to_file(path: &str, buf: &str) {
    let mut file = File::create(path).unwrap();
    file.write_all(buf.as_bytes()).unwrap();
}

fn main() {
    write_to_file("output/pokemon_names.txt", &process_pokemon_names());
    write_to_file("output/type_efficacy.txt", &process_type_efficacy());
    write_to_file("output/moves.txt", &process_moves());
    write_to_file("output/move_names.txt", &process_move_names());
}
