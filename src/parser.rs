// Parser error.
#[derive(Debug, PartialEq, Eq)]
pub struct ParserError<'a> {
    at: usize,
    context: &'a str,
    message: &'a str,
}

impl<'a> ParserError<'a> {
    pub fn new(message: &'a str, context: &'a str, at: usize) -> Self {
        Self {
            message: message,
            at: at,
            context: context,
        }
    }
}

impl std::error::Error for ParserError<'_> {}

impl std::fmt::Display for ParserError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_fmt(format_args!("ParserError at {:?}", self.at))
    }
}

// Parser.
pub fn parse(contents: &str) -> Result<Vec<(&str, &str)>, ParserError> {
    let mut parts: Vec<(&str, &str)> = Vec::new();

    #[derive(Debug, PartialEq)]
    enum State {
        Value,
        Identifier,
        Multiline { indent: usize },
        Skip,
    }
    let mut state = State::Identifier;
    let mut indent = 0;
    let mut start = 0;
    let mut key = "";

    let chars = contents.char_indices();
    for (idx, chr) in chars {
        match state {
            State::Value => {
                let rest_of_line = contents[start..idx].trim();

                if let '\n' = chr {
                    if rest_of_line.is_empty() {
                        state = State::Skip;
                    } else {
                        if rest_of_line == "|" {
                            state = State::Multiline { indent };
                            start = idx + 1;
                        } else {
                            return Err(ParserError::new(
                                "Unknown value",
                                &contents[start..idx],
                                idx,
                            ));
                        }
                    }
                    indent = 0;
                }
            }
            State::Multiline {
                indent: current_indent,
            } => {
                if let '\n' = chr {
                    // Peek ahead.
                    let upcoming_line = &contents[(idx + '\n'.len_utf8())..];
                    let mut upcoming_indent = 0;
                    let mut is_empty = false;

                    // Count indent up to first actual char.
                    for chr in upcoming_line.chars() {
                        if let '\n' = chr {
                            // Allow empty lines
                            is_empty = true;
                            break;
                        } else if let ' ' = chr {
                            upcoming_indent += 1;
                        } else {
                            break;
                        }
                    }
                    if !is_empty && upcoming_indent <= current_indent {
                        parts.push((key, &contents[start..idx]));
                        state = State::Skip;
                        indent = 0;
                    }
                }
            }
            State::Identifier => {
                // Skip initial YAML dashes
                if let '-' = chr {
                    start = idx + '-'.len_utf8();
                } else if let ':' = chr {
                    key = &contents[start..idx].trim();
                    state = State::Value;
                    start = idx + ':'.len_utf8();
                }
            }
            State::Skip => {
                if let ' ' = chr {
                    indent += 1;
                } else if !chr.is_whitespace() {
                    state = State::Identifier;
                    start = idx;
                }
            }
        }
    }
    // Ending e.g. without newline
    if state != State::Skip {
        return Err(ParserError::new(
            "Unfinished data. Perhaps missing new line.",
            &contents[start..],
            start,
        ));
    }

    Ok(parts)
}

#[cfg(test)]
mod test {
    use super::parse;

    #[test]
    pub fn basics() {
        let contents = concat!(
            "---\n",
            "Board: |\n",
            "    xxxx\n",
            "    xxxx\n",
            "X: |\n",
            "    x\n",
            "   xxx\n",
            "    x\n",
            "Y: |\n",
            "  xxx\n"
        );
        let map = parse(contents).unwrap();

        assert_eq!(map[0], ("Board", "    xxxx\n    xxxx"));
        assert_eq!(map[1], ("X", "    x\n   xxx\n    x"));
        assert_eq!(map[2], ("Y", "  xxx"));
    }
}
