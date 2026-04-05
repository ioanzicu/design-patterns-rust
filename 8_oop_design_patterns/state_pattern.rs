struct DraftPost {
    content: String,
}

struct PendingReviewPost {
    content: String,
}

struct PublishedPost {
    content: String,
}

enum PostState {
    Draft(DraftPost),
    PendingReview(PendingReviewPost),
    Published(PublishedPost),
}

pub struct Post {
    state: PostState,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: PostState::Draft(DraftPost {
                content: String::new(),
            }),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        if let PostState::Draft(ref mut draft) = self.state {
            draft.content.push_str(text);
        } else {
            println!("Connot add text in current state.");
        }
    }

    pub fn request_review(&mut self) {
        if let PostState::Draft(draft) = std::mem::replace(
            &mut self.state,
            PostState::Draft(DraftPost {
                content: String::new(),
            }),
        ) {
            self.state = PostState::PendingReview(PendingReviewPost {
                content: draft.content,
            });
        } else {
            println!("Post must be in Draft state to request review.");
        }
    }

    pub fn approve(&mut self) {
        if let PostState::PendingReview(pending) = std::mem::replace(
            &mut self.state,
            PostState::Draft(DraftPost {
                content: String::new(),
            }),
        ) {
            self.state = PostState::Published(PublishedPost {
                content: pending.content,
            })
        } else {
            println!("Post must be Pending Review to approve.");
        }
    }

    pub fn content(&self) -> &str {
        match &self.state {
            PostState::Draft(s) => &s.content,
            PostState::PendingReview(s) => &s.content,
            PostState::Published(s) => &s.content,
        }
    }
}

fn main() {
    let mut post = Post::new();
    post.add_text("Learning about state patterns in Rust.");
    println!("Content (Draft): {}", post.content());

    post.request_review();
    post.add_text("This won't be added."); // Text won't be added - wrong state
    post.approve();
    println!("Content (Published): {}", post.content());

    post.request_review(); // Invalid transition
}
