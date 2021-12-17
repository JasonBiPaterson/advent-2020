use std::{fs, slice::Iter};

type Bits<'a> = Iter<'a, bool>;

fn parse_input(input: &str) -> Vec<bool> {
    input
        .trim()
        .chars()
        .flat_map(|c| {
            let n = c.to_digit(16).unwrap();
            (0_u32..=3).rev().map(move |m| (n >> m & 1) == 1)
        })
        .collect()
}

fn drain_to_int(bits: &mut Bits, value: &mut usize, n: usize) -> usize {
    for &b in bits.take(n) {
        *value <<= 1;
        *value |= b as usize;
    }
    *value
}

fn literal_value(bits: &mut Bits) -> usize {
    let mut value = 0;
    loop {
        let group_prefix = *bits.next().unwrap();
        drain_to_int(bits, &mut value, 4);
        if !group_prefix {
            break;
        }
    }
    value
}

fn parse_packet(bits: &mut Bits) -> (usize, usize) {
    let version = drain_to_int(bits, &mut 0, 3);
    let type_id = drain_to_int(bits, &mut 0, 3);

    if type_id == 4 {
        let value = literal_value(bits);
        return (version, value);
    }

    let mut version_sum = 0;
    let mut values = vec![];
    let length_type = *bits.next().unwrap();

    if length_type {
        let no_sub_packets = drain_to_int(bits, &mut 0, 11);

        for _ in 0..no_sub_packets {
            let (version, value) = parse_packet(bits);
            version_sum += version;
            values.push(value);
        }
    } else {
        let sub_packet_length = drain_to_int(bits, &mut 0, 15);
        let start_len = bits.len();

        while start_len - bits.len() < sub_packet_length {
            let (version, value) = parse_packet(bits);
            version_sum += version;
            values.push(value);
        }
    }

    let value = match type_id {
        0 => values.iter().sum(),
        1 => values.iter().product(),
        2 => *values.iter().min().unwrap(),
        3 => *values.iter().max().unwrap(),
        5 => (values[0] > values[1]) as usize,
        6 => (values[0] < values[1]) as usize,
        7 => (values[0] == values[1]) as usize,
        _ => panic!(),
    };

    (version + version_sum, value)
}

fn main() {
    let input: String = fs::read_to_string("input/day16").expect("Failed to read input");

    let bits = parse_input(&input);

    let (part1, part2) = parse_packet(&mut bits.iter());

    println!("Part 1: {}", part1);
    println!("Part 1: {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let test_inputs = [
            ("8A004A801A8002F478", 16),
            ("620080001611562C8802118E34", 12),
            ("C0015000016115A2E0802F182340", 23),
            ("A0016C880162017C3686B18A3D4780", 31),
        ];
        for (input, result) in test_inputs {
            assert_eq!(parse_packet(&mut parse_input(input).iter()).0, result);
        }
    }

    #[test]
    fn test_part2() {
        let test_inputs = [
            ("C200B40A82", 3),
            ("04005AC33890", 54),
            ("880086C3E88112", 7),
            ("CE00C43D881120", 9),
            ("D8005AC2A8F0", 1),
            ("F600BC2D8F", 0),
            ("9C005AC2F8F0", 0),
            ("9C0141080250320F1802104A08", 1),
        ];
        for (input, result) in test_inputs {
            assert_eq!(parse_packet(&mut parse_input(input).iter()).1, result);
        }
    }
}
