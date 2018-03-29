//
// Fit stack-based programs into the gene interface, so we can generate and mutate them
//

use super::lang;
use super::gene;

use std::fmt::Debug;
use rand::Rng;

// A program as a gene. This is a simple wrapper so we can implement the required trait.
#[derive(Clone, Debug)]
pub struct ProgramGene(pub Vec<lang::Prog>);

// Generate a random number or command
fn rand_prog<R: Rng>(rng: &mut R) -> lang::Prog {
    if rng.gen() {
        // 50% chance of number
        lang::Prog::D(rng.gen_range(-10, 11))
    } else {
        // 50% chance of command
        let cmd = match rng.gen_range(0, 5) {
            0 => lang::Command::Add,
            1 => lang::Command::Sub,
            2 => lang::Command::Mult,
            3 => lang::Command::Div,
            _ => lang::Command::Dup,
        };
        lang::Prog::C(cmd)
    }
}

impl gene::Gene for ProgramGene {
    fn generate<R: Rng>(rng: &mut R) -> Self {
        // Generate a random sequence of numbers & commands
        // Pick a length between 1 and 10
        let len: usize = rng.gen_range(1, 11);
        // Fill a vec with progs
        let mut prog = Vec::new();
        for _ in 0 .. len {
            prog.push(rand_prog(rng));
        }
        ProgramGene(prog)
    }

    fn mutate<R: Rng>(&self, rng: &mut R) -> Self {
        // Add, delete, or replace a random prog
        let mut result = self.0.clone();
        match rng.gen_range(0, 3) {
            0 => {
                // Add
                let prog = rand_prog(rng);
                let i = rng.gen_range(0, result.len()+1);
                result.insert(i, prog);
            }
            1 => {
                // Delete
                if result.len() > 0 {
                    let i = rng.gen_range(0, result.len());
                    result.remove(i);
                }
            }
            _ => {
                // Replace
                if result.len() > 0 {
                    let prog = rand_prog(rng);
                    let i = rng.gen_range(0, result.len());
                    result[i] = prog;
                }
            }
        }
        ProgramGene(result)
    }
}

// Use to create a fitness function that runs the program and compares output to the given reference
// function. Also gives a slight bonus to shorter programs.
pub fn fitness<F: Fn(i32, i32) -> i32>(f: F, g: &ProgramGene) -> f32 {
    let mut total = 0;
    let mut successful = 0;
    // Iterate through the test cases
    for a in 0 .. 10 {
        for b in 0 .. 10 {
            // Create a stack
            let mut s = lang::Stack::new();
            // Add the inputs
            s.push(a);
            s.push(b);
            // Run the program
            s.queue_program(&g.0);
            s.run_until(100);
            // Compare the output
            let result = s.pop();
            if result == f(a, b) {
                successful += 1;
            }
            total += 1;
        }
    }
    // Fitness is successful / total test cases
    let correctness = successful as f32 / total as f32;
    let shortness = 1.0 - (g.0.len() as f32 / 100.0);
    0.99 * correctness + 0.01 * shortness
}

#[cfg(test)]
mod tests {
    use super::*;
    use gene::Gene;
    use ::rand::SeedableRng;

    #[test]
    fn generate_and_mutate() {
        let rng = &mut ::rand::StdRng::from_seed(&[123]);
        // Generate some random genes
        let mut genes: Vec<ProgramGene> = Vec::new();
        for _ in 0 .. 1000 {
            genes.push(gene::Gene::generate(rng));
        }

        // Mutate them
        for g in genes {
            g.mutate(rng).mutate(rng).mutate(rng);
        }
    }

    #[test]
    fn test_fitness() {
        let eps = 0.000001;
        // Test that the program returns a + b
        let good_prog = ProgramGene(vec![lang::Prog::C(lang::Command::Add)]);
        assert!((fitness(|a,b| a + b, &good_prog) - 0.9999).abs() < eps);

        // Test that the program returns a + b, with a longer program (less fit)
        let okay_prog = ProgramGene(vec![lang::Prog::C(lang::Command::Add), lang::Prog::C(lang::Command::Dup), lang::Prog::C(lang::Command::Dup), lang::Prog::C(lang::Command::Dup), lang::Prog::C(lang::Command::Dup)]);
        assert!((fitness(|a,b| a + b, &okay_prog) - 0.9995).abs() < eps);

        // Test program that always returns -1
        let bad_prog = ProgramGene(vec![lang::Prog::D(-1)]);
        assert!((fitness(|a,b| a + b, &bad_prog) - 0.0099).abs() < eps);
    }
}
