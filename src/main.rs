mod generation;
mod utils;

use rayon::prelude::*;

fn main() {
    let seeds: Vec<u32> = (0..100).collect();
    seeds.par_iter().for_each(|i| {
        generation::generate_planet(*i);
    });
}
