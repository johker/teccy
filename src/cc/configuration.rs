
pub struct Configuration {
    // The maximum number of reactions to take place
    pub max_react_cnt: u32,
    // Initial Number of I combinators
    pub ni0: u32,
    // Initial Number of K combinators
    pub nk0: u32,
    // Initial Number of S combinators
    pub ns0: u32,

}

impl Configuration {
    pub fn new() -> Self {
        Self {
            max_react_cnt: 100,
            ni0: 1000,
            nk0: 1000,
            ns0: 1000,
        }
    }
}
