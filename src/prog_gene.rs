//
// Fit stack-based programs into the gene interface, so we can generate and mutate them
//

use super::lang;
use super::gene;

use rand::Rng;

// A program as a gene. This is a simple wrapper so we can implement the required trait.
pub struct ProgramGene(Vec<lang::Prog>);

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
}
