//! ## Run
//! cargo r --bin quiz-4

fn main() {
    let (.., x, y) = (0, 1, ..);
    print!("{}", b"066"[y][x]); // 54 (此时 x=1, y=.. , 所以取到了中间的那个'6')
}
