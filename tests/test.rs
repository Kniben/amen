use amen::run_amen;

#[test]
fn single_alt_shorted() {
    assert_eq!(
        run(b"", &["Long Johnsson"]),
        Ok("Long Johnsson".to_string())
    );
}

#[test]
fn multi_alts() {
    assert_eq!(run(b"p", &["foo", "pho"]), Ok("pho".to_string()));
    assert_eq!(run(b"t", &["police", "potato"]), Ok("potato".to_string()));
    assert_eq!(
        run(b"a", &["pan", "police", "potato"]),
        Ok("pan".to_string())
    );
}

fn run(key_input: &[u8], phrase_input: &[&str]) -> Result<String, String> {
    let mut output = vec![];
    let phrases = phrase_input
        .iter()
        .map(|phrase| phrase.to_string())
        .collect();
    Ok(run_amen(key_input, &mut output, phrases).map_err(|e| e.to_string())?)
}
