use std::collections::HashMap;
use termion::cursor::Goto;

#[derive(Debug)]
pub struct Layout<'a> {
    pub column_width: u16,
    pub size: (u16, u16),
    text_positions: HashMap<&'a str, (u16, u16)>,
}

impl<'a> Layout<'a> {
    pub fn new() -> Self {
        Self {
            column_width: 0,
            size: (0, 0),
            text_positions: HashMap::new(),
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

    pub fn update(
        &mut self,
        texts: &[&'a str],
        texts_count: usize,
        term_size: (u16, u16),
    ) -> Result<(), std::io::Error> {
        let spacing = 2;
        self.column_width =
            spacing + texts.into_iter().map(|text| text.len()).max().unwrap() as u16;
        self.set_size(texts_count, term_size);
        self.set_text_positions(texts);
        Ok(())
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

    fn set_size(&mut self, text_count: usize, term_size: (u16, u16)) {
        let column_count = term_size.0 / self.column_width;
        let row_count = 1 + text_count as u16 / column_count;
        self.size = (column_count, row_count);
    }

    fn set_text_positions(&mut self, texts: &[&'a str]) {
        // TODO Shorten texts that are too long
        // TODO Only display part of texts when too many to fit half a screen

        let mut texts_iter = texts.clone().into_iter();
        self.text_positions.clear();
        for x in 0..self.size.0 {
            for y in 0..self.size.1 {
                if let Some(next) = texts_iter.next() {
                    self.text_positions.insert(next, (x * self.column_width, y));
                } else {
                    return;
                }
            }
        }
    }
}
