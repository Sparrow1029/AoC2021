const PUZZLE_INPUT: &str = concat!(
    "E0529D18025800ABCA6996534CB22E4C00FB48E233BAEC947A8AA010CE1249DB51A02CC7",
    "DB67EF33D4002AE6ACDC40101CF0449AE4D9E4C071802D400F84BD21CAF3C8F2C35295EF",
    "3E0A600848F77893360066C200F476841040401C88908A19B001FD35CCF0B40012992AC8",
    "1E3B980553659366736653A931018027C87332011E2771FFC3CEEC0630A80126007B0152",
    "E2005280186004101060C03C0200DA66006B8018200538012C01F3300660401433801A60",
    "07380132DD993100A4DC01AB0803B1FE2343500042E24C338B33F5852C3E002749803B04",
    "22EC782004221A41A8CE600EC2F8F11FD0037196CF19A67AA926892D2C643675A0C013C0",
    "0CC0401F82F1BA168803510E3942E969C389C40193CFD27C32E005F271CE4B95906C1510",
    "03A7BD229300362D1802727056C00556769101921F200AC74015960E97EC3F2D03C24300",
    "46C0119A3E9A3F95FD3AFE40132CEC52F4017995D9993A90060729EFCA52D3168021223F",
    "2236600ECC874E10CC1F9802F3A71C00964EC46E6580402291FE59E0FCF2B4EC31C9C7A6",
    "860094B2C4D2E880592F1AD7782992D204A82C954EA5A52E8030064D02A6C1E4EA852FE8",
    "3D49CB4AE4020CD80272D3B4AA552D3B4AA5B356F77BF1630056C0119FF16C5192901CED",
    "FB77A200E9E65EAC01693C0BCA76FEBE73487CC64DEC804659274A00CDC401F8B51CE3F8",
    "803B05217C2E40041A72E2516A663F119AC72250A00F44A98893C453005E57415A00BCD5",
    "F1DD66F3448D2600AC66F005246500C9194039C01986B317CDB10890C94BF68E6DF950C0",
    "802B09496E8A3600BCB15CA44425279539B089EB7774DDA33642012DA6B1E15B005C0010",
    "C8C917A2B880391160944D30074401D845172180803D1AA3045F00042630C5B866200CC2",
    "A9A5091C43BBD964D7F5D8914B46F040"
);

enum EndSubpackets {
    NumBits(usize),
    CountSubpackets(usize),
}

type BitIndex = usize;

pub struct Accumulator {
    value: usize,
}

impl Accumulator {
    fn increment(&mut self, amt: usize) {
        self.value += amt;
    }
}

fn create_bitvec(input: &str) -> Vec<u8> {
    (0..input.len() - 1)
        .step_by(2)
        .map(|i| u8::from_str_radix(&input[i..i + 2], 16).expect("All chars should be hex"))
        .collect()
}

fn read<const N: usize>(input: &[u8], start_bits: usize) -> usize {
    const BITS: usize = u8::BITS as usize;

    let mut result = 0;
    for i in 0..N {
        let index = (start_bits + i) / BITS;
        let offset = (start_bits + i) % BITS;
        let next_bit = (input[index] >> (BITS - 1 - offset)) & 0x1;

        result = result << 1 | next_bit as usize;
    }
    // println!("read value: {result:<0N$b}");
    result
}

fn get_literal_value(input: &[u8], start_bit: BitIndex) -> (usize, BitIndex) {
    let mut bit_idx = start_bit;
    let mut value = 0;
    loop {
        let continue_reading = read::<1>(input, bit_idx);
        value = value << 4 | read::<4>(input, bit_idx + 1);
        bit_idx += 5;

        if continue_reading == 0 {
            break;
        }
    }
    (value, bit_idx)
}

fn process_operator_packet(
    input: &[u8],
    start_bit: BitIndex,
    type_id: usize,
    version_sum: &mut Accumulator,
    value_sum: &mut Accumulator,
) -> (usize, BitIndex) {
    let mut bit_idx = start_bit;
    let length_id = read::<1>(input, bit_idx);
    bit_idx += 1;

    let subpacket_condition = if length_id == 0 {
        // Next 15 bits represent total length in bits of sub-packets contained in this packet.
        let end_subpacket_bits = read::<15>(input, bit_idx) + 15 + bit_idx;
        bit_idx += 15;

        EndSubpackets::NumBits(end_subpacket_bits)
    } else {
        // Next 11 bits represent the nmer of sub-packets immediately contained by this packet.
        let subpacket_count = read::<11>(input, bit_idx);
        bit_idx += 11;

        EndSubpackets::CountSubpackets(subpacket_count)
    };

    let mut subpackets = 1;
    let mut subpacket_values = {
        // Process first packet
        let (value, subpacket_bit_idx) = parse(input, bit_idx, version_sum, value_sum);
        bit_idx = subpacket_bit_idx;

        value
    };

    while match subpacket_condition {
        EndSubpackets::NumBits(x) => bit_idx < x,
        EndSubpackets::CountSubpackets(x) => subpackets < x,
    } {
        let (value, subpacket_bit_idx) = parse(input, bit_idx, version_sum, value_sum);
        bit_idx = subpacket_bit_idx;
        subpackets += 1;

        match type_id {
            0 => subpacket_values += value,
            1 => subpacket_values *= value,
            2 => subpacket_values = std::cmp::min(subpacket_values, value),
            3 => subpacket_values = std::cmp::max(subpacket_values, value),
            5 => subpacket_values = if value < subpacket_values { 1 } else { 0 },
            6 => subpacket_values = if value > subpacket_values { 1 } else { 0 },
            7 => subpacket_values = if value == subpacket_values { 1 } else { 0 },
            _ => unreachable!(),
        }
    }

    (subpacket_values, bit_idx)
}

// parse takes version, type
// if type == 4 get literal value for this packet
// else
// type is operation over next set of packets:
// match next bit
//   0 -> get next 15 bits for num bits to 'take'
//   1 -> get next 11 bits for num times to call parse...?
pub fn parse(
    input: &[u8],
    start_bit: BitIndex,
    version_sum: &mut Accumulator,
    value_sum: &mut Accumulator,
) -> (usize, BitIndex) {
    let mut bit_idx = start_bit;

    let version = read::<3>(input, bit_idx);
    version_sum.increment(version);
    let type_id = read::<3>(input, bit_idx + 3);
    bit_idx += 6;

    match type_id {
        4 => {
            let (new_value, new_bit_idx) = get_literal_value(input, bit_idx);
            value_sum.increment(new_value);
            (new_value, new_bit_idx)
        }
        _ => {
            let (new_value, new_bit_idx) =
                process_operator_packet(input, bit_idx, type_id, version_sum, value_sum);
            value_sum.increment(new_value);
            (new_value, new_bit_idx)
        }
    }
}

pub fn run() {
    let input = PUZZLE_INPUT;
    let bitvec: &[u8] = &create_bitvec(input);
    let mut version_sum = Accumulator { value: 0 };
    let mut value_sum = Accumulator { value: 0 };
    let (value, final_bit_idx) = parse(bitvec, 0, &mut version_sum, &mut value_sum);
    println!(
        "Value {value}, final bit_idx {final_bit_idx}, value_sum {}, version_sum {}",
        value_sum.value, version_sum.value
    );
}
