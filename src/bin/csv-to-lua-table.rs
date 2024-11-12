use csv::Reader;
use itertools::Itertools;
use rust_tools::get_type_efficacy;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Write;
use std::result::Result;

const EMPTY_TYPE: u64 = 0;
const EFFECTIVE: u64 = 100;
const SUPER_EFFECTIVE: u64 = 200;

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

fn process_pokemon_names(input: &str) -> String {
    let mut reader = Reader::from_reader(input.as_bytes());

    let pokemon_names = reader.deserialize::<Pokemon>().map(Result::unwrap);

    pokemon_names
        .into_iter()
        .fold(String::new(), |mut acc, value| {
            let id = value.id;
            // First letter is capitalized
            let name = value.name[0..1].to_uppercase() + &value.name[1..];
            acc.push_str(&format!("[{id}] = \"{name}\", \n"));
            acc
        })
}

fn process_pokemon_types(input: &str) -> String {
    let mut reader = Reader::from_reader(input.as_bytes());

    let pokemon_types: Vec<PokemonType> = reader
        .deserialize::<PokemonType>()
        .map(Result::unwrap)
        .collect();

    let mut pokemon_types_tree: BTreeMap<u64, (u64, u64)> = BTreeMap::new();

    pokemon_types.iter().for_each(|pokemon| match pokemon.slot {
        1 => {
            pokemon_types_tree.insert(pokemon.id, (pokemon.type_id, EMPTY_TYPE));
        }
        2 => {
            pokemon_types_tree.insert(
                pokemon.id,
                (pokemon_types_tree[&pokemon.id].0, pokemon.type_id),
            );
        }
        _ => (),
    });

    pokemon_types_tree
        .into_iter()
        .fold(String::new(), |mut acc, value| {
            let id = value.0;
            let first_type = value.1 .0;
            let second_type = value.1 .1;
            acc.push_str(&format!("[{id}] = {{{first_type}, {second_type}}}, \n"));
            acc
        })
}

fn process_type_efficacy(input: &str) -> String {
    let mut reader = Reader::from_reader(input.as_bytes());

    let type_efficacy: Vec<TypeEfficacy> = reader
        .deserialize::<TypeEfficacy>()
        .map(Result::unwrap)
        .collect();

    let mut type_efficacy_tree: BTreeMap<u64, BTreeMap<u64, u64>> = BTreeMap::new();

    for efficacy in &type_efficacy {
        if type_efficacy_tree.get(&efficacy.damage).is_none() {
            type_efficacy_tree.insert(efficacy.damage, BTreeMap::default());
        }

        type_efficacy_tree
            .get_mut(&efficacy.damage)
            .map(|val| val.insert(efficacy.target, efficacy.factor));
    }

    type_efficacy_tree
        .into_iter()
        .fold(String::new(), |mut acc, value| {
            let damage_id = value.0;
            let values = value.1.into_iter().fold(String::new(), |mut acc, val| {
                let target_id = val.0;
                let amount = val.1;
                acc.push_str(&format!("\t[{target_id}] = {amount}, \n"));
                acc
            });
            acc.push_str(&format!("[{damage_id}] = {{\n{values}}}, \n"));
            acc
        })
}

fn process_moves(input: &str) -> String {
    let mut reader = Reader::from_reader(input.as_bytes());

    let moves = reader
        .deserialize::<Move>()
        .map(std::result::Result::unwrap);

    let attack_moves: Vec<Move> = moves
        .into_iter()
        .filter(|move_| move_.power.is_some())
        .collect();

    let mut type_moves_tree: BTreeMap<u64, Vec<u64>> = BTreeMap::new();

    for move_ in &attack_moves {
        if type_moves_tree.get(&move_.type_id).is_none() {
            type_moves_tree.insert(move_.type_id, Vec::default());
        }

        if let Some(val) = type_moves_tree.get_mut(&move_.type_id) {
            val.push(move_.id);
        }
    }

    type_moves_tree
        .into_iter()
        .fold(String::new(), |mut acc, value| {
            let type_ = value.0;
            let moves = value.1.iter().join(", ");
            acc.push_str(&format!("[{type_}] = {{ {moves} }}, \n"));
            acc
        })
}

fn process_move_names(input: &str) -> String {
    let mut reader = Reader::from_reader(input.as_bytes());

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
            let id = value.0;
            let name = &value.1;
            acc.push_str(&format!("[{id}] = \"{name}\", \n"));
            acc
        })
}

