pub mod bigrams;
pub mod symbols;
pub mod trigrams;
mod utils;

use crate::geometry::Geometry;
use crate::kalamine::{Corpus, Layout};
use crate::{corpus, keystrokes};

pub fn analyse(layout: &Layout, corpus: &Corpus, geometry: Geometry) {
    let char_to_keyseq = keystrokes::build_keyseq_map(&layout.keymap, &layout.deadkeys);

    let unsupported = symbols::unsupported_characters(&corpus.symbols, &char_to_keyseq);
    dbg!(&unsupported);

    // TODO: repair this
    // let stats = symbols::calc_finger_freq(&char_to_keyseq, &corpus.symbols);
    // dbg!(&stats);

    let bigrams_freq = corpus::keysym_ngram_freq(&corpus.digrams, &char_to_keyseq);

    let (sfb, sku) = bigrams::bigram_stats(&bigrams_freq, geometry);
    let mut sfb = utils::map_to_vec(sfb);
    utils::sort_vec_by_value(&mut sfb);
    sfb.reverse();
    dbg!(&sfb);
    dbg!(&sku);

    let trigrams_freq = corpus::keysym_ngram_freq(&corpus.trigrams, &char_to_keyseq);

    trigrams::trigram_stats(&trigrams_freq);
}
