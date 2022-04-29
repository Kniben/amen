use std::collections::HashMap;
use termion::cursor::Goto;

#[derive(Debug)]
pub struct Layout<'a> {
    pub column_width: u16,
    pub size: (u16, u16),
    text_positions: HashMap<&'a str, (u16, u16)>,
}

impl<'a> Layout<'a> {
    pub fn new(texts: &[&'a str], term_size: (u16, u16)) -> Self {
        let spacing = 2;
        let column_width = spacing + texts.into_iter().map(|text| text.len()).max().unwrap() as u16;
        let size = compute_size(texts.len(), column_width, term_size);
        let text_positions = compute_text_positions(texts, column_width, size);
        Self {
            column_width,
            size,
            text_positions,
        }
    }

    pub fn clear(&self) -> String {
        if let Some(top) = self.text_positions.values().map(|(_, y)| y).min() {
            let bot = self.text_positions.values().map(|(_, y)| y).max().unwrap();
            (*top..=*bot)
                .map(|i| {
                    format!(
                        "{}{}",
                        termion::cursor::Goto(1, i),
                        termion::clear::UntilNewline
                    )
                })
                .collect()
        } else {
            "".to_string()
        }
    }

    pub fn offset(&mut self, offset: (u16, u16)) {
        for text_pos in self.text_positions.values_mut() {
            text_pos.0 += offset.0;
            text_pos.1 += offset.1;
        }
    }

    pub fn goto_text_position(&self, text: &str) -> Goto {
        let pos = self.text_positions.get(text).unwrap();
        termion::cursor::Goto(pos.0, pos.1)
    }
}

fn compute_size(text_count: usize, column_width: u16, term_size: (u16, u16)) -> (u16, u16) {
    let column_count = term_size.0 / column_width;
    let row_count = 1 + text_count as u16 / column_count;
    (column_count, row_count)
}

fn compute_text_positions<'a>(
    texts: &[&'a str],
    column_width: u16,
    size: (u16, u16),
) -> HashMap<&'a str, (u16, u16)> {
    // TODO Shorten texts that are too long
    // TODO Only display part of texts when too many to fit half a screen

    let mut text_positions = HashMap::new();
    let mut texts_iter = texts.into_iter();
    for x in 0..size.0 {
        for y in 0..size.1 {
            if let Some(next) = texts_iter.next() {
                text_positions.insert(*next, (1 + x * column_width, 1 + y));
            } else {
                return text_positions;
            }
        }
    }
    text_positions
}
