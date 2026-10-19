mod ex;
mod ex1;
mod ex2;
fn main() {
    let pair = ex::Pair {
        first: "Rust",
        second: 101,
    };
    pair.show();
    ex1::ex1();
    ex2::ex2();
}
