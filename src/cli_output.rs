use comfy_table::{self, presets, Attribute::Bold, Cell, CellAlignment::Right};
use klayan::{
    hands::{Finger, Hand},
    stats::Stats,
};
use strum::IntoEnumIterator;

pub fn print_output(stats: Stats) {
    let mut header: Vec<Cell> = Vec::new();
    let mut rows: [Vec<Cell>; 8] = Default::default();

    // fingers
    header.push(Cell::new("finger").add_attribute(Bold));
    header.push(Cell::new("usage").set_alignment(Right).add_attribute(Bold));
    header.push(Cell::new("sfb").add_attribute(Bold));
    header.push(Cell::new("sku").add_attribute(Bold));

    for (i, finger) in Finger::iter().enumerate() {
        let name = match finger {
            Finger::LeftPinky => "left  pinky",
            Finger::LeftRing => "left  ring",
            Finger::LeftMiddle => "left  middle",
            Finger::LeftIndex => "left  index",
            Finger::RightIndex => "right index",
            Finger::RightMiddle => "right middle",
            Finger::RightRing => "right ring",
            Finger::RightPinky => "right pinky",
            Finger::Thumb => continue,
        };
        rows[i].push(Cell::new(name));

        let usage = stats.unigrams.finger_usage.get(&finger).unwrap();
        rows[i].push(Cell::new(format!("{usage:.1}")).set_alignment(Right));

        let sfb = stats.bigrams.per_finger_sfb.get(&finger).unwrap();
        rows[i].push(Cell::new(format!("{sfb:.2}")));

        let sku = stats.bigrams.per_finger_sku.get(&finger).unwrap();
        rows[i].push(Cell::new(format!("{sku:.2}")));
    }

    header.push(ngram_header("symbol stats", 20));
    let unsupported = stats.symbols.total_unsupported;
    rows[0].push(ngram_stat("unsupported", unsupported));

    for (i, hand) in Hand::iter().enumerate() {
        let name = match hand {
            Hand::Left => "left  hand",
            Hand::Right => "right hand",
            Hand::Thumbs => "thumbs",
        };
        let usage = stats.unigrams.hand_usage.get(&hand).unwrap();
        rows[i + 1].push(ngram_stat(name, *usage));
    }
    for i in 4..8 {
        rows[i].push(Cell::new(""));
    }

    header.push(ngram_header("bigram stats", 17));
    rows[0].push(ngram_stat("sku", stats.bigrams.total_sku));
    rows[1].push(ngram_stat("sfb", stats.bigrams.total_sfb));
    rows[2].push(ngram_stat("lsb", stats.bigrams.total_lsb));
    rows[3].push(ngram_stat("scissors", stats.bigrams.total_scissors));
    rows[4].push(ngram_stat("in rolls", stats.bigrams.total_in_rolls));
    rows[5].push(ngram_stat("out rolls", stats.bigrams.total_out_rolls));

    header.push(ngram_header("trigram stats", 20));
    rows[0].push(ngram_stat("sks", stats.trigrams.total_sks));
    rows[1].push(ngram_stat("sfs", stats.trigrams.total_sfs));
    rows[2].push(ngram_stat("redirects", stats.trigrams.total_redirects));
    rows[3].push(ngram_stat(
        "bad redirects",
        stats.trigrams.total_bad_redirects,
    ));

    // TODO: add all rolls and all redirects ?

    let mut table = comfy_table::Table::new();

    table.load_preset(presets::NOTHING).set_header(header);

    for i in 0..8 {
        table.add_row(rows[i].clone());
    }

    // TODO: add detail lists

    println!("{table}");
}

fn ngram_header(name: &str, size: usize) -> Cell {
    Cell::new(format!("{name:>size$}"))
        .set_alignment(Right)
        .add_attribute(Bold)
}

fn ngram_stat(name: &str, val: f32) -> Cell {
    let n = if val >= 10.0 || val < 0.01 { 1 } else { 2 };
    Cell::new(format!("{name}  {val:>4.0$}", n)).set_alignment(Right)
}
