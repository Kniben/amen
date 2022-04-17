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

    // TODO Really need to clone?
    pub fn update<I: IntoIterator<Item = &'a str> + Clone>(
        &mut self,
        texts: &I,
        texts_count: usize,
        origin: (u16, u16),
        term_size: (u16, u16),
    ) -> Result<(), std::io::Error> {
        let spacing = 1;
        self.column_width = spacing
            + texts
                .clone()
                .into_iter()
                .map(|text| text.len())
                .max()
                .unwrap() as u16;
        self.set_size(texts_count, term_size);
        self.set_text_positions(texts, origin);
        Ok(())
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

    // TODO Really need to clone?
    fn set_text_positions<I: IntoIterator<Item = &'a str> + Clone>(
        &mut self,
        texts: &I,
        origin: (u16, u16),
    ) {
        // TODO Shorten texts that are too long
        // TODO Only display part of texts when too many to fit half a screen

        let mut texts_iter = texts.clone().into_iter();
        self.text_positions.clear();
        for x in 0..self.size.0 {
            for y in 0..self.size.1 {
                if let Some(next) = texts_iter.next() {
                    self.text_positions
                        .insert(next, (origin.0 + x * self.column_width, origin.1 + y));
                } else {
                    return;
                }
            }
        }
    }
}
