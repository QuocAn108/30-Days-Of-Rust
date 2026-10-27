trait Summarize {
    fn summarize(&self) -> String;
}

struct Book {
    title: String,
    author: String,
}

impl Summarize for Book {
    fn summarize(&self) -> String {
        format!(
            "This Book title is {} created by {}",
            self.title, self.author
        )
    }
}

struct Article {
    title: String,
    publish_date: String,
}

impl Summarize for Article {
    fn summarize(&self) -> String {
        format!(
            "This Article title is {} and publish at {}",
            self.title, self.publish_date
        )
    }
}

pub fn ex1() {
    let book = Book {
        title: String::from("Tieu thuyet An Dep Trai"),
        author: String::from("Quoc An"),
    };
    let article = Article {
        title: String::from("Tieu thuyet An Dep Trai"),
        publish_date: String::from("10-08-2004"),
    };

    println!("{}", book.summarize());
    println!("{}", article.summarize());
}
