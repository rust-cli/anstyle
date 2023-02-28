use anstyle_parse::*;

use std::vec::Vec;

const MAX_PARAMS: usize = 32;
const MAX_OSC_RAW: usize = 1024;
const MAX_OSC_PARAMS: usize = 16;

static OSC_BYTES: &[u8] = &[
    0x1b, 0x5d, // Begin OSC
    b'2', b';', b'j', b'w', b'i', b'l', b'm', b'@', b'j', b'w', b'i', b'l', b'm', b'-', b'd', b'e',
    b's', b'k', b':', b' ', b'~', b'/', b'c', b'o', b'd', b'e', b'/', b'a', b'l', b'a', b'c', b'r',
    b'i', b't', b't', b'y', 0x07, // End OSC
];

#[derive(Default)]
struct Dispatcher {
    dispatched: Vec<Sequence>,
}

#[derive(Debug, PartialEq, Eq)]
enum Sequence {
    Osc(Vec<Vec<u8>>, bool),
    Csi(Vec<Vec<u16>>, Vec<u8>, bool, char),
    Esc(Vec<u8>, bool, u8),
    DcsHook(Vec<Vec<u16>>, Vec<u8>, bool, char),
    DcsPut(u8),
    DcsUnhook,
}

impl Perform for Dispatcher {
    fn osc_dispatch(&mut self, params: &[&[u8]], bell_terminated: bool) {
        let params = params.iter().map(|p| p.to_vec()).collect();
        self.dispatched.push(Sequence::Osc(params, bell_terminated));
    }

    fn csi_dispatch(&mut self, params: &Params, intermediates: &[u8], ignore: bool, c: char) {
        let params = params.iter().map(|subparam| subparam.to_vec()).collect();
        let intermediates = intermediates.to_vec();
        self.dispatched
            .push(Sequence::Csi(params, intermediates, ignore, c));
    }

    fn esc_dispatch(&mut self, intermediates: &[u8], ignore: bool, byte: u8) {
        let intermediates = intermediates.to_vec();
        self.dispatched
            .push(Sequence::Esc(intermediates, ignore, byte));
    }

    fn hook(&mut self, params: &Params, intermediates: &[u8], ignore: bool, c: char) {
        let params = params.iter().map(|subparam| subparam.to_vec()).collect();
        let intermediates = intermediates.to_vec();
        self.dispatched
            .push(Sequence::DcsHook(params, intermediates, ignore, c));
    }

    fn put(&mut self, byte: u8) {
        self.dispatched.push(Sequence::DcsPut(byte));
    }

    fn unhook(&mut self) {
        self.dispatched.push(Sequence::DcsUnhook);
    }
}

#[test]
fn parse_osc() {
    let mut dispatcher = Dispatcher::default();
    let mut parser = Parser::new();

    for byte in OSC_BYTES {
        parser.advance(&mut dispatcher, *byte);
    }

    assert_eq!(dispatcher.dispatched.len(), 1);
    match &dispatcher.dispatched[0] {
        Sequence::Osc(params, _) => {
            assert_eq!(params.len(), 2);
            assert_eq!(params[0], &OSC_BYTES[2..3]);
            assert_eq!(params[1], &OSC_BYTES[4..(OSC_BYTES.len() - 1)]);
        }
        _ => panic!("expected osc sequence"),
    }
}

#[test]
fn parse_empty_osc() {
    let mut dispatcher = Dispatcher::default();
    let mut parser = Parser::new();

    for byte in &[0x1b, 0x5d, 0x07] {
        parser.advance(&mut dispatcher, *byte);
    }

    assert_eq!(dispatcher.dispatched.len(), 1);
    match &dispatcher.dispatched[0] {
        Sequence::Osc(..) => (),
        _ => panic!("expected osc sequence"),
    }
}

#[test]
fn parse_osc_max_params() {
    let params = ";".repeat(MAX_PARAMS + 1);
    let input = format!("\x1b]{}\x1b", &params[..]).into_bytes();
    let mut dispatcher = Dispatcher::default();
    let mut parser = Parser::new();

    for byte in input {
        parser.advance(&mut dispatcher, byte);
    }

    assert_eq!(dispatcher.dispatched.len(), 1);
    match &dispatcher.dispatched[0] {
        Sequence::Osc(params, _) => {
            assert_eq!(params.len(), MAX_OSC_PARAMS);
            assert!(params.iter().all(Vec::is_empty));
        }
        _ => panic!("expected osc sequence"),
    }
}

