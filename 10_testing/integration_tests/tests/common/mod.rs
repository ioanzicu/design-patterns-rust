pub fn create_sample_long_text() -> String {
    "This is a very long string that we might want to use in multiple integration tests for various analysis purposes. It contains several words and pubcuration marks like commas, and even exclamation points!".to_string()
}

pub fn common_banned_word_list() -> Vec<&'static str> {
    vec!["heck", "darn", "gosh"]
}
