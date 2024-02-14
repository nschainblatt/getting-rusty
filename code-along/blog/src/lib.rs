enum ReviewStatus {
    UnApproved,
    InReview,
    Approved,
}

pub struct Post {
    text: String,
    content: String,
    status: ReviewStatus,
}

impl Post {
    pub fn new() -> Post {
        Post {
            text: String::from(""),
            content: String::from(""),
            status: ReviewStatus::UnApproved,
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.text = String::from(text);
    }

    pub fn request_review(&mut self) {
        self.status = ReviewStatus::InReview;
    }

    pub fn approve(&mut self) {
        self.status = ReviewStatus::Approved;
        self.content = self.text.to_string();
    }

    pub fn content(&mut self) -> String {
        self.content.to_string()
    }
}
