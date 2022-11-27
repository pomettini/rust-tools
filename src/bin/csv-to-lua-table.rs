#![feature(map_try_insert)]

use csv::*;
use itertools::*;
use serde::*;
use std::collections::*;
use std::fs::File;
use std::io::Write;

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
    // identifier: String,
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

fn process_type_efficacy() -> String {
    let file = include_str!("../../csv/type_efficacy.csv");

    let mut reader = Reader::from_reader(file.as_bytes());

    let type_efficacy: Vec<TypeEfficacy> = reader
        .deserialize::<TypeEfficacy>()
        .map(|efficacy| efficacy.unwrap())
        .collect();

    let mut tree: BTreeMap<u64, BTreeMap<u64, bool>> = BTreeMap::new();

    type_efficacy.iter().for_each(|efficacy| {
        if tree.get(&efficacy.damage).is_none() {
            tree.insert(efficacy.damage, BTreeMap::default());
        }

        tree.get_mut(&efficacy.damage)
            .map(|val| val.insert(efficacy.target, efficacy.factor > 100));
    });

    tree.into_iter().fold(String::new(), |mut acc, value| {
        let values = value.1.into_iter().fold(String::new(), |mut acc, val| {
            acc.push_str(&format!("\t[{}] = {}, \n", val.0, val.1));
            acc
        });
        acc.push_str(&format!("[{}] = {{\n{}}}, \n", value.0, values));
        acc
    })
}

fn process_moves() -> String {
    let file = include_str!("../../csv/moves.csv");

    let mut reader = Reader::from_reader(file.as_bytes());

    let moves: Vec<Move> = reader
        .deserialize::<Move>()
        .map(|move_| move_.unwrap())
        .collect();

    let attack_moves: Vec<Move> = moves
        .into_iter()
        .filter(|move_| move_.power.is_some())
        .collect();

    let mut tree: BTreeMap<u64, Vec<u64>> = BTreeMap::new();

    attack_moves.iter().for_each(|move_| {
        if tree.get(&move_.type_id).is_none() {
            tree.insert(move_.type_id, Vec::default());
        }

        if let Some(val) = tree.get_mut(&move_.type_id) {
            val.push(move_.id)
        }
    });

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

    let moves: Vec<MoveName> = reader
        .deserialize::<MoveName>()
        .map(|move_| move_.unwrap())
        .collect();

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
    write_to_file("output/type_efficacy.txt", &process_type_efficacy());
    write_to_file("output/moves.txt", &process_moves());
    write_to_file("output/move_names.txt", &process_move_names());
}
