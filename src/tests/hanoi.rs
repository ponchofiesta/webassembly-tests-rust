//use std::borrow::Cow;

static mut MOVES: &'static String = &String::new();

pub struct Hanoi {
    moves: String
}

impl Hanoi {
    fn hanoi(&mut self, n: i32, from: &str, via: &str, to: &str) -> String {
        if n > 0 {
            self.hanoi(n - 1, from, to, via);
            self.moves.push_str(from);
            self.moves.push_str("->");
            self.moves.push_str(to);
            self.moves.push_str("\n");
            self.hanoi(n - 1, via, from, to);
        }
        return self.moves.into();
    }
}


//pub fn hanoi<'a>(n: i32, from: &str, via: &str, to: &str, movescow: &'a str) -> Cow<'a, str> {
//    if n > 0 {
//        let mut moves = String::from(hanoi(n - 1, from, to, via, moves));
//        moves.push_str(from);
//        moves.push_str("->");
//        moves.push_str(to);
//        moves.push_str("\n");
//        let moves = String::from(hanoi(n - 1, via, from, to, &moves));
//        return Cow::Owned(moves);
//    }
//    return Cow::Borrowed(moves);
//}
