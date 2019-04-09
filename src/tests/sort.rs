#[derive(Serialize, Deserialize, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct User {
    pub id: i32,
    pub name: String,
}

pub fn sort(data: &mut Vec<User>) -> &Vec<User> {
    data.sort_by(|left, right| left.cmp(&right));
    data
}
