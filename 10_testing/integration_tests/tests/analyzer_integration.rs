// The name `integration_tests` must match the `name` field in Cargo.toml
mod common;
use integration_tests;

#[test]
fn test_word_count_integration() {
    let sample_text = "This is a sample sentence.";

    // Call the public function from out library's public module
    let count = integration_tests::analysis::count_words(sample_text);
    assert_eq!(count, 5, "Word count should be 5");
}

#[test]
fn test_profanity_checker_integration() {
    let sample_text_clean = "A lovely day for a walk.";
    let sample_text_profane = "This is a darn naughty sentence.";

    let banned = ["darn", "naughty"];

    assert!(
        !integration_tests::analysis::contains_profanity(sample_text_clean, &banned),
        "Clean text should not contain profanity"
    );

    assert!(
        integration_tests::analysis::contains_profanity(sample_text_profane, &banned),
        "Profane text should be detected"
    );
}

#[test]
fn text_gather_stats_integration() {
    let sample_text = "Hello world!"; // 2 words, 12 chars
    let stats = integration_tests::analysis::gather_stats(sample_text);
    assert_eq!(stats.word_count, 2);
    assert_eq!(stats.character_count, 12);

    // public struct fields returned by public functions also can be accessed
}

#[test]
fn test_empty_string_stats() {
    let stats = integration_tests::analysis::gather_stats("");
    assert_eq!(stats.word_count, 0);
    assert_eq!(stats.character_count, 0);
}

#[test]
fn test_long_text_word_count() {
    let long_text = common::create_sample_long_text();
    let count = integration_tests::analysis::count_words(&long_text);

    assert!(
        count > 10,
        "Expected more than 10 words in the long sample text"
    );
}

#[test]
fn test_profanity_with_common_list() {
    let text_with_profanity = "Oh heck, this is not good.";
    let banned_list = common::common_banned_word_list();
    assert!(integration_tests::analysis::contains_profanity(
        text_with_profanity,
        &banned_list
    ));
}
