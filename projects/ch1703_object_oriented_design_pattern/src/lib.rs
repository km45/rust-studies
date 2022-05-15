pub mod StatePattern {
    pub struct Post {
        state: Option<Box<State>>,
        content: String,
    }

    impl Post {
        pub fn new() -> Post {
            Post {
                state: Some(Box::new(Draft {})),
                content: String::new(),
            }
        }

        pub fn add_text(&mut self, text: &str) {
            self.content.push_str(text);
        }

        pub fn content(&self) -> &str {
            self.state.as_ref().unwrap().content(&self)
        }

        pub fn request_review(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.request_review());
            }
        }

        pub fn approve(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.approve());
            }
        }
    }

    trait State {
        // Take ownership of the state value to consume the old state.
        fn request_review(self: Box<Self>) -> Box<State>;
        fn approve(self: Box<Self>) -> Box<State>;

        fn content<'a>(&self, post: &'a Post) -> &'a str {
            ""
        }
    }

    struct Draft {}

    impl State for Draft {
        fn request_review(self: Box<Self>) -> Box<State> {
            Box::new(PendingReview {})
        }

        fn approve(self: Box<Self>) -> Box<State> {
            self
        }
    }

    struct PendingReview {}

    impl State for PendingReview {
        fn request_review(self: Box<Self>) -> Box<State> {
            self
        }

        fn approve(self: Box<Self>) -> Box<State> {
            Box::new(Published {})
        }
    }

    struct Published {}

    impl State for Published {
        fn request_review(self: Box<Self>) -> Box<State> {
            self
        }

        fn approve(self: Box<Self>) -> Box<State> {
            self
        }

        fn content<'a>(&self, post: &'a Post) -> &'a str {
            &post.content
        }
    }
}

pub mod TypedState {
    pub struct Post {
        content: String,
    }

    pub struct DraftPost {
        content: String,
    }

    pub struct PendingReviewPost {
        content: String,
    }

    impl Post {
        pub fn new() -> DraftPost {
            DraftPost {
                content: String::new(),
            }
        }

        pub fn content(&self) -> &str {
            &self.content
        }
    }

    impl DraftPost {
        pub fn add_text(&mut self, text: &str) {
            self.content.push_str(text);
        }

        pub fn request_review(self) -> PendingReviewPost {
            PendingReviewPost {
                content: self.content,
            }
        }
    }

    impl PendingReviewPost {
        pub fn approve(self) -> Post {
            Post {
                content: self.content,
            }
        }
    }
}
