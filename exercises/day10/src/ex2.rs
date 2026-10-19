use std::fmt::Display;
use std::mem;

//  Cách 1: Hoán đổi trực tiếp giá trị của 2 biến bằng Clone
// pub fn my_swap<T: Clone>(a: &mut T, b: &mut T) {
//     let temp = a.clone();
//     *a = b.clone();
//     *b = temp;
// }
//
//  Cách 2: Nhận giá trị vào và trả về tuple đã đảo vị trí (chuyển quyền sở hữu, không cần Clone)
// pub fn swap_values<T>(a: T, b: T) -> (T, T) {
//     (b, a)
// }
pub fn swap_variables<T>(a: &mut T, b: &mut T) {
    mem::swap(a, b);
}

#[derive(Debug)]
pub enum CustomResult<T, E> {
    Ok(T),
    Err(E),
}

fn divide(a: f64, b: f64) -> CustomResult<f64, String> {
    if b == 0.0 {
        CustomResult::Err(String::from("Cannot divide by zero"))
    } else {
        CustomResult::Ok(a / b)
    }
}

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct Post<T> {
    pub title: String,
    pub author: String,
    pub content: T,
}

impl<T: Display> Summary for Post<T> {
    fn summarize(&self) -> String {
        format!(
            "Bài viết '{}' của {}: {}",
            self.title, self.author, self.content
        )
    }
}
pub fn ex2() {
    let mut x = 10;
    let mut y = 20;

    println!("Trước khi swap: x = {}, y = {}", x, y);
    swap_variables(&mut x, &mut y);
    println!("Sau khi swap: x = {}, y = {}", x, y);

    let mut s1 = String::from("Rust");
    let mut s2 = String::from("Generics");
    swap_variables(&mut s1, &mut s2);
    println!("Sau khi swap chuỗi: s1 = {}, s2 = {}", s1, s2);

    println!("\n--- 2. Test CustomResult<T, E> ---");
    let res_ok = divide(10.0, 2.0);
    let res_err = divide(10.0, 0.0);

    match res_ok {
        CustomResult::Ok(val) => println!("Kết quả phép chia: {}", val),
        CustomResult::Err(err) => println!("Lỗi: {}", err),
    }

    match res_err {
        CustomResult::Ok(val) => println!("Kết quả phép chia: {}", val),
        CustomResult::Err(err) => println!("Lỗi: {}", err),
    }

    println!("\n--- 3. Test Trait Summary ---");
    let post_text = Post {
        title: String::from("Rust Generics"),
        author: String::from("An"),
        content: "Hiểu sâu về trait bounds và monomorphization",
    };
    println!("{}", post_text.summarize());

    let post_number = Post {
        title: String::from("Điểm đánh giá"),
        author: String::from("Reviewer"),
        content: 100,
    };
    println!("{}", post_number.summarize());
}