fn process_pokemon_weaknesses(input: &str) -> String {
    let mut reader = Reader::from_reader(input.as_bytes());

    let pokemon_types: Vec<PokemonType> = reader
        .deserialize::<PokemonType>()
        .map(Result::unwrap)
        .collect();

    let mut pokemon_types_tree: BTreeMap<u64, (u64, u64)> = BTreeMap::new();

    pokemon_types.iter().for_each(|pokemon| match pokemon.slot {
        1 => {
            pokemon_types_tree.insert(pokemon.id, (pokemon.type_id, 0));
        }
        2 => {
            pokemon_types_tree.insert(
                pokemon.id,
                (pokemon_types_tree[&pokemon.id].0, pokemon.type_id),
            );
        }
        _ => (),
    });

    let mut index_weaknesses_tree: BTreeMap<u64, Vec<u64>> = BTreeMap::new();

    for pokemon in &pokemon_types_tree {
        let mut weak_types: Vec<u64> = Vec::new();
        let pokemon_id = *pokemon.0;
        let first_type = &pokemon.1 .0;
        let second_type = &pokemon.1 .1;
        get_type_efficacy()
            .iter()
            .for_each(|type_| match second_type {
                &EMPTY_TYPE => {
                    if type_.1[first_type] == SUPER_EFFECTIVE {
                        weak_types.push(*type_.0);
                    }
                }
                _ => {
                    if type_.1[first_type] * type_.1[second_type] > (EFFECTIVE * EFFECTIVE) {
                        weak_types.push(*type_.0);
                    }
                }
            });
        index_weaknesses_tree.insert(pokemon_id, weak_types);
    }

    index_weaknesses_tree
        .into_iter()
        .fold(String::new(), |mut acc, value| {
            let id = value.0;
            let weak_types = value.1.iter().join(", ");
            acc.push_str(&format!("[{id}] = {{ {weak_types} }}, \n"));
            acc
        })
}

fn write_to_file(path: &str, buf: &str) {
    let mut file = File::create(path).unwrap();
    file.write_all(buf.as_bytes()).unwrap();
}

fn main() {
    // TODO: Refactor all of this with builder pattern
    write_to_file(
        "output/pokemon_names.txt",
        &process_pokemon_names(include_str!("../../csv/pokemon.csv")),
    );
    write_to_file(
        "output/pokemon_types.txt",
        &process_pokemon_types(include_str!("../../csv/pokemon_types.csv")),
    );
    write_to_file(
        "output/pokemon_weaknesses.txt",
        &process_pokemon_weaknesses(include_str!("../../csv/pokemon_types.csv")),
    );
    write_to_file(
        "output/type_efficacy.txt",
        &process_type_efficacy(include_str!("../../csv/type_efficacy.csv")),
    );
    write_to_file(
        "output/moves.txt",
        &process_moves(include_str!("../../csv/moves.csv")),
    );
    write_to_file(
        "output/move_names.txt",
        &process_move_names(include_str!("../../csv/move_names.csv")),
    );
}

#[test]
fn test_pokemon_names() {
    let mut input = String::new();
    input.push_str("id,identifier,species_id,height,weight,base_experience,order,is_default\n");
    input.push_str("1,bulbasaur,1,7,69,64,1,1\n");
    input.push_str("905,enamorus-incarnate,905,16,480,,,1\n");

    let mut output = String::new();
    output.push_str("[1] = \"Bulbasaur\", \n");
    output.push_str("[905] = \"Enamorus-incarnate\", \n");

    let result = process_pokemon_names(&input);

    assert_eq!(result, output);
}

#[test]
fn test_pokemon_types() {
    let mut input = String::new();
    input.push_str("pokemon_id,type_id,slot\n");
    input.push_str("1,12,1\n");
    input.push_str("1,4,2\n");
    input.push_str("897,8,1\n");

    let mut output = String::new();
    output.push_str("[1] = {12, 4}, \n");
    output.push_str("[897] = {8, 0}, \n");

    let result = process_pokemon_types(&input);

    assert_eq!(result, output);
}

#[test]
fn test_pokemon_weaknesses() {
    let mut input = String::new();
    input.push_str("pokemon_id,type_id,slot\n");
    input.push_str("1,12,1\n");
    input.push_str("1,4,2\n");
    input.push_str("895,16,1\n");

    let mut output = String::new();
    output.push_str("[1] = { 3, 10, 14, 15 }, \n");
    output.push_str("[895] = { 15, 16, 18 }, \n");

    let result = process_pokemon_weaknesses(&input);

    assert_eq!(result, output);
}

#[test]
fn test_type_efficacy() {
    let mut input = String::new();
    input.push_str("damage_type_id,target_type_id,damage_factor\n");
    input.push_str("1,1,100\n");
    input.push_str("18,17,200\n");

    let mut output = String::new();
    output.push_str("[1] = {\n\t[1] = 100, \n}, \n");
    output.push_str("[18] = {\n\t[17] = 200, \n}, \n");

    let result = process_type_efficacy(&input);

    assert_eq!(result, output);
}

#[test]
fn test_moves() {
    let mut input = String::new();
    input.push_str("id,identifier,generation_id,type_id,power,pp,accuracy,priority,target_id,damage_class_id,effect_id,effect_chance,contest_type_id,contest_effect_id,super_contest_effect_id\n");
    input.push_str("1,pound,1,1,40,35,100,0,10,2,1,,5,1,5\n");
    input.push_str("839,barb-barrage,8,4,60,15,100,0,10,2,3,30,,,\n");

    let mut output = String::new();
    output.push_str("[1] = { 1 }, \n");
    output.push_str("[4] = { 839 }, \n");

    let result = process_moves(&input);

    assert_eq!(result, output);
}

#[test]
fn test_move_names() {
    let mut input = String::new();
    input.push_str("move_id,local_language_id,name\n");
    input.push_str("1,1,はたく\n");
    input.push_str("1,8,Botta\n");
    input.push_str("825,1,アストラルビット\n");
    input.push_str("825,8,Schegge Astrali\n");

    let mut output = String::new();
    output.push_str("[1] = \"Botta\", \n");
    output.push_str("[825] = \"Schegge Astrali\", \n");

    let result = process_move_names(&input);

    assert_eq!(result, output);
}
