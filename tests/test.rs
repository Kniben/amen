use amen::{layout::Layout, run_amen};

#[test]
fn multi_alts() {
    assert_eq!(run(&["foo", "pho"], b"p"), Ok("pho"));
    assert_eq!(run(&["police", "potato"], b"t"), Ok("potato"));
    assert_eq!(run(&["pan", "police", "potato"], b"a"), Ok("pan"));
}

#[test]
fn duplicates() {
    assert_eq!(run(&["paper", "car", "car"], b"c"), Ok("car"));
}

#[test]
fn prefix() {
    assert_eq!(run(&["santa", "santa claus"], b"sa"), Ok("santa"));
}

#[test]
fn suffix() {
    assert_eq!(run(&["santa", "santa claus"], b"sc"), Ok("santa claus"));
}

// TODO Implement this feature
// #[test]
// fn duplicate_phrases_shorted() {
//     assert_eq!(run(&["pan", "pan"], b""), Ok("pan"));
// }

// TODO Implement this feature
// #[test]
// fn case_insensitive_user_input() {
//     assert_eq!(run(&["Bear", "cow"], b"b"), Ok("Bear"));
//     assert_eq!(run(&["eRROR", "Relax"], b"r"), Ok("Relax"));
// }

fn run<'a>(phrases: &'a [&str], key_input: &[u8]) -> Result<&'a str, String> {
    let layout = {
        let term_size = (100, 100);
        Layout::new(phrases, term_size)
    };
    run_amen(key_input, vec![], phrases, &layout).map_err(|e| e.to_string())
}
