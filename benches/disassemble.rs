use std::fs;

use criterion::{criterion_group, criterion_main, Criterion};

use mos_6502_disassembler::disassemble;

fn bench_disassemble(c: &mut Criterion) {
    c.bench_function("bench_disassemble_mega_bin", |b| {
        let input = fs::read("test-bin/mega.bin").unwrap();
        b.iter(|| {
            std::hint::black_box(for _ in 1..=100 {
                disassemble(&input);
            });
        });
    });

    c.bench_function("bench_disassemble_giga_bin", |b| {
        let input = fs::read("test-bin/giga.bin").unwrap();
        b.iter(|| {
            std::hint::black_box(for _ in 1..=10 {
                disassemble(&input);
            });
        });
    });
}

criterion_group!(benches, bench_disassemble);
criterion_main!(benches);
