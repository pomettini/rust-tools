fn main() {
    let input = include_str!("../../csv/pokemon.csv");

    let pokemons: Vec<&str> = input
        .lines()
        .skip(1)
        .map(|line| line.split(',').collect())
        .collect();

    dbg!(pokemons);

    let mut output = String::new();
}
