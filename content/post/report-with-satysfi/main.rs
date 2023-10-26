fn main() {
    let s = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s.trim_end().to_owned()
    };
    let n: i32 = s.parse().unwrap();
    println!(
        "{}",
        if n % 3 == 0 || s.contains('3') {
            "aho".to_owned()
        } else {
            s
        }
    );
}
