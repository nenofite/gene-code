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
    let mut pool = gene::Pool::new(100, |g| prog_gene::fitness(|a, b| 3 + a - b*b, g), rng);
    // Print header row
    println!("Generation\tFitness...");
    // Evolve for many generations
    for i in 0 .. 1000 {
        pool.evolve(rng);
        //println!("Iter {} best: {}", i, pool.get_best());
        // Print generation
        println!("{}", i);
        // Print the fitness of each gene
        //for g in &pool.genes {
        //    print!("\t{}", g.1);
        //}
        //println!();
    }
    for g in &pool.genes {
        println!("{}", g.0);
    }
    // Print the best gene
    let best = pool.get_best();
    println!("Best ({}): {}", best.1, best.0);
}
