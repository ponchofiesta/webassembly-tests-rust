use std::sync::Mutex;
use tests::sort::User;

lazy_static! {
    pub static ref DATA_SORT_BASE: Mutex<Vec<User>> = Mutex::new(vec![]);
    pub static ref DATA_SORT: Mutex<Vec<User>> = Mutex::new(vec![]);
    pub static ref DATA_AES_BASE: Mutex<Vec<u8>> = Mutex::new(vec![]);
    pub static ref DATA_AES: Mutex<Vec<Vec<u8>>> = Mutex::new(vec![]);
}
