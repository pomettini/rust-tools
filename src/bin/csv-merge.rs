use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

fn multiple_csv_to_hashmap(multiple_csv: Vec<&str>) -> HashMap<&str, Vec<&str>> {
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();

    multiple_csv.into_iter().for_each(|csv| {
        csv.lines()
            .map(|line| line.split('|').collect::<Vec<&str>>())
            .for_each(|line_split| {
                let key = *line_split.first().unwrap();
                let value = *line_split.get(1).unwrap();
                map.entry(key).or_insert_with(Vec::new).push(value);
            });
    });

    map
}

fn change_parameters_format(input: &mut String) {
    // TODO: Replace with Regex for consistency
    *input = input.replace("[%", "{");
    *input = input.replace(']', "}");
}

fn main() {
    let mut input: Vec<&str> = Vec::new();
    input.push(include_str!("../../csv/brisca_en.txt"));
    input.push(include_str!("../../csv/brisca_it.txt"));
    input.push(include_str!("../../csv/brisca_es.txt"));

    let mut output = String::new();
    output.push_str("Key;Type;Desc;English;Italian;Spanish\n");

    multiple_csv_to_hashmap(input).into_iter().for_each(|key| {
        output.push_str(&format!("{};Text;;{}\n", key.0, key.1.join(";")));
    });

    // Changing terms format from [%something] to {something}
    // change_parameters_format(&mut output);

    let mut file = File::create("brisca_merged.csv").unwrap();
    file.write_all(output.as_bytes()).unwrap();
}
