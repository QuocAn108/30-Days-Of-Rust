mod book;
mod rectangle;
fn main() {
    let book = book::Book {
        title: String::from("Sach Ti Phu"),
        author: String::from("An Dep Trai"),
        pages: 100,
        publisher: String::from("Bao Quoc Gia"),
    };
    book.display();
    book.calculate();

    let rectangle = rectangle::Rectangle {
        width: 10.0,
        length: 20.0,
    };
    println!("Area Of Rectangle: {}", rectangle.cal_area());
    println!("Perimeter Of Rectangle: {}", rectangle.cal_perimeter());
    let color1 = RGB(255, 0, 0);
    let color2 = RGB(255, 0, 0);
    if color1.is_equal(&color2) {
        println!("The colors are equal.");
    } else {
        println!("The colors are not equal.");
    }
}

struct RGB(i32, i32, i32);
impl RGB {
    fn is_equal(&self, other: &RGB) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2
    }
}
fn trigger_check() {
    println!("Signal triggered!");
    IsEqual;
}
struct IsEqual;
