pub mod bigrams;
pub mod symbols;
pub mod trigrams;
pub mod unigrams;
mod utils;

use crate::geometry::Geometry;
use crate::kalamine::{Corpus, Layout};
use crate::{corpus, keyseq};

pub fn analyse(layout: &Layout, corpus: &Corpus, geometry: Geometry) -> Stats {
    let char_to_keyseq = keyseq::build_keyseq_map(&layout.keymap, &layout.deadkeys);

    let symbol_stats = symbols::symbol_stats(&corpus.symbols, &char_to_keyseq);

    let symbol_freq = corpus::keysym_freq(&corpus.symbols, &char_to_keyseq);
    let unigram_stats = unigrams::unigram_stats(&symbol_freq);

    let bigrams_freq = corpus::keysym_ngram_freq(&corpus.digrams, &char_to_keyseq);
    let bigram_stats = bigrams::bigram_stats(&bigrams_freq, geometry);

    let trigrams_freq = corpus::keysym_ngram_freq(&corpus.trigrams, &char_to_keyseq);
    let trigram_stats = trigrams::trigram_stats(&trigrams_freq);

    Stats {
        symbols: symbol_stats,
        unigrams: unigram_stats,
        bigrams: bigram_stats,
        trigrams: trigram_stats,
    }
}

pub struct Stats {
    pub symbols: symbols::SymbolStats,
    pub unigrams: unigrams::UnigramStats,
    pub bigrams: bigrams::BigramStats,
    pub trigrams: trigrams::TrigramStats,
}
