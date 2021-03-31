use std::io::{self, BufRead, BufReader, Read};
use std::iter::Peekable;
use std::str::FromStr;

use crate::parse::{Cell, Size};
use crate::parse::{ParseError, Pattern, PatternConfig, Rules};

pub fn parse(input: impl Read) -> Result<Pattern, ParseError> {
    let buf_reader = BufReader::new(input);
    let mut content_line_iter = buf_reader
        .lines()
        .filter(|l| {
            if let Ok(line) = l.as_ref() {
                !line.is_empty()
            } else {
                false
            }
        })
        // This doesn't seem to be working as expected (it's not trimming spaces on all lines)
        .map(|l| {
            if let Ok(line) = l.as_ref() {
                Ok(line.trim().to_owned())
            } else {
                l
            }
        })
        .peekable();

    let version = parse_version(&mut content_line_iter)?;
    let config = parse_header(&mut content_line_iter)?;

    let alive_list = match version.as_ref() {
        "1.05" => parse_life_105_cell_blocks(&mut content_line_iter)?,
        "1.06" => parse_life_106_list(&mut content_line_iter)?,
        _ => {
            return Err(ParseError::InvalidFormat(
                "Unknown .life/.lif version".to_owned(),
            ))
        }
    };

    // TODO: Calculate size from alive_list and adjust list to center on top left.
    let size = Size {
        width: 0,
        height: 0,
    };

    Ok(Pattern {
        size,
        alive_list,
        config,
    })
}

fn parse_version(
    input: &mut impl Iterator<Item = io::Result<String>>,
) -> Result<String, ParseError> {
    if let Some(line) = input.next() {
        let line = line?;
        if &line[..5] != "#Life" {
            Err(ParseError::InvalidFormat(
                "Invalid .life/.lif file format".to_owned(),
            ))
        } else {
            Ok(line[6..].to_owned())
        }
    } else {
        Err(ParseError::Empty)
    }
}

fn parse_header(
    input: &mut Peekable<impl Iterator<Item = io::Result<String>>>,
) -> Result<PatternConfig, ParseError> {
    let mut description: Option<String> = None;
    let mut ruleset = None;
    let mut author = None;
    let wrap_edges = false;

    // Fails to parse last header line, if file has no content
    while let Some(next_line) = input.peek() {
        if let Ok(next_line_str) = next_line {
            // TODO: Handle empty lines
            if &next_line_str[..1] != "#" || &next_line_str[..2] == "#P" {
                return Ok(PatternConfig {
                    description,
                    ruleset,
                    author,
                    wrap_edges,
                });
            }
        }

        if let Some(line) = input.next() {
            let line = line?;

            if line.is_empty() {
                continue;
            }

            match &line[..2] {
                "#C" | "#D" => {
                    if description.is_none() {
                        description = Some(String::new());
                    }
                    description = description.map(|desc| desc + line[2..].trim() + "\n");
                }
                "#N" => ruleset = Some(Rules::default()),
                "#R" => ruleset = Some(Rules::from_str(&line[2..].trim())?),
                "#O" => author = Some(line[2..].to_owned()),
                _ => break,
            }
        }
    }

    Ok(PatternConfig {
        ruleset,
        description,
        author,
        wrap_edges,
    })
}

fn parse_life_105_cell_blocks(
    input: impl Iterator<Item = io::Result<String>>,
) -> Result<Vec<Cell>, ParseError> {
    panic!("Not implemented!");
}

fn parse_life_106_list(
    input: impl Iterator<Item = io::Result<String>>,
) -> Result<Vec<Cell>, ParseError> {
    input
        .map(|line| {
            let line = line?;
            let mut coords = line.split(' ');
            let x: u32 = match coords.next() {
                Some(x) => x.parse()?,
                None => {
                    return Err(ParseError::InvalidFormat(format!(
                        "{}: Line contains invalid x position format",
                        line
                    )))
                }
            };
            let y: u32 = match coords.next() {
                Some(y) => y.parse()?,
                None => {
                    return Err(ParseError::InvalidFormat(format!(
                        "{}: Line contains invalid y position format",
                        line
                    )))
                }
            };

            Ok(Cell::new_alive(x, y))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_life_106_glider() {
        let input = "#Life 1.06
0 0
0 1
1 0

1 2
2 0"
        .as_bytes();
        let output = Pattern {
            alive_list: vec![
                Cell::new_alive(0, 0),
                Cell::new_alive(0, 1),
                Cell::new_alive(1, 0),
                Cell::new_alive(1, 2),
                Cell::new_alive(2, 0),
            ],
            config: PatternConfig {
                author: None,
                description: None,
                ruleset: None,
                wrap_edges: false,
            },
            size: Size {
                height: 0,
                width: 0,
            },
        };

        assert_eq!(parse(input).unwrap(), output);
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
.*."
        .as_bytes();
        let output = Pattern {
            alive_list: vec![
                Cell::new_alive(0, 0),
                Cell::new_alive(0, 1),
                Cell::new_alive(0, 2),
                Cell::new_alive(1, 0),
                Cell::new_alive(2, 1),
            ],
            config: PatternConfig {
                author: Some(String::from("John Doe")),
                description: Some(String::from("This is a glider.\nIt's an easy pattern.")),
                ruleset: Some(Rules::default()),
                wrap_edges: false,
            },
            size: Size {
                height: 0,
                width: 0,
            },
        };

        assert_eq!(parse(input).unwrap(), output);
    }
}
