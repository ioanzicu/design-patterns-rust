use std::rc::Rc;

// 1. Document, TextViewer and WordCount struct
#[derive(Debug)]
struct Document {
    content: String,
}

struct TextViewer {
    document: Rc<Document>,
}

struct WordCounter {
    document: Rc<Document>,
}

fn main() {
    // 2. Create and shared document
    let document = Document {
        content: "Tabula rasa...".to_string(),
    };
    let document_ref = Rc::<Document>::new(document);

    println!(
        "Initial strong_reference count: {:?}.",
        Rc::<Document>::strong_count(&document_ref)
    ); // 1

    // 3. Create "Viewers"
    let text_viewer = TextViewer {
        document: Rc::clone(&document_ref),
    };

    let word_counter = WordCounter {
        document: Rc::clone(&document_ref),
    };

    println!(
        "Current strong_reference count: {:?}.",
        Rc::<Document>::strong_count(&document_ref)
    ); // 3

    // 4. Access the shared data
    // Both print "Tabula rasa..." since is the same content
    println!(
        "TextViewer document content: {:?}\nlen: {}",
        text_viewer.document.content,
        text_viewer.document.content.len()
    );

    println!(
        "WordCounter document content: {:?}\nlen: {}",
        word_counter.document.content,
        word_counter.document.content.len()
    );

    // 5. Observe cleanup (optional)
    drop(text_viewer.document);

    println!(
        "strong_reference count after drop(): {:?}.",
        Rc::<Document>::strong_count(&document_ref)
    ); // 2
}
