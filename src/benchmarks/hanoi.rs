pub struct Hanoi {
    moves: String
}

impl Hanoi {

    pub fn new() -> Hanoi {
        Hanoi {
            moves: String::new()
        }
    }

    pub fn hanoi(&mut self, n: i32, from: &str, via: &str, to: &str) -> &str {
        if n > 0 {
            self.hanoi(n - 1, from, to, via);
            self.moves.push_str(from);
            self.moves.push_str("->");
            self.moves.push_str(to);
            self.moves.push_str("\n");
            self.hanoi(n - 1, via, from, to);
        }
        return &self.moves;
    }
}
