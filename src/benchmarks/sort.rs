use std::cmp::Ordering;

#[derive(Serialize, Deserialize, Debug, Eq, Ord)]
pub struct User {
    pub id: i32,
    pub name: String,
}

impl Clone for User {
    fn clone(&self) -> Self {
        User{
            id: self.id.clone(),
            name: self.name.clone()
        }
    }
}

impl PartialEq for User {
    fn eq(&self, other: &User) -> bool {
        self.id.eq(&other.id) && self.name.eq(&other.name)
    }
}

impl PartialOrd for User {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.name.partial_cmp(&other.name)
    }
}

pub fn sort(data: &mut Vec<User>) -> &Vec<User> {
    data.sort();
    data
}
