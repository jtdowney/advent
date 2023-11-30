use std::{
    collections::HashSet, convert::TryInto, num::ParseIntError, ops::RangeInclusive, str::FromStr,
};

type TicketField = u64;

#[derive(Clone, Debug)]
struct Ticket {
    fields: Vec<TicketField>,
}

impl FromStr for Ticket {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fields = s
            .split(',')
            .map(str::trim)
            .map(str::parse)
            .collect::<Result<Vec<TicketField>, _>>()?;
        let ticket = Ticket { fields };
        Ok(ticket)
    }
}

impl Ticket {
    fn is_valid(&self, rules: &[Rule]) -> bool {
        self.fields
            .iter()
            .any(|field| rules.iter().all(|rule| !rule.is_match(field)))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Rule {
    name: String,
    ranges: [RangeInclusive<TicketField>; 2],
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut parts = value.split(':').map(str::trim);
        let name = parts.next().map(str::to_string).unwrap();
        let ranges = parts
            .next()
            .and_then(|v| {
                v.split(" or ")
                    .map(|r| {
                        r.split('-')
                            .map(str::parse)
                            .collect::<Result<Vec<TicketField>, _>>()
                            .map(|r| (r[0]..=r[1]))
                            .ok()
                    })
                    .collect::<Option<Vec<RangeInclusive<TicketField>>>>()
            })
            .unwrap();
        let rule = Rule {
            name,
            ranges: ranges.try_into().unwrap(),
        };
        Ok(rule)
    }
}

impl Rule {
    fn is_match(&self, field: &TicketField) -> bool {
        self.ranges.iter().any(|rule| rule.contains(field))
    }
}

struct State {
    rules: Vec<Rule>,
    valid_tickets: Vec<Ticket>,
    invalid_tickets: Vec<Ticket>,
    my_ticket: Ticket,
}

#[aoc_generator(day16)]
fn generator(input: &str) -> State {
    let mut parts = input.split("\n\n");
    let rules = parts
        .next()
        .and_then(|part| {
            part.lines()
                .map(str::parse)
                .collect::<Result<Vec<Rule>, _>>()
                .ok()
        })
        .unwrap();
    let my_ticket = parts
        .next()
        .and_then(|part| part.lines().nth(1).and_then(|line| line.parse().ok()))
        .unwrap();
    let nearby_tickets: Vec<Ticket> = parts
        .next()
        .and_then(|part| part.lines().skip(1).map(|line| line.parse().ok()).collect())
        .unwrap();
    let (invalid_tickets, valid_tickets): (Vec<Ticket>, Vec<Ticket>) = nearby_tickets
        .iter()
        .cloned()
        .partition(|ticket| ticket.is_valid(&rules));

    State {
        rules,
        valid_tickets,
        invalid_tickets,
        my_ticket,
    }
}

#[aoc(day16, part1)]
fn part1(state: &State) -> TicketField {
    state
        .invalid_tickets
        .iter()
        .flat_map(|ticket| {
            ticket
                .fields
                .iter()
                .filter(|field| state.rules.iter().all(|rule| !rule.is_match(field)))
        })
        .sum()
}

#[aoc(day16, part2)]
fn part2(state: &State) -> TicketField {
    let fields_length = state.my_ticket.fields.len();
    let mut field_matches = (0..fields_length)
        .map(|field_index| {
            let matching_rules = state
                .rules
                .iter()
                .enumerate()
                .filter(|(_, rule)| {
                    state
                        .valid_tickets
                        .iter()
                        .all(|ticket| rule.is_match(&ticket.fields[field_index]))
                })
                .map(|(i, _)| i)
                .collect::<Vec<_>>();
            (field_index, matching_rules)
        })
        .collect::<Vec<_>>();
    field_matches.sort_by_key(|(_, matching_rules)| matching_rules.len());

    let (rule_indicies, _) = field_matches.iter().fold(
        (vec![0; fields_length], HashSet::new()),
        |(mut rule_incicies, mut used_rules), (field_index, rules)| {
            let candidate_rules: HashSet<_> = rules.iter().copied().collect();
            let next = *candidate_rules.difference(&used_rules).last().unwrap();
            used_rules.insert(next);
            rule_incicies[*field_index] = next;
            (rule_incicies, used_rules)
        },
    );

    state
        .rules
        .iter()
        .enumerate()
        .filter(|(_, r)| r.name.starts_with("departure"))
        .map(|(rule_index, _)| {
            let field_index = rule_indicies.iter().position(|&i| i == rule_index).unwrap();
            state.my_ticket.fields[field_index]
        })
        .product()
}
