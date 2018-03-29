//
// Evolve stack-based programs
//

extern crate rand;

mod lang;
mod gene;
mod prog_gene;

// Evolve programs to solve addition, then print out the winners.
pub fn main() {
    // Make a pool
    let rng = &mut rand::thread_rng();
    let mut pool = gene::Pool::new(100, rng);
    // Evolve for many generations
    for i in 0 .. 1000 {
        pool.evolve(|g| prog_gene::fitness(|a, b| a + b + 32, g), rng);
        println!("Iter {} best: {:?}", i, pool.get_best());
    }
    // Print the best gene
    println!("Best: {:?}", pool.get_best());
}
