use criterion::{black_box, Criterion};

use anstyle_parse::*;

struct BenchDispatcher;
impl Perform<char> for BenchDispatcher {
    fn print(&mut self, c: char) {
        black_box(c);
    }

    fn execute(&mut self, byte: u8) {
        black_box(byte);
    }

    fn hook(&mut self, params: &Params, intermediates: &[u8], ignore: bool, c: u8) {
        black_box((params, intermediates, ignore, c));
    }

    fn put(&mut self, byte: u8) {
        black_box(byte);
    }

    fn osc_dispatch(&mut self, params: &[&[u8]], bell_terminated: bool) {
        black_box((params, bell_terminated));
    }

    fn csi_dispatch(&mut self, params: &Params, intermediates: &[u8], ignore: bool, c: u8) {
        black_box((params, intermediates, ignore, c));
    }

    fn esc_dispatch(&mut self, intermediates: &[u8], ignore: bool, byte: u8) {
        black_box((intermediates, ignore, byte));
    }
}
impl Perform<&'_ str> for BenchDispatcher {
    fn print(&mut self, c: &'_ str) {
        black_box(c);
    }

    fn execute(&mut self, byte: u8) {
        black_box(byte);
    }

    fn hook(&mut self, params: &Params, intermediates: &[u8], ignore: bool, c: u8) {
        black_box((params, intermediates, ignore, c));
    }

    fn put(&mut self, byte: u8) {
        black_box(byte);
    }

    fn osc_dispatch(&mut self, params: &[&[u8]], bell_terminated: bool) {
        black_box((params, bell_terminated));
    }

    fn csi_dispatch(&mut self, params: &Params, intermediates: &[u8], ignore: bool, c: u8) {
        black_box((params, intermediates, ignore, c));
    }

    fn esc_dispatch(&mut self, intermediates: &[u8], ignore: bool, byte: u8) {
        black_box((intermediates, ignore, byte));
    }
}

#[derive(Default)]
struct Strip(String);
impl Strip {
    fn with_capacity(capacity: usize) -> Self {
        Self(String::with_capacity(capacity))
    }
}
impl Perform<char> for Strip {
    fn print_control(byte: u8) -> bool {
        byte.is_ascii_whitespace()
    }
    fn print(&mut self, c: char) {
        self.0.push(c);
    }
}
impl Perform<&'_ str> for Strip {
    fn print_control(byte: u8) -> bool {
        byte.is_ascii_whitespace()
    }
    fn print(&mut self, c: &'_ str) {
        self.0.push_str(c);
    }
}

fn parse(c: &mut Criterion) {
    for (name, content) in [
        #[cfg(feature = "utf8")]
        ("demo.vte", &include_bytes!("../tests/demo.vte")[..]),
        ("rg_help.vte", &include_bytes!("../tests/rg_help.vte")[..]),
        ("rg_linus.vte", &include_bytes!("../tests/rg_linus.vte")[..]),
        (
            "state_changes",
            &b"\x1b]2;X\x1b\\ \x1b[0m \x1bP0@\x1b\\"[..],
        ),
    ] {
        let mut group = c.benchmark_group(name);
        group.bench_function("advance", |b| {
            b.iter(|| {
                let mut dispatcher = BenchDispatcher;
                let mut parser = Parser::<DefaultCharAccumulator>::new();

                for byte in content {
                    parser.advance(&mut dispatcher, *byte);
                }
            })
        });
        if let Ok(content) = std::str::from_utf8(content) {
            group.bench_function("advance_str", |b| {
                b.iter(|| {
                    let mut dispatcher = BenchDispatcher;
                    let mut parser = Parser::<DefaultCharAccumulator>::new();

                    parser.advance_str(&mut dispatcher, content);
                })
            });
        }
        group.bench_function("advance_strip", |b| {
            b.iter(|| {
                let mut stripped = Strip::with_capacity(content.len());
                let mut parser = Parser::<DefaultCharAccumulator>::new();

                for byte in content {
                    parser.advance(&mut stripped, *byte);
                }

                black_box(stripped.0)
            })
        });
        if let Ok(content) = std::str::from_utf8(content) {
            group.bench_function("advance_str_strip", |b| {
                b.iter(|| {
                    let mut stripped = Strip::with_capacity(content.len());
                    let mut parser = Parser::<DefaultCharAccumulator>::new();

                    parser.advance_str(&mut stripped, content);

                    black_box(stripped.0)
                })
            });
        }
    }
}

criterion::criterion_group!(benches, parse);
criterion::criterion_main!(benches);
