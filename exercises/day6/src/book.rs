pub struct Book {
    pub title: String,
    pub author: String,
    pub pages: i32,
    pub publisher: String,
}

impl Book {
    pub fn display(&self) {
        println!("----Book Details----");
        println!("Title: {}", self.title);
        println!("Author: {}", self.author);
        println!("Publisher: {}", self.publisher);
    }
    pub fn calculate(&self) {
        println!("Total Pages: {}", self.pages);
    }
}
