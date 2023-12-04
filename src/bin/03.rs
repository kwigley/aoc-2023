advent_of_code::solution!(3);

#[derive(Debug)]
struct Part {
    number: u32,
    locations: Vec<(i32, i32)>,
}

#[derive(Debug)]
struct Engine {
    parts: Vec<Part>,
    symbols: Vec<(i32, i32)>,
}

fn parse_engine(input: &str) -> Engine {
    let mut engine = Engine {
        parts: Vec::new(),
        symbols: Vec::new(),
    };
    for (y, line) in input.lines().enumerate() {
        let mut acc = "".to_string();
        let mut parts: Vec<String> = vec![];
        for c in line.chars() {
            if !c.is_numeric() {
                if !acc.is_empty() {
                    parts.push(acc.clone());
                    acc = "".to_string();
                }
                if c == '.' {
                    parts.push("".to_string());
                } else {
                    parts.push(c.to_string());
                }
            } else {
                acc.push(c);
            }
        }
        parts.push(acc);
        let mut x = 0;
        for part in parts {
            if part.parse::<u32>().is_ok() {
                let len = part.len();
                let mut p = Part {
                    number: part.parse::<u32>().unwrap(),
                    locations: Vec::new(),
                };
                for j in 0..len {
                    p.locations.push((x + j as i32, y as i32));
                }
                engine.parts.push(p);
                x += len as i32;
            } else if !part.is_empty() {
                engine.symbols.push((x, y as i32));
                x += 1;
            } else {
                x += 1;
            }
        }
    }
    engine
}

pub fn part_one(input: &str) -> Option<u32> {
    let engine = parse_engine(input);

    Some(
        engine
            .parts
            .iter()
            .filter(|p| {
                p.locations.iter().any(|l| {
                    engine.symbols.iter().any(|s| {
                        (l.0 + 1 == s.0 && l.1 - 1 == s.1) ||
                        // x l x
                        // x s x
                        // x x x
                        (l.0 == s.0 && l.1 - 1 == s.1) ||
                        // x x l
                        // x s x
                        // x x x
                        (l.0 - 1 == s.0 && l.1 - 1 == s.1) ||
                        // x x x
                        // l s x
                        // x x x
                        (l.0 + 1 == s.0 && l.1 == s.1) ||
                        // x x x
                        // x s l
                        // x x x
                        (l.0 - 1 == s.0 && l.1 == s.1) ||
                        // x x x
                        // x s x
                        // l x x
                        (l.0 + 1 == s.0 && l.1 + 1 == s.1) ||
                        // x x x
                        // x s x
                        // x l x
                        (l.0 == s.0 && l.1 + 1 == s.1) ||
                        // x x x
                        // x s x
                        // x x l
                        (l.0 -1 == s.0 && l.1 + 1 == s.1)
                    })
                })
            })
            .map(|p| p.number)
            .sum(),
    )
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
