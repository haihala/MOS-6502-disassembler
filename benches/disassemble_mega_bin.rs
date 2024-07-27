use std::fs;

use criterion::{criterion_group, criterion_main, Criterion};

use mos_6502_disassembler::disassemble;

fn bench_disassemble_mega_bin(c: &mut Criterion) {
    c.bench_function("bench_disassemble", |b| {
        let input = fs::read("test-bin/mega.bin").unwrap();
        b.iter(|| {
            std::hint::black_box(for _ in 1..=100 {
                let _ = disassemble(&input);
            });
        });
    });
}

criterion_group!(benches, bench_disassemble_mega_bin);
criterion_main!(benches);
