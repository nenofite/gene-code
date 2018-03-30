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
    let mut pool = gene::Pool::new(1000, rng);
    // Print header row
    println!("Generation\tFitness...");
    // Evolve for many generations
    for i in 0 .. 1000 {
        pool.evolve(|g| prog_gene::fitness(|a, b| a*a + b*b, g), rng);
        //println!("Iter {} best: {}", i, pool.get_best());
        // Print generation
        print!("{}", i);
        // Print the fitness of each gene
        for g in &pool.genes {
            print!("\t{}", g.1);
        }
        println!();
    }
    // Print the best gene
    println!("Best: {}", pool.get_best());
}
