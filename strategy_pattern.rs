trait TextFormattingStrategy {
    fn format(&self, text: &str) -> String;
}

struct UpperCaseFormatter;
impl TextFormattingStrategy for UpperCaseFormatter {
    fn format(&self, text: &str) -> String {
        text.to_uppercase()
    }
}

struct LowerCaseFormatter;
impl TextFormattingStrategy for LowerCaseFormatter {
    fn format(&self, text: &str) -> String {
        text.to_lowercase()
    }
}

struct TextProcessor {
    strategy: Box<dyn TextFormattingStrategy>,
}

impl TextProcessor {
    fn new(strategy: Box<dyn TextFormattingStrategy>) -> Self {
        TextProcessor { strategy }
    }

    fn set_strategy(&mut self, strategy: Box<dyn TextFormattingStrategy>) {
        self.strategy = strategy;
    }

    fn process(&self, text: &str) -> String {
        self.strategy.format(text)
    }
}

fn main() {
    let mut processor = TextProcessor::new(Box::new(UpperCaseFormatter));
    let text = "Hello, Strategy Pattern!";

    let uppercase_result = processor.process(text);
    println!("Using uppercase strategy: {}", uppercase_result);

    processor.set_strategy(Box::new(LowerCaseFormatter));
    let lowercase_result = processor.process(text);
    println!("Using lowercase strategy: {}", lowercase_result);
}
