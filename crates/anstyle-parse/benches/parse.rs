#![cfg_attr(all(feature = "nightly", test), feature(test))]

#[cfg(all(feature = "nightly", test))]
mod bench {
    extern crate std;
    extern crate test;

    use anstyle_parse::*;

    use test::{black_box, Bencher};

    static VTE_DEMO: &[u8] = include_bytes!("../tests/demo.vte");

    struct BenchDispatcher;
    impl Perform for BenchDispatcher {
        fn print(&mut self, c: char) {
            black_box(c);
        }

        fn execute(&mut self, byte: u8) {
            black_box(byte);
        }

        fn hook(&mut self, params: &Params, intermediates: &[u8], ignore: bool, c: char) {
            black_box((params, intermediates, ignore, c));
        }

        fn put(&mut self, byte: u8) {
            black_box(byte);
        }

        fn osc_dispatch(&mut self, params: &[&[u8]], bell_terminated: bool) {
            black_box((params, bell_terminated));
        }

        fn csi_dispatch(&mut self, params: &Params, intermediates: &[u8], ignore: bool, c: char) {
            black_box((params, intermediates, ignore, c));
        }

        fn esc_dispatch(&mut self, intermediates: &[u8], ignore: bool, byte: u8) {
            black_box((intermediates, ignore, byte));
        }
    }

    #[bench]
    fn testfile(b: &mut Bencher) {
        b.iter(|| {
            let mut dispatcher = BenchDispatcher;
            let mut parser = Parser::new();

            for byte in VTE_DEMO {
                parser.advance(&mut dispatcher, *byte);
            }
        });
    }

    #[bench]
    fn state_changes(b: &mut Bencher) {
        let input = b"\x1b]2;X\x1b\\ \x1b[0m \x1bP0@\x1b\\";
        b.iter(|| {
            let mut dispatcher = BenchDispatcher;
            let mut parser = Parser::new();

            for _ in 0..1_000 {
                for byte in input {
                    parser.advance(&mut dispatcher, *byte);
                }
            }
        });
    }
}
