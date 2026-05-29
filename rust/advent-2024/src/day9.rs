use std::collections::BTreeMap;

use anyhow::Context;
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, Copy)]
struct Block {
    id: u64,
    length: u8,
}

type Disk = BTreeMap<u64, Block>;
type Freelist = BTreeMap<u64, u8>;

#[aoc_generator(day9)]
fn generator(input: &str) -> anyhow::Result<(Disk, Freelist)> {
    #[derive(Default)]
    struct State {
        disk: Disk,
        freelist: Freelist,
        position: u64,
    }

    let State { disk, freelist, .. } =
        input
            .chars()
            .enumerate()
            .try_fold(State::default(), |mut acc, (i, c)| {
                let length = c.to_digit(10).context("invalid digit")? as u8;
                if i % 2 == 0 {
                    let id = i as u64 / 2;
                    acc.disk.insert(acc.position, Block { id, length });
                } else if length > 0 {
                    acc.freelist.insert(acc.position, length);
                }

                acc.position += length as u64;
                anyhow::Ok(acc)
            })?;
    Ok((disk, freelist))
}

fn compact(disk: &Disk, freelist: &Freelist) -> Disk {
    let mut disk = disk.clone();
    let mut freelist = freelist.clone();
    let mut compacted = Disk::new();

    while let Some((position, block)) = disk.pop_last() {
        let (free_position, free_length) = freelist.pop_first().unwrap();
        if free_position > position {
            compacted.insert(position, block);
            continue;
        }

        if free_length < block.length {
            let fragment_length = block.length - free_length;
            freelist.insert(position, free_length);
            disk.insert(
                position + free_length as u64,
                Block {
                    id: block.id,
                    length: fragment_length,
                },
            );
            compacted.insert(
                free_position,
                Block {
                    id: block.id,
                    length: free_length,
                },
            );
        } else {
            let remaining_length = free_length - block.length;
            if remaining_length > 0 {
                freelist.insert(free_position + block.length as u64, remaining_length);
            }

            freelist.insert(position, block.length);
            compacted.insert(free_position, block);
        }
    }

    compacted
}

fn compact2(disk: &Disk, freelist: &Freelist) -> Disk {
    let mut disk = disk.clone();
    let mut freelist = freelist.clone();
    let mut compacted = Disk::new();

    while let Some((position, block)) = disk.pop_last() {
        let Some((&free_position, &free_length)) = freelist
            .iter()
            .find(|&(&free_position, &length)| free_position < position && length >= block.length)
        else {
            compacted.insert(position, block);
            continue;
        };

        freelist.remove(&free_position);

        let remaining_length = free_length - block.length;
        if remaining_length > 0 {
            freelist.insert(free_position + block.length as u64, remaining_length);
        }

        compacted.insert(free_position, block);
    }

    compacted
}

fn checksum(disk: &Disk) -> u64 {
    disk.iter()
        .map(|(position, &Block { id, length })| {
            (0..length).map(|i| id * (position + i as u64)).sum::<u64>()
        })
        .sum()
}

#[aoc(day9, part1)]
fn part1((disk, freelist): &(Disk, Freelist)) -> u64 {
    let disk = compact(disk, freelist);
    checksum(&disk)
}

#[aoc(day9, part2)]
fn part2((disk, freelist): &(Disk, Freelist)) -> u64 {
    let disk = compact2(disk, freelist);
    checksum(&disk)
}
