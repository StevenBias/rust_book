use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

pub struct Post {
    state: Option<Box<dyn State>>,
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
        if let Some(s) = &self.state {
            // Add as_str to convert String to &str for push_str method
            self.content.push_str(s.add_text(text).as_str());
        } else {
        }
    }
    
    pub fn content(&self) -> &str {
        // as_ref method to get the reference of the value
        self.state.as_ref().unwrap().content(&self)
    }
    
    pub fn request_review(&mut self) {
        // "take" method the Some value out of the state field and leave a None in its place
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }

    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject())
        }
    }

    pub fn get_state(&self) -> &str {
        type_of(&self.state)
    }
}

trait State {
    fn add_text(&self, text: &str) -> String {
        String::new()
    }
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

impl State for Draft {
    fn add_text(&self, text: &str) -> String {
        String::from(text)
    }

    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview{nb_of_approve: 0})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview{nb_of_approve: 0})
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

struct PendingReview {
    nb_of_approve: u32,
}

impl PendingReview {
    fn check_apporve(&mut self) -> bool {
        if self.nb_of_approve == 0 {
            self.nb_of_approve += 1;
            false
        } else {
            self.nb_of_approve = 0;
            true
        }
    }
}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(mut self: Box<Self>) -> Box<dyn State> {
        if self.check_apporve() {
            Box::new(Published{})
        } else {
            self
        }
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft{})
    }
}
