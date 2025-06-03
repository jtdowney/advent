use anyhow::anyhow;
use nom::{
    Finish, IResult, Parser,
    bits::{bits, complete::take},
    branch::alt,
    bytes::complete::take_while_m_n,
    combinator::{map_res, verify},
    multi::many1,
};

type BitSlice<'a> = (&'a [u8], usize);

#[derive(Debug, Clone)]
enum Packet {
    Literal {
        version: u8,
        value: u64,
    },
    Operator {
        version: u8,
        operation: Operation,
        packets: Vec<Packet>,
    },
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

fn hex_digit(input: &str) -> IResult<&str, u8> {
    map_res(
        take_while_m_n(2, 2, |c: char| c.is_ascii_hexdigit()),
        |s: &str| u8::from_str_radix(s, 16),
    )
    .parse(input)
}

fn hex_string(input: &str) -> IResult<&str, Vec<u8>> {
    many1(hex_digit).parse(input)
}

struct PacketHeader {
    version: u8,
    packet_type: u8,
}

fn header(input: BitSlice) -> IResult<BitSlice, PacketHeader> {
    let (input, version) = take(3usize).parse(input)?;
    let (input, packet_type) = take(3usize).parse(input)?;

    Ok((
        input,
        PacketHeader {
            version,
            packet_type,
        },
    ))
}

fn variable_length_value(input: BitSlice) -> IResult<BitSlice, u64> {
    let mut nibbles = Vec::new();
    let mut remaining = input;

    loop {
        let (input, continue_bit) = take::<_, u8, _, _>(1usize).parse(remaining)?;
        let (input, nibble) = take::<_, u8, _, _>(4usize).parse(input)?;
        nibbles.push(nibble);
        remaining = input;

        if continue_bit == 0 {
            break;
        }
    }

    let value = nibbles
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (offset, &nibble)| {
            acc | ((nibble as u64) << (offset * 4))
        });

    Ok((remaining, value))
}

fn literal_packet(input: BitSlice) -> IResult<BitSlice, Packet> {
    let (input, header) = verify(header, |header| header.packet_type == 4).parse(input)?;
    let (input, value) = variable_length_value(input)?;

    let version = header.version;
    Ok((input, Packet::Literal { version, value }))
}

fn consumed_length(a: BitSlice, b: BitSlice) -> usize {
    let (input_a, index_a) = a;
    let (input_b, index_b) = b;
    (input_a.len() * 8 - index_a) - (input_b.len() * 8 - index_b)
}

fn operator_subpackets(input: BitSlice) -> IResult<BitSlice, Vec<Packet>> {
    let (input, length_type) = take::<_, u8, _, _>(1usize).parse(input)?;

    if length_type == 0 {
        // Subpackets by total length
        let (input, length) = take::<_, u16, _, _>(15usize).parse(input)?;
        let length = length as usize;

        let mut packets = vec![];
        let mut remaining_input = input;
        let mut consumed = 0;
        while consumed < length {
            let (next_input, packet) = packet(remaining_input)?;

            consumed += consumed_length(remaining_input, next_input);
            remaining_input = next_input;

            packets.push(packet);
        }

        Ok((remaining_input, packets))
    } else {
        // Subpackets by count
        let (input, count) = take::<_, u16, _, _>(11usize).parse(input)?;
        let mut packets = vec![];
        let mut remaining_input = input;

        for _ in 0..count {
            let (next_input, packet) = packet(remaining_input)?;
            packets.push(packet);
            remaining_input = next_input;
        }

        Ok((remaining_input, packets))
    }
}

fn operator_packet(input: BitSlice) -> IResult<BitSlice, Packet> {
    let (input, header) = verify(header, |header| header.packet_type != 4).parse(input)?;
    let (input, packets) = operator_subpackets(input)?;

    let version = header.version;
    let operation = match header.packet_type {
        0 => Operation::Sum,
        1 => Operation::Product,
        2 => Operation::Minimum,
        3 => Operation::Maximum,
        5 => Operation::GreaterThan,
        6 => Operation::LessThan,
        7 => Operation::EqualTo,
        _ => unreachable!(),
    };

    Ok((
        input,
        Packet::Operator {
            version,
            operation,
            packets,
        },
    ))
}

fn packet(input: BitSlice) -> IResult<BitSlice, Packet> {
    alt((literal_packet, operator_packet)).parse(input)
}

fn parse_packet(input: &[u8]) -> IResult<&[u8], Packet> {
    bits(packet).parse(input)
}

#[aoc_generator(day16)]
fn generator(input: &str) -> anyhow::Result<Packet> {
    let (_, data) = hex_string(input)
        .finish()
        .map_err(|_| anyhow!("unable to parse hex"))?;

    let (_, packet) = parse_packet(&data)
        .finish()
        .map_err(|_| anyhow!("unable to parse packets"))?;

    Ok(packet)
}

#[aoc(day16, part1)]
fn part1(input: &Packet) -> usize {
    let mut search = vec![input];
    let mut count = 0;

    while let Some(packet) = search.pop() {
        match packet {
            Packet::Literal { version, .. } => count += *version as usize,
            Packet::Operator {
                version, packets, ..
            } => {
                count += *version as usize;
                search.extend(packets);
            }
        }
    }

    count
}

fn evaluate(packet: &Packet) -> u64 {
    match packet {
        Packet::Literal { value, .. } => *value,
        Packet::Operator {
            operation, packets, ..
        } => match operation {
            Operation::Sum => packets.iter().map(evaluate).sum(),
            Operation::Product => packets.iter().map(evaluate).product(),
            Operation::Minimum => packets.iter().map(evaluate).min().unwrap_or_default(),
            Operation::Maximum => packets.iter().map(evaluate).max().unwrap_or_default(),
            Operation::GreaterThan => {
                let left = evaluate(&packets[0]);
                let right = evaluate(&packets[1]);
                if left > right { 1 } else { 0 }
            }
            Operation::LessThan => {
                let left = evaluate(&packets[0]);
                let right = evaluate(&packets[1]);
                if left < right { 1 } else { 0 }
            }
            Operation::EqualTo => {
                let left = evaluate(&packets[0]);
                let right = evaluate(&packets[1]);
                if left == right { 1 } else { 0 }
            }
        },
    }
}

#[aoc(day16, part2)]
fn part2(input: &Packet) -> u64 {
    evaluate(input)
}
