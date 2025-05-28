use anyhow::anyhow;
use aoc_runner_derive::aoc;
use nom::{Finish, IResult, Parser};

fn marker(input: &[u8]) -> IResult<&[u8], (usize, usize)> {
    use nom::{
        bytes::complete::tag,
        character::complete::u32,
        combinator::map,
        sequence::delimited,
    };

    map(
        delimited(tag("("), |input| {
            let (input, length) = u32(input)?;
            let (input, _) = tag("x")(input)?;
            let (input, count) = u32(input)?;
            Ok((input, (length, count)))
        }, tag(")")),
        |(length, count)| (length as usize, count as usize),
    ).parse(input)
}

fn decompress(mut input: &[u8]) -> anyhow::Result<Vec<u8>> {
    let mut decompressed = Vec::with_capacity(input.len() * 2);
    if !input.iter().any(|&b| b == b'(') {
        return Ok(input.to_vec());
    }

    while let Some(i) = input.iter().position(|&b| b == b'(') {
        decompressed.extend_from_slice(&input[..i]);

        let (rest, (length, count)) = marker
            .parse(&input[i..])
            .finish()
            .map_err(|e| anyhow!("Error parsing marker: {:?}", e))?;
        let (data, rest) = rest.split_at(length);

        decompressed.extend(data.iter().cycle().take(length * count));
        input = rest;
    }

    Ok(decompressed)
}

#[aoc(day9, part1)]
fn part1(input: &[u8]) -> anyhow::Result<usize> {
    let decompressed = decompress(input)?;
    Ok(decompressed.len())
}

#[aoc(day9, part2)]
fn part2(input: &[u8]) -> anyhow::Result<usize> {
    let mut data = input.to_vec();
    loop {
        let next = decompress(&data)?;
        if next.len() == data.len() {
            break;
        }

        data = next;
    }

    Ok(data.len())
}
