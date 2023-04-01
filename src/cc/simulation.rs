use crate::cc::configuration::Configuration;


pub struct Simulation {
    // Reaction rate for cleavage
    k_cl: f32,
    // Reaction rate for condensation
    k_co: f32,
    // Reaction rate for S Reduction 
    k_s: f32,
    // Reaction rate for K Reduction
    k_k: f32,
    // Reaction rate for I Reduction 
    k_i: f32,

}

impl Simulation {
    pub fn run(configuration: &Configuration) {
        let mut react_cnt = 0;
        loop {

            react_cnt += 1;
            if react_cnt > ccc.max_react_cnt {
                break;
            }
        }
    }
}
