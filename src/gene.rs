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
pub struct Pool<T> {
    // The genes in the pool paired with their fitness, in no particular order. Do not assume the
    // fitness value is up to date
    pub genes: Vec<(T, f32)>,
}

impl<T: Gene> Pool<T> {
    // Create and fill a pool of the given size.
    pub fn new<R: Rng>(size: usize, rng: &mut R) -> Pool<T> {
        let mut pool = Pool { genes: Vec::with_capacity(size) };
        for _ in 0 .. size {
            pool.genes.push((Gene::generate(rng), 0.0));
        }
        pool
    }

    // Evolve one generation using the given fitness function. All genes currently in the pool are
    // evaluated for fitness, then the most fit half is kept and the least fit half is replaced
    // with mutations of the more fit half.
    pub fn evolve<F, R>(&mut self, fitness: F, rng: &mut R)
    where F: Fn(&T) -> f32,
          R: Rng {
        // Update fitness of all genes
        for pair in self.genes.iter_mut() {
            pair.1 = fitness(&pair.0);
        }

        // Sort by fitness. Because the pool is almost always two sorted lists concatenated
        // together, stable sort will actually be faster than unstable sort. This is only untrue on
        // the first generation, and if a mutation was vastly better/worse than its parent.
        self.genes.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(::std::cmp::Ordering::Equal).reverse());

        // Replace less fit half with mutations of the more fit half
        let third = self.genes.len() / 3;
        let twothird = self.genes.len() * 2 / 3;
        for i in 0 .. third {
            self.genes[i + third].0 = self.genes[i].0.mutate(rng);
            //self.genes[i + third].0 = self.genes[0].0.mutate(rng);
        }
        for i in twothird .. self.genes.len() {
            self.genes[i].0 = Gene::generate(rng);
        }
    }

    // Get the current best gene. This is only valid after a call to evolve.
    pub fn get_best(&self) -> &T {
        &self.genes[0].0
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
        // A deterministic mock RNG for testing
        //let rng = &mut rand::mock::StepRng::new(0, 1);
        let rng = &mut rand::thread_rng();

        // Generate the pool by calling generate() n times.
        let mut pool = Pool::<TestGene>::new(10, rng);
        assert_eq!(pool.genes[0].0.id, 1);
        assert_eq!(pool.genes[9].0.id, 10);

        // Evolve the pool
        let fitness = |g: &TestGene| { g.id as f32 };
        pool.evolve(fitness, rng);

        // Make sure genes were re-ordered by fitness
        assert_eq!(pool.genes[0].0.id, 10);
        assert_eq!(pool.get_best().id, 10);
        assert_eq!(pool.genes[0].1, 10.0);
        assert_eq!(pool.genes[2].0.id, 8);
        assert_eq!(pool.genes[2].1, 8.0);

        // Make sure middle third is mutations of most fit third
        assert_eq!(pool.genes[3].0.id, -10);
        assert_eq!(pool.genes[5].0.id, -8);

        // Make sure last third are new, random genes
        assert_eq!(pool.genes[6].0.id, 11);
        assert_eq!(pool.genes[9].0.id, 14);
    }
}
