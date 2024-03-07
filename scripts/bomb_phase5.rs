fn main() {
    let targets = &[15, 0, 5, 11, 13, 1];
    for target in targets {
        for c in b'a'..=b'z' {
            if (c & 0xf) == *target {
                print!("{}", c as char);
                break;
            }
        }
    }
}
