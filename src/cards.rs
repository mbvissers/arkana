use std::error::Error;
use std::fs::File;
use std::path::Path;

#[derive(Debug, Default)]
pub struct Card {
    pub front: String,
    pub back: String,
}

pub fn get_deck(path: String, has_headers: bool) -> Result<Vec<Card>, Box<dyn Error>> {
    let path = if path == "" {
        "src/csv/hiragana_test.csv"
    } else {
        path.as_str()
    };

    let file = File::open(Path::new(path))?;

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(has_headers)
        .from_reader(file);

    let mut deck: Vec<Card> = Vec::new();

    for result in reader.records() {
        let record = result?;
        deck.push(Card {
            front: String::from(record.get(0).unwrap()),
            back: String::from(record.get(1).unwrap()),
        });
    }

    Ok(deck)
}
