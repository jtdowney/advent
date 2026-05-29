use aoc_runner_derive::{aoc, aoc_generator};

struct Node {
    children: Vec<Node>,
    metadata: Vec<usize>,
}

impl Node {
    fn metadata_sum(&self) -> usize {
        let mine = self.metadata.iter().cloned().sum::<usize>();
        let children = self
            .children
            .iter()
            .map(|c| c.metadata_sum())
            .sum::<usize>();
        mine + children
    }

    fn value(&self) -> usize {
        if self.children.is_empty() {
            self.metadata_sum()
        } else {
            self.metadata
                .iter()
                .filter_map(|i| self.children.get(i - 1).map(|c| c.value()))
                .sum()
        }
    }
}

struct Parser<I: Iterator<Item = usize>> {
    inner: I,
}

impl<I: Iterator<Item = usize>> Parser<I> {
    fn new<T>(iter: T) -> Parser<I>
    where
        T: IntoIterator<Item = usize, IntoIter = I>,
    {
        Parser {
            inner: iter.into_iter(),
        }
    }

    fn parse_node(&mut self) -> Option<Node> {
        let children_count = self.inner.next()?;
        let metadata_count = self.inner.next()?;
        let children = (0..children_count)
            .map(|_| self.parse_node())
            .collect::<Option<Vec<Node>>>()?;
        let metadata = (0..metadata_count)
            .map(|_| self.inner.next())
            .collect::<Option<Vec<usize>>>()?;

        Some(Node { children, metadata })
    }
}

#[aoc_generator(day8)]
fn generator(input: &str) -> Option<Node> {
    input
        .split_whitespace()
        .map(|part| part.parse())
        .collect::<Result<Vec<usize>, _>>()
        .ok()
        .and_then(|iter| Parser::new(iter).parse_node())
}

#[aoc(day8, part1)]
fn part1(input: &Node) -> usize {
    input.metadata_sum()
}

#[aoc(day8, part2)]
fn part2(input: &Node) -> usize {
    input.value()
}
