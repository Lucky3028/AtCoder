use std::collections::VecDeque;
use proconio::input;

// https://atcoder.jp/contests/joi2022yo2/tasks/joi2022_yo2_a

fn main() {
    input! {
        times: usize,
        operation: [String; times]
    }

    let mut que = VecDeque::new();
    let mut returned = Vec::new();

    operation.iter().for_each(|str| {
        if str == &"READ".to_string() {
            let book = que.pop_front();
            returned.push(book.unwrap());
        } else {
            que.push_front(str);
        }
    });

    returned.iter().for_each(|str| println!("{}", str));
}
