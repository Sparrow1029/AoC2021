/// Copied from here: https://docs.rs/mbryant-aoc2021/latest/src/mbryant_aoc2021/lib.rs.html#1705-1850
/// I was trying to do something similar with reading the bitstream, but the `read` function was a great implementation

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

/// Parse every two characters as a hex byte
fn convert_hex_input(input: &str) -> Vec<u8> {
    // Skip NUL byte
    (0..input.len() - 1)
        .step_by(2)
        .map(|i| u8::from_str_radix(&input[i..i + 2], 16).expect("All chars should be hex"))
        .collect()
}

/// Type alias for index of bit inside bit array
type BitIndex = usize;

/// Read `N` bits from input starting at `start_bit`.
///
/// naïvely read one bit at a time.
/// TIL: const generics passed as func param...
fn read<const N: usize>(input: &[u8], start_bit: BitIndex) -> usize {
    const BITS: usize = u8::BITS as usize; // size of u8 in bits (8)

    let mut result = 0;

    for i in 0..N {
        let index = (start_bit + i) / BITS;
        let offset = (start_bit + i) % BITS;
        let next_bit = (input[index] >> (BITS - 1 - offset)) & 0x1;

        result = result << 1 | next_bit as usize;
    }
    println!("{result:<03b}");

    result
}

/// Recursively parse input from `start_bit`, returning (computed value, next bit to be read).
///
/// If `SUM_VERSIONS` is `true` (part 1), value returned will be sum of all packet versions.
/// Otherwise, value returned will be a result of the expression defined by the packet's operator type.
fn parse<const SUM_VERSIONS: bool>(input: &[u8], start_bit: BitIndex) -> (usize, BitIndex) {
    let mut cur_bit = start_bit;

    let version = read::<3>(input, cur_bit);
    println!("version {version} ({version:<03b}");
    let type_id = read::<3>(input, cur_bit + 3);
    println!("type_id {type_id} ({type_id:<03b}");
    cur_bit += 6;

    let value = match type_id {
        4 => {
            // "literal value" packet
            let mut value = 0;
            loop {
                let continue_reading = read::<1>(input, cur_bit);
                value = value << 4 | read::<4>(input, cur_bit + 1);
                cur_bit += 5;

                if continue_reading == 0 {
                    break;
                }
            }
            if SUM_VERSIONS {
                version
            } else {
                value
            }
        }
        _ => {
            // "Operator" packet
            let length_id = read::<1>(input, cur_bit);
            cur_bit += 1;

            // Helper to differentiate what the length_type_id represents for the next packet bits
            enum EndSubpackets {
                Bits(usize),
                Count(usize),
            }

            let subpacket_condition = if length_id == 0 {
                // next 15 bits represent total length in bits of sub-packets contained in this packet.
                let end_subpacket_bits = read::<15>(input, cur_bit) + 15 + cur_bit;
                cur_bit += 15;

                EndSubpackets::Bits(end_subpacket_bits)
            } else {
                // next 11 bits represent number of sub-packets immediately contained by this packet.
                let subpacket_count = read::<11>(input, cur_bit);
                cur_bit += 11;

                EndSubpackets::Count(subpacket_count)
            };

            let mut subpackets = 1;
            let mut subpacket_values = if SUM_VERSIONS {
                println!("SUM_VERSIONS is true returning version ({version})");
                version
            } else {
                // Parse the first packet so we can initialize the subpacket values properly.
                let (value, subpacket_start_bit) = parse::<SUM_VERSIONS>(input, cur_bit);
                cur_bit = subpacket_start_bit;

                value
            };

            while match subpacket_condition {
                EndSubpackets::Bits(x) => cur_bit < x,
                EndSubpackets::Count(x) => subpackets < x,
            } {
                let (value, subpacket_start_bit) = parse::<SUM_VERSIONS>(input, cur_bit);
                cur_bit = subpacket_start_bit;
                subpackets += 1;

                if SUM_VERSIONS {
                    println!("In subpacket - SUM_VERSIONS=true so adding val {value} to spkt_vals {subpacket_values}");
                    subpacket_values += value;
                } else {
                    match type_id {
                        0 => subpacket_values += value,
                        1 => subpacket_values *= value,
                        2 => subpacket_values += std::cmp::min(subpacket_values, value),
                        3 => subpacket_values += std::cmp::max(subpacket_values, value),
                        5 => subpacket_values += if value < subpacket_values { 1 } else { 0 },
                        6 => subpacket_values += if value > subpacket_values { 1 } else { 0 },
                        7 => subpacket_values += if value == subpacket_values { 1 } else { 0 },
                        _ => unreachable!(),
                    }
                }
            }
            subpacket_values
        }
    };

    (value, cur_bit)
}

pub fn run() {
    let test_str = "8A004A801A8002F478";
    let input: &[u8] = &convert_hex_input(test_str);
    println!("Input as ints: {input:?}");
    for i in input.iter() {
        print!("{i:b}");
    }
    let result = parse::<true>(input, 0);
    println!("{result:?}")
}
