pub mod bigrams;
pub mod symbols;
pub mod utils;

use crate::kalamine::{Corpus, Layout};
use crate::keystrokes;

pub fn analyse(layout: &Layout, corpus: &Corpus) {
    let sym_to_keystrokes = keystrokes::build_keystrokes_map(layout);

    let stats = symbols::calc_finger_freq(&sym_to_keystrokes, &corpus.symbols);
    dbg!(&stats);

    let (sfb, sku) = bigrams::calc_bigrams(&sym_to_keystrokes, &corpus.digrams);
    let mut sfb = utils::map_to_vec(sfb);
    utils::sort_vec_by_value(&mut sfb);
    sfb.reverse();
    dbg!(&sfb);
    dbg!(&sku);
}
