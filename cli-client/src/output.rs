use crate::tarnished::Tarnished;
use comfy_table::Table;

pub fn print_as_table(tarnished: Vec<Tarnished>) {
    if tarnished.is_empty() {
        println!("No one has been tarnished yet!");
        return;
    }

    let mut table = Table::new();
    table.set_header(vec!["Tarnished", "Strikes"]);

    for tarnished in tarnished {
        table.add_row(vec![tarnished.name, tarnished.strikes.to_string()]);
    }

    println!("{table}");
}

pub fn print_strikes(name: &str, strikes: u8) {
    println!("{} has now {} strikes!", name, strikes);
}
