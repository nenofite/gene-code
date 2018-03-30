//
// The genetic algorithm
//

extern crate rand;
use rand::Rng;

// A type that can be used as a gene. Specifically, it must support random generation and mutation.
pub trait Gene {
    // Generate a new random gene. This is initially used to fill the pool.
    fn generate<R: Rng>(rng: &mut R) -> Self;

    // Generate a new gene that is a mutation of this gene.
    fn mutate<R: Rng>(&self, rng: &mut R) -> Self;
}

// A pool of genes
pub struct Pool<T, F> {
    // The genes in the pool paired with their fitness, in no particular order. Do not assume the
    // fitness value is up to date
    pub genes: Vec<(T, f32)>,
    // The back-buffer of genes, used when stirring and mutating the pool
    back_genes: Vec<(T, f32)>,
    // The fitness function
    fitness: F,
}

impl<T, F> Pool<T, F>
    where T: Gene,
          F: Fn(&T) -> f32,
    {

    // Create and fill a pool of the given size.
    pub fn new<R: Rng>(size: usize, fitness: F, rng: &mut R) -> Self {
        let mut pool = Pool {
            genes: Vec::with_capacity(size),
            back_genes: Vec::with_capacity(size),
            fitness: fitness,
        };
        for _ in 0 .. size {
            let gene = Gene::generate(rng);
            let fit = (pool.fitness)(&gene);
            pool.genes.push((gene, fit));
        }
        pool
    }

    // Evolve one generation using the given fitness function. All genes currently in the pool are
    // evaluated for fitness, then the most fit half is kept and the least fit half is replaced
    // with mutations of the more fit half.
    pub fn evolve<R: Rng>(&mut self, rng: &mut R) {
        // The pool size to maintain
        let len = self.genes.len();

        // Swap into the back buffer so we can assemble a new pool of genes
        ::std::mem::swap(&mut self.genes, &mut self.back_genes);

        // Sum up the total fitness
        let mut total_fitness = 0.0;
        for pair in &self.back_genes {
            total_fitness += pair.1;
        }

        // Fill the first third of the pool by stochastic selection (higher fitness = more likely
        // to be selected), and the next third with mutations of these selected genes
        self.genes.clear();
        while self.genes.len() < len * 2/3 && !self.back_genes.is_empty() {
            // Pick a number within total fitness
            let mut f = rng.gen_range(0.0, total_fitness);
            // Select the gene under that fitness offset
            let mut i = 0;
            f -= self.back_genes[i].1;
            while f > 0.0 {
                i = (i + 1) % self.back_genes.len();
                f -= self.back_genes[i].1;
            }
            // Subtract its fitness from the total
            total_fitness -= self.back_genes[i].1;
            // Add a mutation of the gene
            let mutated_gene = self.back_genes[i].0.mutate(rng);
            let mutated_fit = (self.fitness)(&mutated_gene);
            self.genes.push((mutated_gene, mutated_fit));
            // Move the gene from back_genes to genes
            self.genes.push(self.back_genes.remove(i));
        }

        // Fill the last third by generating new genes
        while self.genes.len() < len {
            let generated_gene = Gene::generate(rng);
            let generated_fit = (self.fitness)(&generated_gene);
            self.genes.push((generated_gene, generated_fit));
        }
    }

    // Get the current best gene. This is only valid after a call to evolve.
    pub fn get_best(&self) -> &T {
        let mut best = &self.genes[0];
        for g in &self.genes {
            if g.1 > best.1 {
                best = g;
            }
        }
        &best.0
    }
}

#[cfg(test)]
mod tests {
    extern crate rand;
    use super::*;
    use rand::Rng;

    static mut NEXT_ID: i32 = 1;

    struct TestGene {
        id: i32,
    }

    impl Gene for TestGene {
        fn generate<R: Rng>(_rng: &mut R) -> Self {
            unsafe {
                let id = NEXT_ID;
                NEXT_ID += 1;
                TestGene { id: id }
            }
        }

        fn mutate<R: Rng>(&self, _rng: &mut R) -> Self {
            TestGene { id: -self.id }
        }
    }

    #[test]
    fn gen_pool() {
        // A deterministic RNG for testing
        use rand::SeedableRng;
        let rng = &mut rand::Isaac64Rng::from_seed(&[123]);

        // Generate the pool by calling generate() n times.
        let fitness = |g: &TestGene| { g.id as f32 };
        let mut pool = Pool::new(10, fitness, rng);
        assert_eq!(pool.genes[0].0.id, 1);
        assert_eq!(pool.genes[9].0.id, 10);

        // Evolve the pool
        pool.evolve(rng);

        // Make sure 4 new genes were generated
        unsafe {
            assert_eq!(NEXT_ID, 15);
        }

        // Make sure the same genes were selected (because we know the random seed)
        // First the mutation, then the original
        assert_eq!(pool.genes[0].0.id, -6);
        assert_eq!(pool.genes[1].0.id, 6);
        assert_eq!(pool.genes[2].0.id, -9);
        assert_eq!(pool.genes[3].0.id, 9);
        assert_eq!(pool.genes[4].0.id, -1);
        assert_eq!(pool.genes[5].0.id, 1);
        // Remaining third is newly generated genes
        assert_eq!(pool.genes[6].0.id, 11);
        assert_eq!(pool.genes[7].0.id, 12);
        assert_eq!(pool.genes[8].0.id, 13);
        assert_eq!(pool.genes[9].0.id, 14);
        // Also ensure the genes all have up-to-date fitness values
        for g in &pool.genes {
            assert_eq!(g.0.id as f32, g.1);
        }
    }
}
