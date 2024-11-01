use std::vec::Vec;

use proptest::prelude::*;

use anstyle_parse::*;

const MAX_PARAMS: usize = 32;
const MAX_OSC_RAW: usize = 1024;
const MAX_OSC_PARAMS: usize = 16;

static OSC_BYTES: &[u8] = &[
    0x1b, 0x5d, // Begin OSC
    b'2', b';', b'j', b'w', b'i', b'l', b'm', b'@', b'j', b'w', b'i', b'l', b'm', b'-', b'd', b'e',
    b's', b'k', b':', b' ', b'~', b'/', b'c', b'o', b'd', b'e', b'/', b'a', b'l', b'a', b'c', b'r',
    b'i', b't', b't', b'y', 0x07, // End OSC
];

fn start() -> Dispatcher {
    Dispatcher::default()
}

#[derive(Default, PartialEq, Eq, Debug)]
struct Dispatcher {
    dispatched: Vec<Sequence>,
}

impl Perform for Dispatcher {
    fn print(&mut self, c: char) {
        self.dispatched.push(Sequence::Print(c));
    }

    fn osc_dispatch(&mut self, params: &[&[u8]], bell_terminated: bool) {
        let params = params.iter().map(|p| p.to_vec()).collect();
        self.dispatched.push(Sequence::Osc(params, bell_terminated));
    }

