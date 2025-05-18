pub mod bigrams;
pub mod symbols;
pub mod trigrams;
mod utils;

use crate::geometry::Geometry;
use crate::kalamine::{Corpus, Layout};
use crate::{corpus, keyseq};

pub fn analyse(layout: &Layout, corpus: &Corpus, geometry: Geometry) -> Stats {
    let char_to_keyseq = keyseq::build_keyseq_map(&layout.keymap, &layout.deadkeys);

    let unsupported = symbols::unsupported_characters(&corpus.symbols, &char_to_keyseq);
    dbg!(&unsupported);

    let symbol_freq = corpus::keysym_freq(&corpus.symbols, &char_to_keyseq);
    let finger_freq = symbols::calc_finger_freq(&symbol_freq);
    dbg!(&finger_freq);

    let bigrams_freq = corpus::keysym_ngram_freq(&corpus.digrams, &char_to_keyseq);

    let bigram_stats= bigrams::bigram_stats(&bigrams_freq, geometry);
    dbg!(&bigram_stats.total_sfb);

    let trigrams_freq = corpus::keysym_ngram_freq(&corpus.trigrams, &char_to_keyseq);

    let trigram_stats = trigrams::trigram_stats(&trigrams_freq);

    Stats {
        bigrams: bigram_stats,
        trigrams: trigram_stats,
    }
}


pub struct Stats {
    pub bigrams: bigrams::BigramStats,
    pub trigrams: trigrams::TrigramStats,
}