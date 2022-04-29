use amen::create_layout;
use amen::run_amen;

#[test]
fn single_alt_shorted() {
    assert_eq!(run(&["Long Johnsson"], b""), Ok("Long Johnsson"));
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

fn run<'a>(phrases: &'a [&str], key_input: &[u8]) -> Result<&'a str, String> {
    let layout = create_layout(phrases, (100, 100)).map_err(|e| e.to_string())?;
    run_amen(key_input, vec![], phrases, &layout).map_err(|e| e.to_string())
}