    fn csi_dispatch(&mut self, params: &Params, intermediates: &[u8], ignore: bool, c: u8) {
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

    fn hook(&mut self, params: &Params, intermediates: &[u8], ignore: bool, c: u8) {
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

impl std::ops::Deref for Dispatcher {
    type Target = [Sequence];

    fn deref(&self) -> &Self::Target {
        &self.dispatched
    }
}

impl<D> std::ops::Add<D> for Dispatcher
where
    D: Into<Dispatcher>,
{
    type Output = Self;
    fn add(mut self, rhs: D) -> Self::Output {
        self.dispatched.extend(rhs.into().dispatched);
        self
    }
}

impl From<Vec<Sequence>> for Dispatcher {
    fn from(seq: Vec<Sequence>) -> Self {
        Dispatcher { dispatched: seq }
    }
}

impl From<&'_ str> for Dispatcher {
    fn from(printable: &'_ str) -> Self {
        printable
            .chars()
            .map(Sequence::from)
            .collect::<Vec<_>>()
            .into()
    }
}

impl From<&'_ [u8]> for Dispatcher {
    fn from(printable: &'_ [u8]) -> Self {
        String::from_utf8_lossy(printable)
            .chars()
            .map(Sequence::from)
            .collect::<Vec<_>>()
            .into()
    }
}

impl From<Sequence> for Dispatcher {
    fn from(seq: Sequence) -> Self {
        vec![seq].into()
    }
}

impl From<char> for Dispatcher {
    fn from(printable: char) -> Self {
        Sequence::from(printable).into()
    }
}

#[derive(PartialEq, Eq, Debug)]
enum Sequence {
    Print(char),
    Osc(Vec<Vec<u8>>, bool),
    Csi(Vec<Vec<u16>>, Vec<u8>, bool, u8),
    Esc(Vec<u8>, bool, u8),
    DcsHook(Vec<Vec<u16>>, Vec<u8>, bool, u8),
    DcsPut(u8),
    DcsUnhook,
}

impl From<char> for Sequence {
    fn from(printable: char) -> Self {
        Self::Print(printable)
    }
}

macro_rules! advance {
    ($name: ident, $gen: ident) => {
        #[test]
        fn $name() {
            let (input, expected) = $gen();
            let mut dispatcher = Dispatcher::default();
            let mut parser = Parser::<DefaultCharAccumulator>::new();

            for byte in &input {
                parser.advance(&mut dispatcher, *byte);
            }

            assert_eq!(expected, dispatcher);
        }
    };
}

fn gen_osc() -> (Vec<u8>, Dispatcher) {
    let input = OSC_BYTES.to_vec();
    let expected = start()
        + Sequence::Osc(
            vec![
                OSC_BYTES[2..3].to_vec(),
                OSC_BYTES[4..(OSC_BYTES.len() - 1)].to_vec(),
            ],
            true,
        );
    (input, expected)
}

advance!(advance_osc, gen_osc);

fn gen_empty_osc() -> (Vec<u8>, Dispatcher) {
    let input = [0x1b, 0x5d, 0x07].into();
    let expected = start() + Sequence::Osc(vec![vec![]], true);
    (input, expected)
}

advance!(advance_empty_osc, gen_empty_osc);

fn gen_osc_max_params() -> (Vec<u8>, Dispatcher) {
    let params = ";".repeat(MAX_PARAMS + 1);
    let input = format!("\x1b]{}\x1b", &params[..]).into_bytes();
    let expected = start() + Sequence::Osc(vec![vec![]; MAX_OSC_PARAMS], false);
    (input, expected)
}

advance!(advance_osc_max_params, gen_osc_max_params);

fn gen_osc_bell_terminated() -> (Vec<u8>, Dispatcher) {
    let input = b"\x1b]11;ff/00/ff\x07".to_vec();
    let expected = start()
        + Sequence::Osc(
            vec![input[2..4].to_vec(), input[5..(input.len() - 1)].to_vec()],
            true,
        );
    (input, expected)
}

advance!(advance_osc_bell_terminated, gen_osc_bell_terminated);

fn gen_osc_c0_st_terminated() -> (Vec<u8>, Dispatcher) {
    let input = b"\x1b]11;ff/00/ff\x1b\\".to_vec();
    let expected = start()
        + Sequence::Osc(
            vec![input[2..4].to_vec(), input[5..(input.len() - 2)].to_vec()],
            false,
        )
        + Sequence::Esc(vec![], false, 92);
    (input, expected)
}

advance!(advance_osc_c0_st_terminated, gen_osc_c0_st_terminated);

fn gen_osc_with_utf8_arguments() -> (Vec<u8>, Dispatcher) {
    let input = [
        0x0d, 0x1b, 0x5d, 0x32, 0x3b, 0x65, 0x63, 0x68, 0x6f, 0x20, 0x27, 0xc2, 0xaf, 0x5c, 0x5f,
        0x28, 0xe3, 0x83, 0x84, 0x29, 0x5f, 0x2f, 0xc2, 0xaf, 0x27, 0x20, 0x26, 0x26, 0x20, 0x73,
        0x6c, 0x65, 0x65, 0x70, 0x20, 0x31, 0x07,
    ]
    .to_vec();
    let expected =
        start() + Sequence::Osc(vec![vec![b'2'], input[5..(input.len() - 1)].to_vec()], true);
    (input, expected)
}

advance!(advance_osc_with_utf8_arguments, gen_osc_with_utf8_arguments);

fn gen_osc_containing_string_terminator() -> (Vec<u8>, Dispatcher) {
    let input = b"\x1b]2;\xe6\x9c\xab\x1b\\".to_vec();
    let expected = start()
        + Sequence::Osc(
            vec![vec![b'2'], input[4..(input.len() - 2)].to_vec()],
            false,
        )
        + Sequence::Esc(vec![], false, 92);
    (input, expected)
}

advance!(
    advance_osc_containing_string_terminator,
    gen_osc_containing_string_terminator
);

fn gen_exceed_max_buffer_size() -> (Vec<u8>, Dispatcher) {
    static NUM_BYTES: usize = MAX_OSC_RAW + 100;
    static INPUT_START: &[u8] = &[0x1b, b']', b'5', b'2', b';', b's'];
    static INPUT_END: &[u8] = b"\x07";
    let mut input = INPUT_START.to_vec();
    input.resize(INPUT_START.len() + NUM_BYTES, b'a');
    input.extend(INPUT_END);
    let mut param = vec![115];
    #[cfg(not(feature = "core"))]
    param.extend(vec![97; NUM_BYTES + INPUT_END.len() - 1]);
    #[cfg(feature = "core")]
    param.extend(vec![97; MAX_OSC_RAW - INPUT_END.len() - 2]);
    let expected = start() + Sequence::Osc(vec![b"52".to_vec(), param], true);
    (input, expected)
}

advance!(advance_exceed_max_buffer_size, gen_exceed_max_buffer_size);

fn gen_csi_max_params() -> (Vec<u8>, Dispatcher) {
    // This will build a list of repeating '1;'s
    // The length is MAX_PARAMS - 1 because the last semicolon is interpreted
    // as an implicit zero, making the total number of parameters MAX_PARAMS
    let params = "1;".repeat(MAX_PARAMS - 1);
    let input = format!("\x1b[{}p", &params[..]).into_bytes();
    let mut params = vec![vec![1]; MAX_PARAMS - 1];
    params.push(vec![0]);
    let expected = start() + Sequence::Csi(params, vec![], false, b'p');
    (input, expected)
}

advance!(advance_csi_max_params, gen_csi_max_params);

fn gen_csi_params_ignore_long_params() -> (Vec<u8>, Dispatcher) {
    // This will build a list of repeating '1;'s
    // The length is MAX_PARAMS because the last semicolon is interpreted
    // as an implicit zero, making the total number of parameters MAX_PARAMS + 1
    let params = "1;".repeat(MAX_PARAMS);
    let input = format!("\x1b[{}p", &params[..]).into_bytes();
    let params = vec![vec![1]; MAX_PARAMS];
    let expected = start() + Sequence::Csi(params, vec![], true, b'p');
    (input, expected)
}

advance!(
    advance_csi_params_ignore_long_params,
    gen_csi_params_ignore_long_params
);

fn gen_csi_params_trailing_semicolon() -> (Vec<u8>, Dispatcher) {
    let input = b"\x1b[4;m".to_vec();
    let expected = start() + Sequence::Csi(vec![vec![4], vec![0]], vec![], false, b'm');
    (input, expected)
}

advance!(
    advance_csi_params_trailing_semicolon,
    gen_csi_params_trailing_semicolon
);

fn gen_csi_params_leading_semicolon() -> (Vec<u8>, Dispatcher) {
    let input = b"\x1b[;4m".to_vec();
    let expected = start() + Sequence::Csi(vec![vec![0], vec![4]], vec![], false, b'm');
    (input, expected)
}

advance!(
    advance_csi_params_leading_semicolon,
    gen_csi_params_leading_semicolon
);

fn gen_csi_long_param() -> (Vec<u8>, Dispatcher) {
    // The important part is the parameter, which is (i64::MAX + 1)
    let input = b"\x1b[9223372036854775808m".to_vec();
    let expected = start() + Sequence::Csi(vec![vec![u16::MAX]], vec![], false, b'm');
    (input, expected)
}

advance!(advance_csi_long_param, gen_csi_long_param);

fn gen_csi_reset() -> (Vec<u8>, Dispatcher) {
    let input = b"\x1b[3;1\x1b[?1049h".to_vec();
    let expected = start() + Sequence::Csi(vec![vec![1049]], vec![b'?'], false, b'h');
    (input, expected)
}

advance!(advance_csi_reset, gen_csi_reset);

fn gen_csi_subparameters() -> (Vec<u8>, Dispatcher) {
    let input = b"\x1b[38:2:255:0:255;1m".to_vec();
    let expected =
        start() + Sequence::Csi(vec![vec![38, 2, 255, 0, 255], vec![1]], vec![], false, b'm');
    (input, expected)
}

advance!(advance_csi_subparameters, gen_csi_subparameters);

fn gen_dcs_max_params() -> (Vec<u8>, Dispatcher) {
    let params = "1;".repeat(MAX_PARAMS + 1);
    let input = format!("\x1bP{}p", &params[..]).into_bytes();
    let expected = start() + Sequence::DcsHook(vec![vec![1]; MAX_PARAMS], vec![], true, b'p');
    (input, expected)
}

advance!(advance_dcs_max_params, gen_dcs_max_params);

fn gen_dcs_reset() -> (Vec<u8>, Dispatcher) {
    let input = b"\x1b[3;1\x1bP1$tx\x9c".to_vec();
    let expected = start()
        + Sequence::DcsHook(vec![vec![1]], vec![36], false, b't')
        + Sequence::DcsPut(b'x')
        + Sequence::DcsUnhook;
    (input, expected)
}

advance!(advance_dcs_reset, gen_dcs_reset);

fn gen_dcs() -> (Vec<u8>, Dispatcher) {
    let input = b"\x1bP0;1|17/ab\x9c".to_vec();
    let expected = start()
        + Sequence::DcsHook(vec![vec![0], vec![1]], vec![], false, b'|')
        + Sequence::DcsPut(b'1')
        + Sequence::DcsPut(b'7')
        + Sequence::DcsPut(b'/')
        + Sequence::DcsPut(b'a')
        + Sequence::DcsPut(b'b')
        + Sequence::DcsUnhook;
    (input, expected)
}

advance!(advance_dcs, gen_dcs);

fn gen_intermediate_reset_on_dcs_exit() -> (Vec<u8>, Dispatcher) {
    let input = b"\x1bP=1sZZZ\x1b+\x5c".to_vec();
    let expected = start()
        + Sequence::DcsHook(vec![vec![1]], vec![61], false, b's')
        + Sequence::DcsPut(b'Z')
        + Sequence::DcsPut(b'Z')
        + Sequence::DcsPut(b'Z')
        + Sequence::DcsUnhook
        + Sequence::Esc(vec![b'+'], false, b'\\');
    (input, expected)
}

advance!(
    advance_intermediate_reset_on_dcs_exit,
    gen_intermediate_reset_on_dcs_exit
);

fn gen_esc_reset() -> (Vec<u8>, Dispatcher) {
    let input = b"\x1b[3;1\x1b(A".to_vec();
    let expected = start() + Sequence::Esc(vec![b'('], false, b'A');
    (input, expected)
}

advance!(advance_esc_reset, gen_esc_reset);

fn gen_params_buffer_filled_with_subparam() -> (Vec<u8>, Dispatcher) {
    let input = b"\x1b[::::::::::::::::::::::::::::::::x\x1b".to_vec();
    let expected = start() + Sequence::Csi(vec![vec![0; 32]], vec![], true, b'x');
    (input, expected)
}

advance!(
    advance_params_buffer_filled_with_subparam,
    gen_params_buffer_filled_with_subparam
);

proptest! {
    #[test]
    #[cfg(feature = "utf8")]
    #[cfg_attr(any(miri, not(feature = "utf8")), ignore)]
    fn advance_utf8(input in "\\PC*") {
        let expected = Dispatcher::from(input.as_str());

        let mut dispatcher = Dispatcher::default();
        let mut parser = Parser::<Utf8Parser>::new();

        for byte in input.as_bytes() {
            parser.advance(&mut dispatcher, *byte);
        }

        assert_eq!(expected, dispatcher);
    }
}
