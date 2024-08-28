use crate::tarnished::Tarnished;

pub fn print_as_table(tarnished: Vec<Tarnished>) {
    println!("{0: <10} | {1: <10} |", "Tarnished", "Strikes");
    for tarnished in tarnished {
        println!("{0: <10} | {1: <10} |", tarnished.name, tarnished.strikes);
    }
}

pub fn print_strikes(name: &str, strikes: i8) {
    println!("{} has now {} strikes!", name, strikes);
}
