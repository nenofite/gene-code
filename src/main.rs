//
// Evolve stack-based programs
//

extern crate rand;

mod lang;
mod gene;
mod prog_gene;

// Determine the fitness of a program by running the program and comparing it to the desired
// result. Also gives a slight bonus to shorter programs.
fn fitness(g: &prog_gene::ProgramGene) -> f32 {
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
            if result == a + b {
                successful += 1;
            }
            total += 1;
        }
    }
    // Fitness is successful / total test cases
    successful as f32 / total as f32
}

pub fn main() {

}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_fitness() {
        // A program that produces correct output should have fitness of 1.0
        let good_prog = prog_gene::ProgramGene(vec![lang::Prog::C(lang::Command::Add)]);
        assert_eq!(fitness(&good_prog), 1.0);

        // A program that produces all incorrect output should have fitness of 0.0
        let bad_prog = prog_gene::ProgramGene(vec![lang::Prog::D(-1)]);
        assert_eq!(fitness(&bad_prog), 0.0);
    }
}
