use criterion::{black_box, criterion_group, Criterion};
use mf_trie::test_util::*;
use mf_trie::EnglishLetter;
use mf_trie::NodeTrie;
use mf_trie::Trie;

fn bench_insert(c: &mut Criterion) {
    let sample_keys = random_strings(1000, 32, *b"seedseedseedseed");
    let sample_vals = random_u32s(1000);
    c.bench_function("insert 1000", |b| {
        b.iter(|| {
            let mut trie: NodeTrie<EnglishLetter> = NodeTrie::new();
            // let mut set = std::collections::HashMap::new();
            for i in 0..1000 {
                let key = &sample_keys[i];
                let val = sample_vals[i];
                trie.add(EnglishLetter::from_str(key).as_slice());
                // set.insert(key, val);
            }
        })
    });
}

fn bench_some_get(c: &mut Criterion) {
    let size = 1000;
    let sample_keys = random_strings(size, 12, *b"seedseedseedseed");
    let sample_vals = random_u32s(size);
    let mut trie: NodeTrie<EnglishLetter> = NodeTrie::new();
    let mut set = std::collections::HashMap::new();
    for i in 0..size {
        let key = &sample_keys[i];
        let val = sample_vals[i];
        trie.add(EnglishLetter::from_str(key).as_slice());
        set.insert(key, val);
    }
    c.bench_function("get existing 1000", |b| {
        b.iter(|| {
            for i in 0..1000 {
                let key = &sample_keys[i];
                let val = sample_vals[i];
                //assert!(set.get(key).is_some());
                assert!(trie.contains(&EnglishLetter::from_str(key).as_slice()));
            }
        })
    });
}

fn bench_none_get(c: &mut Criterion) {
    let sample_keys = random_strings(1000, 16, *b"seedseedseedseed");
    let sample_vals = random_u32s(1000);
    let mut trie: NodeTrie<EnglishLetter> = NodeTrie::new();
    let mut set = std::collections::HashMap::new();
    for i in 0..1000 {
        let key = &sample_keys[i];
        let val = sample_vals[i];
        trie.add(EnglishLetter::from_str(key).as_slice());
        set.insert(key, val);
    }

    let sample_anti_keys = random_strings(1000, 16, *b"antiantiantianti");
    c.bench_function("get non-existing 1000", |b| {
        b.iter(|| {
            for i in 0..1000 {
                let key = &sample_anti_keys[i];
                // let _ = set.get(key);
                let _ = trie.contains(&EnglishLetter::from_str(key).as_slice());
            }
        })
    });
}

criterion_group!(bench_trie, bench_insert, bench_some_get, bench_none_get,);