#[test]
fn osc_bell_terminated() {
    static INPUT: &[u8] = b"\x1b]11;ff/00/ff\x07";
    let mut dispatcher = Dispatcher::default();
    let mut parser = Parser::new();

    for byte in INPUT {
        parser.advance(&mut dispatcher, *byte);
    }

    assert_eq!(dispatcher.dispatched.len(), 1);
    match &dispatcher.dispatched[0] {
        Sequence::Osc(_, true) => (),
        _ => panic!("expected osc with bell terminator"),
    }
}

#[test]
fn osc_c0_st_terminated() {
    static INPUT: &[u8] = b"\x1b]11;ff/00/ff\x1b\\";
    let mut dispatcher = Dispatcher::default();
    let mut parser = Parser::new();

    for byte in INPUT {
        parser.advance(&mut dispatcher, *byte);
    }

    assert_eq!(dispatcher.dispatched.len(), 2);
    match &dispatcher.dispatched[0] {
        Sequence::Osc(_, false) => (),
        _ => panic!("expected osc with ST terminator"),
    }
}

#[test]
fn parse_osc_with_utf8_arguments() {
    static INPUT: &[u8] = &[
        0x0d, 0x1b, 0x5d, 0x32, 0x3b, 0x65, 0x63, 0x68, 0x6f, 0x20, 0x27, 0xc2, 0xaf, 0x5c, 0x5f,
        0x28, 0xe3, 0x83, 0x84, 0x29, 0x5f, 0x2f, 0xc2, 0xaf, 0x27, 0x20, 0x26, 0x26, 0x20, 0x73,
        0x6c, 0x65, 0x65, 0x70, 0x20, 0x31, 0x07,
    ];
    let mut dispatcher = Dispatcher::default();
    let mut parser = Parser::new();

    for byte in INPUT {
        parser.advance(&mut dispatcher, *byte);
    }

    assert_eq!(dispatcher.dispatched.len(), 1);
    match &dispatcher.dispatched[0] {
        Sequence::Osc(params, _) => {
            assert_eq!(params[0], &[b'2']);
            assert_eq!(params[1], &INPUT[5..(INPUT.len() - 1)]);
        }
        _ => panic!("expected osc sequence"),
    }
}

#[test]
fn osc_containing_string_terminator() {
    static INPUT: &[u8] = b"\x1b]2;\xe6\x9c\xab\x1b\\";
    let mut dispatcher = Dispatcher::default();
    let mut parser = Parser::new();

    for byte in INPUT {
        parser.advance(&mut dispatcher, *byte);
    }

    assert_eq!(dispatcher.dispatched.len(), 2);
    match &dispatcher.dispatched[0] {
        Sequence::Osc(params, _) => {
            assert_eq!(params[1], &INPUT[4..(INPUT.len() - 2)]);
        }
        _ => panic!("expected osc sequence"),
    }
}

#[test]
fn exceed_max_buffer_size() {
    static NUM_BYTES: usize = MAX_OSC_RAW + 100;
    static INPUT_START: &[u8] = &[0x1b, b']', b'5', b'2', b';', b's'];
    static INPUT_END: &[u8] = &[b'\x07'];

    let mut dispatcher = Dispatcher::default();
    let mut parser = Parser::new();

    // Create valid OSC escape
    for byte in INPUT_START {
        parser.advance(&mut dispatcher, *byte);
    }

    // Exceed max buffer size
    for _ in 0..NUM_BYTES {
        parser.advance(&mut dispatcher, b'a');
    }

    // Terminate escape for dispatch
    for byte in INPUT_END {
        parser.advance(&mut dispatcher, *byte);
    }

    assert_eq!(dispatcher.dispatched.len(), 1);
    match &dispatcher.dispatched[0] {
        Sequence::Osc(params, _) => {
            assert_eq!(params.len(), 2);
            assert_eq!(params[0], b"52");

            #[cfg(not(feature = "core"))]
            assert_eq!(params[1].len(), NUM_BYTES + INPUT_END.len());

            #[cfg(feature = "core")]
            assert_eq!(params[1].len(), MAX_OSC_RAW - params[0].len());
        }
        _ => panic!("expected osc sequence"),
    }
}

