use std::io::{self, BufRead, BufReader, Read};
use std::str::FromStr;

use crate::parser::{Cell, CellStatus, Config, ParseError, Pattern, Point, Rules};

pub fn parse(input: impl Read) -> Result<Pattern, ParseError> {
    let buf_reader = BufReader::new(input);
    let mut content_line_iter = buf_reader
        .lines()
        .filter(|result| result.as_ref().map(|line| !line.is_empty()).unwrap_or(true))
        .map(|result| result.map(|line| line.trim().to_owned())); // TODO: check if owning is necessary here

    let version = if let Some(line) = content_line_iter.next() {
        let line = line?;
        if &line[..5] == "#Life" {
            line[5..].trim().to_owned()
        } else {
            return Err(ParseError::InvalidFormat(
                "Unknown .life/.lif format".to_owned(),
            ));
        }
    } else {
        return Err(ParseError::Empty);
    };

    match version.as_str() {
        "1.05" => parse_life_105(&mut content_line_iter),
        "1.06" => parse_life_106(&mut content_line_iter),
        _ => {
            return Err(ParseError::InvalidFormat(
                "Unknown .life/.lif version".to_owned(),
            ))
        }
    }
}

fn parse_comment(config: &mut Config, line: &str) -> Result<(), ParseError> {
    if line.is_empty() || line == "#" {
        return Ok(());
    } else if &line[..1] != "#" {
        return Err(ParseError::InvalidFormat(
            "Tried to parse and invalid comment line".to_owned(),
        ));
    }

    match &line[..2] {
        "#N" => config.ruleset = Some(Rules::default()),
        "#C" | "#D" => config.description.push(line[2..].trim().to_owned()),
        "#R" => config.ruleset = Some(Rules::from_str(&line[2..].trim())?),
        "#O" => config.author = Some(line[2..].trim().to_owned()),
        _ => {} // TODO: Log when unknown "#" line is found
    }

    Ok(())
}

fn parse_life_105(input: impl Iterator<Item = io::Result<String>>) -> Result<Pattern, ParseError> {
    let mut config = Config::default();
    let mut cells = Vec::new();
    let mut current_block_origin = Point::default();
    let mut current_y_offset = 0;
    for line in input {
        let line = line?;
        if &line[..1] == "#" {
            match &line[..2] {
                "#P" => {
                    current_block_origin = Point::from_str(&line[2..])?;
                    current_y_offset = 0;
                }
                _ => parse_comment(&mut config, &line)?,
            }
        } else {
            for (i, c) in line.chars().enumerate() {
                if c == '*' {
                    cells.push(Cell::from_coords(
                        current_block_origin.x + i as isize,
                        current_block_origin.y + current_y_offset,
                    ))
                }
            }
            current_y_offset += 1;
        }
    }
    Ok(Pattern::new(cells, config))
}

fn parse_life_106(input: impl Iterator<Item = io::Result<String>>) -> Result<Pattern, ParseError> {
    let mut config = Config::default();
    let mut cells = Vec::new();
    for line in input {
        let line = line?;
        if &line[..1] == "#" {
            parse_comment(&mut config, &line)?;
        } else {
            let coords = Point::from_str(&line)?;
            cells.push(Cell::new(coords, CellStatus::Alive));
        }
    }
    Ok(Pattern::new(cells, config))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Size;

    #[test]
    fn parse_life_106_glider() {
        let input = "#Life 1.06
#R 23/3
0 0
0 1
1 0

#C Explaining something.
1 2
2 0"
        .as_bytes();
        let output = Pattern {
            alive: vec![
                Cell::from_coords(0, 0),
                Cell::from_coords(0, 1),
                Cell::from_coords(1, 0),
                Cell::from_coords(1, 2),
                Cell::from_coords(2, 0),
            ],
            config: Config {
                author: None,
                description: vec!["Explaining something.".to_owned()],
                ruleset: Some(Rules::new(vec![2, 3], vec![3])),
                wrap_edges: false,
            },
        };

        let parsed = parse(input).unwrap();
        assert_eq!(parsed.size(), Size::new(3, 3));
        assert_eq!(parsed, output);
    }

    #[test]
    fn parse_life_105_glider() {
        let input = "#Life 1.05
#D This is a glider.
#C      It's an easy pattern.
#O John Doe
#N
#P 0 0 
***

*..
.*.

#P 12 5
*.*
.
*"
        .as_bytes();
        let output = Pattern {
            alive: vec![
                Cell::from_coords(0, 0),
                Cell::from_coords(1, 0),
                Cell::from_coords(2, 0),
                Cell::from_coords(0, 1),
                Cell::from_coords(1, 2),
                Cell::from_coords(12, 5),
                Cell::from_coords(14, 5),
                Cell::from_coords(12, 7),
            ],
            config: Config {
                author: Some(String::from("John Doe")),
                description: vec![
                    "This is a glider.".to_owned(),
                    "It's an easy pattern.".to_owned(),
                ],
                ruleset: Some(Rules::default()),
                wrap_edges: false,
            },
        };
        let parsed = parse(input).unwrap();
        assert_eq!(parsed.size(), Size::new(15, 8));
        assert_eq!(parsed, output);
    }
}
