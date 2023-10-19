use criterion::criterion_main;

mod bench_trie;
use crate::bench_trie::bench_trie;

criterion_main!(bench_trie,);
