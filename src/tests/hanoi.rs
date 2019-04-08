use std::borrow::Cow;

pub fn hanoi<'a>(n: i32, from: &str, via: &str, to: &str, moves: &'a str) -> Cow<'a, str> {
    if n > 0 {
        let mut moves = String::from(hanoi(n - 1, from, to, via, moves));
        moves.push_str(from);
        moves.push_str("->");
        moves.push_str(to);
        moves.push_str("\n");
        let moves = String::from(hanoi(n - 1, via, from, to, &moves));
        return Cow::Owned(moves);
    }
    return Cow::Borrowed(moves);
}
