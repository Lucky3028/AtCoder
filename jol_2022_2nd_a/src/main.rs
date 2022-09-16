use proconio::*;

#[derive(fastout)]
fn main() {
    input! {
        n: usize,
        s: [String; n]
    }
    let mut q = vec![];
    s.iter().for_each(|str| {
        if str == &"READ".to_string() {
            let book = q.pop();
            println!("{}", book.unwrap());
        } else {
            q.push(str);
        }
    });
}
