std::collections::HashMap;
use std::io;

fn handle_input() -> u32 {
    let mut string = String::new();
    io::stdin().read_line(&mut string).unwrap();

    string.trim().parse().unwrap()
}

fn add_note(
    value: &mut u32,
    note_value: u32,
    mut banknotes: HashMap<u32, u32>,
) -> HashMap<u32, u32> {
    let mut counter = 0;
    banknotes.insert(note_value, counter);

    while *value / note_value > 0 {
        counter += 1;
        *value -= note_value;
    }

    banknotes.insert(note_value, counter);

    banknotes
}

fn banknotes(mut value: u32, notes: &Vec<u32>) -> HashMap<u32, u32> {
    let mut banknotes: HashMap<u32, u32> = HashMap::new();

    while value != 0 {
        for note in notes {
            banknotes = add_note(&mut value, *note, banknotes);
        }
    }

    banknotes
}

fn display_output(value: u32, notes: Vec<u32>, banknotes: HashMap<u32, u32>) {
    println!("{}", value);

    for note in &notes {
        println!("{} nota(s) de R$ {},00", banknotes[&note], note);
    }
}

fn main() {
    let notes = vec![100, 50, 20, 10, 5, 2, 1];

    let value = handle_input();
    let banknotes = banknotes(value, &notes);
    display_output(value, notes, banknotes);
}
