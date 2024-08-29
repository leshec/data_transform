use std::collections::hash_map::Entry;
use std::collections::HashMap;

static DATA: &str = include_str!("../collins");

use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let mut file = File::options().append(true).open("collins_appended")?;
    let mut dict: HashMap<String, String> = HashMap::new();

    for line in DATA.split("\n") {
        let original: String = line.trim().to_string().to_lowercase();
        let mut word: Vec<char> = line.trim().chars().collect();
        word.sort();
        let alpha: String = word.iter().collect();
        let alpha: String = alpha.to_lowercase();

        match dict.entry(alpha) {
            Entry::Vacant(e) => {
                e.insert(original);
            }
            Entry::Occupied(mut e) => {
                e.get_mut().push_str(&(" ".to_owned() + &original));
            }
        }
    }

    let mut list: Vec<String> = Vec::new();

    for (key, val) in dict.iter() {
        list.push(format!("{key}:{val}"));
    }

    list.sort();
    for item in list.iter() {
        writeln!(&mut file, "{item}")?;
        println!("{item}");
    }

    println!("Count is {}", list.len());

    Ok(())
}
