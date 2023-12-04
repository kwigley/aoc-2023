use itertools::Itertools;

advent_of_code::solution!(2);

#[derive(Debug)]
struct Set {
    blue: u32,
    red: u32,
    green: u32,
}

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<Set>,
}

fn parse_games(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|l| {
            if let Some((id, sets)) = l
                .split_terminator(':')
                .map(|p| p.trim())
                .collect_vec()
                .split_first()
            {
                let mut game = Game {
                    id: id.split_ascii_whitespace().last().unwrap().parse().unwrap(),
                    sets: Vec::new(),
                };
                for set in sets[0].split_terminator(';').collect_vec() {
                    let mut to_return = Set {
                        blue: 0,
                        red: 0,
                        green: 0,
                    };
                    for i in set.split_terminator(',').map(|cube| {
                        let parts = cube.split_ascii_whitespace();
                        parts.map(|p| p.trim()).collect_vec()
                    }) {
                        let j = i.split_at(1);
                        match (j.1[0], j.0[0].parse::<u32>().unwrap()) {
                            ("blue", n) => to_return.blue += n,
                            ("red", n) => to_return.red += n,
                            ("green", n) => to_return.green += n,
                            _ => panic!("Invalid input"),
                        }
                    }
                    game.sets.push(to_return);
                }
                game
            } else {
                panic!("Invalid input");
            }
        })
        .collect_vec()
}

pub fn part_one(input: &str) -> Option<u32> {
    let games = parse_games(input);

    Some(
        games
            .iter()
            .filter(|g| {
                g.sets
                    .iter()
                    .all(|s| s.blue <= 14 && s.red <= 12 && s.green <= 13)
            })
            .map(|g| g.id)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let games = parse_games(input);

    Some(
        games
            .iter()
            .map(|g| {
                let blue = g.sets.iter().map(|s| s.blue).max().unwrap();
                let green = g.sets.iter().map(|s| s.green).max().unwrap();
                let red = g.sets.iter().map(|s| s.red).max().unwrap();
                blue * green * red
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
