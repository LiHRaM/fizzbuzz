#[derive(Debug)]
enum Message {
    Fizz,
    Buzz,
    FizzBuzz,
    Num(i32),
}

fn main() {
    for i in 1..100 {
        let item = match ((i % 5), (i % 3)) {
            (0, 0) => Message::FizzBuzz,
            (_, 0) => Message::Fizz,
            (0, _) => Message::Buzz,
            _ => Message::Num(i),
        };
        println!("{item:?}");
    }
}