#[test]
fn parse_csi_max_params() {
    // This will build a list of repeating '1;'s
    // The length is MAX_PARAMS - 1 because the last semicolon is interpreted
    // as an implicit zero, making the total number of parameters MAX_PARAMS
    let params = "1;".repeat(MAX_PARAMS - 1);
    let input = format!("\x1b[{}p", &params[..]).into_bytes();

    let mut dispatcher = Dispatcher::default();
    let mut parser = Parser::new();

    for byte in input {
        parser.advance(&mut dispatcher, byte);
    }

    assert_eq!(dispatcher.dispatched.len(), 1);
    match &dispatcher.dispatched[0] {
        Sequence::Csi(params, _, ignore, _) => {
            assert_eq!(params.len(), MAX_PARAMS);
            assert!(!ignore);
        }
        _ => panic!("expected csi sequence"),
    }
}

#[test]
fn parse_csi_params_ignore_long_params() {
    // This will build a list of repeating '1;'s
    // The length is MAX_PARAMS because the last semicolon is interpreted
    // as an implicit zero, making the total number of parameters MAX_PARAMS + 1
    let params = "1;".repeat(MAX_PARAMS);
    let input = format!("\x1b[{}p", &params[..]).into_bytes();

    let mut dispatcher = Dispatcher::default();
    let mut parser = Parser::new();

    for byte in input {
        parser.advance(&mut dispatcher, byte);
    }

    assert_eq!(dispatcher.dispatched.len(), 1);
    match &dispatcher.dispatched[0] {
        Sequence::Csi(params, _, ignore, _) => {
            assert_eq!(params.len(), MAX_PARAMS);
            assert!(ignore);
        }
        _ => panic!("expected csi sequence"),
    }
}

#[test]
fn parse_csi_params_trailing_semicolon() {
    let mut dispatcher = Dispatcher::default();
    let mut parser = Parser::new();

    for byte in b"\x1b[4;m" {
        parser.advance(&mut dispatcher, *byte);
    }

    assert_eq!(dispatcher.dispatched.len(), 1);
    match &dispatcher.dispatched[0] {
        Sequence::Csi(params, ..) => assert_eq!(params, &[[4], [0]]),
        _ => panic!("expected csi sequence"),
    }
}

#[test]
fn parse_csi_params_leading_semicolon() {
    // Create dispatcher and check state
    let mut dispatcher = Dispatcher::default();
    let mut parser = Parser::new();

    for byte in b"\x1b[;4m" {
        parser.advance(&mut dispatcher, *byte);
    }

    assert_eq!(dispatcher.dispatched.len(), 1);
    match &dispatcher.dispatched[0] {
        Sequence::Csi(params, ..) => assert_eq!(params, &[[0], [4]]),
        _ => panic!("expected csi sequence"),
    }
}

#[test]
fn parse_long_csi_param() {
    // The important part is the parameter, which is (i64::MAX + 1)
    static INPUT: &[u8] = b"\x1b[9223372036854775808m";
    let mut dispatcher = Dispatcher::default();
    let mut parser = Parser::new();

    for byte in INPUT {
        parser.advance(&mut dispatcher, *byte);
    }

    assert_eq!(dispatcher.dispatched.len(), 1);
    match &dispatcher.dispatched[0] {
        Sequence::Csi(params, ..) => assert_eq!(params, &[[std::u16::MAX as u16]]),
        _ => panic!("expected csi sequence"),
    }
}

#[test]
fn csi_reset() {
    static INPUT: &[u8] = b"\x1b[3;1\x1b[?1049h";
    let mut dispatcher = Dispatcher::default();
    let mut parser = Parser::new();

    for byte in INPUT {
        parser.advance(&mut dispatcher, *byte);
    }

    assert_eq!(dispatcher.dispatched.len(), 1);
    match &dispatcher.dispatched[0] {
        Sequence::Csi(params, intermediates, ignore, _) => {
            assert_eq!(intermediates, &[b'?']);
            assert_eq!(params, &[[1049]]);
            assert!(!ignore);
        }
        _ => panic!("expected csi sequence"),
    }
}

#[test]
fn csi_subparameters() {
    static INPUT: &[u8] = b"\x1b[38:2:255:0:255;1m";
    let mut dispatcher = Dispatcher::default();
    let mut parser = Parser::new();

    for byte in INPUT {
        parser.advance(&mut dispatcher, *byte);
    }

    assert_eq!(dispatcher.dispatched.len(), 1);
    match &dispatcher.dispatched[0] {
        Sequence::Csi(params, intermediates, ignore, _) => {
            assert_eq!(params, &[vec![38, 2, 255, 0, 255], vec![1]]);
            assert_eq!(intermediates, &[]);
            assert!(!ignore);
        }
        _ => panic!("expected csi sequence"),
    }
}

