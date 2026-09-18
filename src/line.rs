
use crate::Error;

#[derive(Copy, Clone)]
pub struct LmnoLine<'s> {
    pub indent: usize,
    pub text: &'s str,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum WhitespaceType {
    Spaces,
    Tabs,
}

pub fn check_whitespace(current: &mut Option<WhitespaceType>, found: WhitespaceType) -> Result<(), Error> {
    match current {
        Some(current) => (*current == found).then_some(()).ok_or(Error::MixedWhitespace),
        None => {
            *current = Some(found);
            Ok(())
        },
    }
}

pub struct LmnoLines<'s> {
    whitespace_type: Option<WhitespaceType>,
    lines: std::str::Lines<'s>,
}

impl<'s> LmnoLines<'s> {
    pub fn new(str: &'s str) -> Self {
        Self {
            whitespace_type: None,
            lines: str.lines(),
        }
    }
}

impl<'s> Iterator for LmnoLines<'s> {
    type Item = Result<LmnoLine<'s>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_transposed().transpose()
    }
}

impl<'s> LmnoLines<'s> {
    pub fn next_transposed(&mut self) -> Result<Option<LmnoLine<'s>>, Error> {
        for line in self.lines.by_ref() {
            let mut indent = 0;
            for char in line.chars() {
                match char {
                    '\t' => check_whitespace(&mut self.whitespace_type, WhitespaceType::Tabs)?,
                    ' ' => check_whitespace(&mut self.whitespace_type, WhitespaceType::Spaces)?,
                    _ => break,
                }
                indent += 1;
            }
            let text = line.trim();
            if text.is_empty() || text.starts_with('#') { continue }
            return Ok(Some(LmnoLine { indent, text }))
        }
        Ok(None)
    }
}
