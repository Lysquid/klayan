use comfy_table::{self, presets, Attribute::Bold, Cell, CellAlignment::Right};
use klayan::{
    hands::{Finger, Hand},
    kalamine::Symbol,
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

    let mut table1 = comfy_table::Table::new();

    table1.load_preset(presets::NOTHING).set_header(header);

    for i in 0..8 {
        table1.add_row(rows[i].clone());
    }

    let list_len = max_list_len(&stats).min(5);

    let mut header: Vec<Cell> = Vec::from([
        Cell::new("sku").add_attribute(Bold),
        Cell::new("sfb").add_attribute(Bold),
        Cell::new("lsb").add_attribute(Bold),
        Cell::new("scissor").add_attribute(Bold),
        Cell::new("in roll").add_attribute(Bold),
        Cell::new("out rol").add_attribute(Bold),
        Cell::new("sks").add_attribute(Bold),
        Cell::new("sfs").add_attribute(Bold),
        Cell::new("redirect").add_attribute(Bold),
        Cell::new("bad redi").add_attribute(Bold),
    ]);
    if !stats.symbols.list_unsupported.is_empty() {
        header.push(Cell::new("unspted").add_attribute(Bold))
    }
    let mut rows: Vec<[Cell; 11]> = vec![core::array::from_fn(|_| Cell::new("")); list_len];

    let list_unsupported: Vec<([Symbol; 1], f32)> = stats
        .symbols
        .list_unsupported
        .iter()
        .map(|(c, f)| ([Symbol::Character(*c); 1], *f))
        .collect();

    list(&mut rows, 0, list_len, stats.bigrams.list_sku);
    list(&mut rows, 1, list_len, stats.bigrams.list_sfb);
    list(&mut rows, 2, list_len, stats.bigrams.list_lsb);
    list(&mut rows, 3, list_len, stats.bigrams.list_scissors);
    list(&mut rows, 4, list_len, stats.bigrams.list_in_rolls);
    list(&mut rows, 5, list_len, stats.bigrams.list_out_rolls);
    list(&mut rows, 6, list_len, stats.trigrams.list_sks);
    list(&mut rows, 7, list_len, stats.trigrams.list_sfs);
    list(&mut rows, 8, list_len, stats.trigrams.list_redirects);
    list(&mut rows, 9, list_len, stats.trigrams.list_bad_redirects);
    list(&mut rows, 10, list_len, list_unsupported);

    let mut table2 = comfy_table::Table::new();

    table2.load_preset(presets::NOTHING).set_header(header);

    for i in 0..list_len {
        table2.add_row(rows[i].clone());
    }

    println!("{table1}");
    println!();
    println!("{table2}");
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

fn list<const N: usize>(
    rows: &mut Vec<[Cell; 11]>,
    col: usize,
    max_len: usize,
    list: Vec<([Symbol; N], f32)>,
) {
    for (i, (symbols, freq)) in list.iter().take(max_len).enumerate() {
        let cell = Cell::new(format!("{} {freq:4.2}", symbols_to_string(symbols)));
        rows.get_mut(i)
            .unwrap()
            .get_mut(col)
            .unwrap()
            .clone_from(&cell);
    }
}

fn symbols_to_string<const N: usize>(ngram: &[Symbol; N]) -> String {
    ngram
        .iter()
        .map(|c| match c {
            Symbol::Character(c) => c,
            Symbol::DeadKey(c) => c,
        })
        .collect()
}

pub fn max_list_len(stats: &Stats) -> usize {
    stats
        .symbols
        .list_unsupported
        .len()
        .max(stats.bigrams.list_sku.len())
        .max(stats.bigrams.list_sfb.len())
        .max(stats.bigrams.list_lsb.len())
        .max(stats.bigrams.list_scissors.len())
        .max(stats.bigrams.list_in_rolls.len())
        .max(stats.bigrams.list_out_rolls.len())
        .max(stats.trigrams.list_sks.len())
        .max(stats.trigrams.list_sfs.len())
        .max(stats.trigrams.list_redirects.len())
        .max(stats.trigrams.list_bad_redirects.len())
}
