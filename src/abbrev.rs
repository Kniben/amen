use std::collections::HashMap;
use trie_rs::TrieBuilder;

pub struct AbbrevString {
    pub original: String,
    pub abbrev: String,
    pub indices: Vec<usize>,
}

pub fn assign_abbrevs<'a>(all_options: Vec<String>) -> HashMap<String, AbbrevString> {
    let all_options_trie = copy_to_trie(&all_options);
    let mut items_by_abbrev = HashMap::new();
    let count = all_options.len();

    for original in all_options {
        let (abbrev, indices) = assign_abbrev_indices(&original, &all_options_trie, count);
        items_by_abbrev.insert(
            abbrev.clone(),
            AbbrevString {
                original,
                abbrev,
                indices,
            },
        );
    }
    items_by_abbrev
}

fn copy_to_trie(items: &Vec<String>) -> trie_rs::Trie<u8> {
    let mut trie_builder = TrieBuilder::new();
    for item in items.clone() {
        trie_builder.push(item);
    }
    trie_builder.build()
}

fn assign_abbrev_indices(
    text: &String,
    all_texts: &trie_rs::Trie<u8>,
    count: usize,
) -> (String, Vec<usize>) {
    let mut abbrev = String::new();
    let mut indices = vec![];
    let mut prev_matches = count;
    for i in 0..text.len() {
        let query = &text[..=i];
        let search = all_texts.predictive_search(query);
        let matches: Vec<_> = search
            .iter()
            .map(|u8s| std::str::from_utf8(u8s).unwrap())
            .collect();

        if matches.len() < prev_matches {
            use std::fmt::Write;
            indices.push(i);
            write!(abbrev, "{}", &text[i..(i + 1)]).unwrap();
        } else if matches.len() == 0 {
            break;
        }

        prev_matches = matches.len();
    }
    (abbrev, indices)
}

#[cfg(test)]
mod test {
    #[test]
    fn test1() {}
}
