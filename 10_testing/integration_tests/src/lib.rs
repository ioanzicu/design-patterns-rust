pub mod analysis {
    pub fn count_words(text: &str) -> usize {
        if text.is_empty() {
            return 0;
        }
        text.split_whitespace().count()
    }

    pub fn contains_profanity(text: &str, banned_words: &[&str]) -> bool {
        let lower_text = text.to_lowercase();
        for word in banned_words {
            if lower_text.contains(word) {
                return true;
            }
        }

        false
    }

    pub struct TextStats {
        pub word_count: usize,
        pub character_count: usize,
    }

    pub fn gather_stats(text: &str) -> TextStats {
        TextStats {
            word_count: count_words(text),
            character_count: text.chars().count(),
        }
    }
}
