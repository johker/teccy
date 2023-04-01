

pub mod configuration;
pub mod pool;
pub mod terminal;

use self::pool::Pool;

pub fn format(pool: &Pool) -> String {
    String::from("Test1 : 1\nTest2: 2")
}