#[test]
fn parse_dcs_max_params() {
    let params = "1;".repeat(MAX_PARAMS + 1);
    let input = format!("\x1bP{}p", &params[..]).into_bytes();
    let mut dispatcher = Dispatcher::default();
    let mut parser = Parser::new();

    for byte in input {
        parser.advance(&mut dispatcher, byte);
    }

    assert_eq!(dispatcher.dispatched.len(), 1);
    match &dispatcher.dispatched[0] {
        Sequence::DcsHook(params, _, ignore, _) => {
            assert_eq!(params.len(), MAX_PARAMS);
            assert!(params.iter().all(|param| param == &[1]));
            assert!(ignore);
        }
        _ => panic!("expected dcs sequence"),
    }
}

#[test]
fn dcs_reset() {
    static INPUT: &[u8] = b"\x1b[3;1\x1bP1$tx\x9c";
    let mut dispatcher = Dispatcher::default();
    let mut parser = Parser::new();

    for byte in INPUT {
        parser.advance(&mut dispatcher, *byte);
    }

    assert_eq!(dispatcher.dispatched.len(), 3);
    match &dispatcher.dispatched[0] {
        Sequence::DcsHook(params, intermediates, ignore, _) => {
            assert_eq!(intermediates, &[b'$']);
            assert_eq!(params, &[[1]]);
            assert!(!ignore);
        }
        _ => panic!("expected dcs sequence"),
    }
    assert_eq!(dispatcher.dispatched[1], Sequence::DcsPut(b'x'));
    assert_eq!(dispatcher.dispatched[2], Sequence::DcsUnhook);
}

#[test]
fn parse_dcs() {
    static INPUT: &[u8] = b"\x1bP0;1|17/ab\x9c";
    let mut dispatcher = Dispatcher::default();
    let mut parser = Parser::new();

    for byte in INPUT {
        parser.advance(&mut dispatcher, *byte);
    }

    assert_eq!(dispatcher.dispatched.len(), 7);
    match &dispatcher.dispatched[0] {
        Sequence::DcsHook(params, _, _, c) => {
            assert_eq!(params, &[[0], [1]]);
            assert_eq!(c, &'|');
        }
        _ => panic!("expected dcs sequence"),
    }
    for (i, byte) in b"17/ab".iter().enumerate() {
        assert_eq!(dispatcher.dispatched[1 + i], Sequence::DcsPut(*byte));
    }
    assert_eq!(dispatcher.dispatched[6], Sequence::DcsUnhook);
}

#[test]
fn intermediate_reset_on_dcs_exit() {
    static INPUT: &[u8] = b"\x1bP=1sZZZ\x1b+\x5c";
    let mut dispatcher = Dispatcher::default();
    let mut parser = Parser::new();

    for byte in INPUT {
        parser.advance(&mut dispatcher, *byte);
    }

    assert_eq!(dispatcher.dispatched.len(), 6);
    match &dispatcher.dispatched[5] {
        Sequence::Esc(intermediates, ..) => assert_eq!(intermediates, &[b'+']),
        _ => panic!("expected esc sequence"),
    }
}

#[test]
fn esc_reset() {
    static INPUT: &[u8] = b"\x1b[3;1\x1b(A";
    let mut dispatcher = Dispatcher::default();
    let mut parser = Parser::new();

    for byte in INPUT {
        parser.advance(&mut dispatcher, *byte);
    }

    assert_eq!(dispatcher.dispatched.len(), 1);
    match &dispatcher.dispatched[0] {
        Sequence::Esc(intermediates, ignore, byte) => {
            assert_eq!(intermediates, &[b'(']);
            assert_eq!(*byte, b'A');
            assert!(!ignore);
        }
        _ => panic!("expected esc sequence"),
    }
}

#[test]
fn params_buffer_filled_with_subparam() {
    static INPUT: &[u8] = b"\x1b[::::::::::::::::::::::::::::::::x\x1b";
    let mut dispatcher = Dispatcher::default();
    let mut parser = Parser::new();

    for byte in INPUT {
        parser.advance(&mut dispatcher, *byte);
    }

    assert_eq!(dispatcher.dispatched.len(), 1);
    match &dispatcher.dispatched[0] {
        Sequence::Csi(params, intermediates, ignore, c) => {
            assert_eq!(intermediates, &[]);
            assert_eq!(params, &[[0; 32]]);
            assert_eq!(c, &'x');
            assert!(ignore);
        }
        _ => panic!("expected csi sequence"),
    }
}
