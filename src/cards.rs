#[derive(Debug)]
pub struct Card {
    pub front: String,
    pub back: String,
}

pub fn get_deck() -> Vec<Card> {
    let hiragana_cards = vec![
        Card {
            front: String::from("あ"),
            back: String::from("A"),
        },
        Card {
            front: String::from("い"),
            back: String::from("I"),
        },
        Card {
            front: String::from("う"),
            back: String::from("U"),
        },
        Card {
            front: String::from("え"),
            back: String::from("E"),
        },
        Card {
            front: String::from("お"),
            back: String::from("O"),
        },
        Card {
            front: String::from("か"),
            back: String::from("Ka"),
        },
        Card {
            front: String::from("き"),
            back: String::from("Ki"),
        },
        Card {
            front: String::from("く"),
            back: String::from("Ku"),
        },
        Card {
            front: String::from("け"),
            back: String::from("Ke"),
        },
        Card {
            front: String::from("こ"),
            back: String::from("Ko"),
        },
        Card {
            front: String::from("さ"),
            back: String::from("Sa"),
        },
        Card {
            front: String::from("し"),
            back: String::from("Shi"),
        },
        Card {
            front: String::from("す"),
            back: String::from("Su"),
        },
        Card {
            front: String::from("せ"),
            back: String::from("Se"),
        },
        Card {
            front: String::from("そ"),
            back: String::from("So"),
        },
        Card {
            front: String::from("た"),
            back: String::from("Ta"),
        },
        Card {
            front: String::from("ち"),
            back: String::from("Chi"),
        },
        Card {
            front: String::from("つ"),
            back: String::from("Tsu"),
        },
        Card {
            front: String::from("て"),
            back: String::from("Te"),
        },
        Card {
            front: String::from("と"),
            back: String::from("To"),
        },
        Card {
            front: String::from("な"),
            back: String::from("Na"),
        },
        Card {
            front: String::from("に"),
            back: String::from("Ni"),
        },
        Card {
            front: String::from("ぬ"),
            back: String::from("Nu"),
        },
        Card {
            front: String::from("ね"),
            back: String::from("Ne"),
        },
        Card {
            front: String::from("の"),
            back: String::from("No"),
        },
        Card {
            front: String::from("は"),
            back: String::from("Ha"),
        },
        Card {
            front: String::from("ひ"),
            back: String::from("Hi"),
        },
        Card {
            front: String::from("ふ"),
            back: String::from("Fu"),
        },
        Card {
            front: String::from("へ"),
            back: String::from("He"),
        },
        Card {
            front: String::from("ほ"),
            back: String::from("Ho"),
        },
        Card {
            front: String::from("ま"),
            back: String::from("Ma"),
        },
        Card {
            front: String::from("み"),
            back: String::from("Mi"),
        },
        Card {
            front: String::from("む"),
            back: String::from("Mu"),
        },
        Card {
            front: String::from("め"),
            back: String::from("Me"),
        },
        Card {
            front: String::from("も"),
            back: String::from("Mo"),
        },
        Card {
            front: String::from("や"),
            back: String::from("Ya"),
        },
        Card {
            front: String::from("ゆ"),
            back: String::from("Yu"),
        },
        Card {
            front: String::from("よ"),
            back: String::from("Yo"),
        },
        Card {
            front: String::from("ら"),
            back: String::from("Ra"),
        },
        Card {
            front: String::from("り"),
            back: String::from("Ri"),
        },
        Card {
            front: String::from("る"),
            back: String::from("Ru"),
        },
        Card {
            front: String::from("れ"),
            back: String::from("Re"),
        },
        Card {
            front: String::from("ろ"),
            back: String::from("Ro"),
        },
        Card {
            front: String::from("わ"),
            back: String::from("Wa"),
        },
        Card {
            front: String::from("を"),
            back: String::from("Wo"),
        },
        Card {
            front: String::from("ん"),
            back: String::from("N"),
        },
    ];
    hiragana_cards
}
