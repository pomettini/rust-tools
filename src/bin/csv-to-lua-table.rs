use csv::*;
use serde::*;
use std::collections::*;
use std::fs::File;
use std::io::Write;
use std::path::Path;

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

#[derive(Debug, Deserialize)]
struct MoveName {
    move_id: u64,
    local_language_id: u64,
    name: String,
}

fn process_type_efficacy() {
    let file = include_str!("type_efficacy.csv");

    let mut rdr = Reader::from_reader(file.as_bytes());

    let efficacy: Vec<TypeEfficacy> = rdr
        .deserialize::<TypeEfficacy>()
        .map(|element| element.unwrap())
        .collect();

    let mut tree: HashMap<u64, HashMap<u64, bool>> = HashMap::new();

    for element in &efficacy {
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

    let moves: Vec<Move> = rdr
        .deserialize::<Move>()
        .map(|element| element.unwrap())
        .collect();

    let attack_moves: Vec<Move> = moves
        .into_iter()
        .filter(|move_| move_.power.is_some())
        .collect();

    let mut tree: HashMap<u64, Vec<u64>> = HashMap::new();

    for element in &attack_moves {
        if tree.get(&element.type_id).is_none() {
            tree.insert(element.type_id, Vec::default());
        }

        if let Some(val) = tree.get_mut(&element.type_id) {
            val.push(element.id)
        }
    }

    dbg!(tree);
}

fn process_move_names() {
    let file = include_str!("move_names.csv");

    let mut rdr = Reader::from_reader(file.as_bytes());

    let moves: Vec<MoveName> = rdr
        .deserialize::<MoveName>()
        .map(|element| element.unwrap())
        .collect();

    let move_names: HashMap<u64, String> = moves
        .into_iter()
        .filter(|move_| move_.local_language_id == 8)
        .fold(HashMap::new(), |mut acc, value| {
            acc.entry(value.move_id).or_insert(value.name);
            acc
        });

    let output = move_names
        .into_iter()
        .fold(String::new(), |mut acc, value| {
            acc.push_str(&format!("[{}] = \"{}\", \n", value.0, &value.1));
            acc
        });

    println!("{}", output);

    write_to_file("move_names.txt", &output);
}

fn write_to_file(path: &str, buf: &str) {
    let mut file = File::create(path).unwrap();
    file.write_all(buf.as_bytes()).unwrap();
}

fn main() {
    // process_type_efficacy();
    // process_moves();
    // process_move_names()
}
