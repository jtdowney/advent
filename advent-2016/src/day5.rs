use aoc_runner_derive::aoc;

#[aoc(day5, part1)]
fn part1(input: &str) -> String {
    (0..)
        .map(|n| format!("{:x}", md5::compute(format!("{}{}", input, n))))
        .filter_map(|hash| {
            if hash.starts_with("00000") {
                Some(hash.chars().nth(5).unwrap())
            } else {
                None
            }
        })
        .take(8)
        .collect()
}

#[aoc(day5, part2)]
fn part2(input: &str) -> Option<String> {
    (0..)
        .map(|n| format!("{:x}", md5::compute(format!("{}{}", input, n))))
        .filter(|hash| hash.starts_with("00000"))
        .scan([None; 8], |password, hash| {
            let index = hash.chars().nth(5).unwrap().to_digit(16).unwrap() as usize;
            if index < 8 && password[index].is_none() {
                password[index] = Some(hash.chars().nth(6).unwrap());
            }

            Some(password.to_vec())
        })
        .find(|password| password.iter().all(|&c| c.is_some()))
        .map(|password| password.into_iter().map(|c| c.unwrap()).collect())
}
