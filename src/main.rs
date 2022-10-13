use clap::Parser;
use rand::Rng;
use rand_distr::{Distribution, Normal};
use std::cmp::Ordering::Less;

struct Individual {
    value: f64,
}

impl Individual {
    fn new(lower_bound: f64, upper_bound: f64) -> Self {
        let mut rng = rand::thread_rng();

        Self {
            value: rng.gen_range(lower_bound..upper_bound),
        }
    }

    fn pair(&self, other: &Individual, alpha: f64) -> Individual {
        Individual {
            value: (self.value * alpha) + (other.value * (1. - alpha)),
        }
    }

    fn mutate(&mut self, lower_bound: f64, upper_bound: f64, rate: f64) {
        let normal = Normal::new(0., rate).unwrap();
        self.value = normal.sample(&mut rand::thread_rng());

        if self.value < lower_bound {
            self.value = lower_bound;
        } else if self.value > upper_bound {
            self.value = upper_bound;
        }
    }
}

struct Population {
    fitness: fn(f64) -> f64,
    individuals: Vec<Individual>,
    size: usize,
}

impl Population {
    fn new(
        size: usize,
        fitness: fn(f64) -> f64,
        init_lower_bound: f64,
        init_upper_bound: f64,
    ) -> Self {
        let mut individuals = vec![];

        for _ in 0..size {
            individuals.push(Individual::new(init_lower_bound, init_upper_bound));
        }

        Self {
            size,
            fitness,
            individuals,
        }
    }
    fn replace(&mut self, mut offspring: Vec<Individual>) {
        self.individuals.append(&mut offspring);
        self.individuals.sort_by(|a, b| {
            let fitness_a = (self.fitness)(a.value);
            let fitness_b = (self.fitness)(b.value);
            fitness_b.partial_cmp(&fitness_a).unwrap_or(Less)
        });
        self.individuals.truncate(self.size);
    }
}

struct Evolution {
    pool: Population,
    n_offsprings: u64,
    pair_alpha: f64,
    mutate_lower_bound: f64,
    mutate_upper_bound: f64,
    mutate_rate: f64,
}

impl Evolution {
    fn new(
        fitness: fn(f64) -> f64,
        pool_size: usize,
        n_offsprings: u64,
        pair_alpha: f64,
        mutate_lower_bound: f64,
        mutate_upper_bound: f64,
        mutate_rate: f64,
        init_lower_bound: f64,
        init_upper_bound: f64,
    ) -> Self {
        Self {
            pool: Population::new(pool_size, fitness, init_lower_bound, init_upper_bound),
            n_offsprings,
            pair_alpha,
            mutate_lower_bound,
            mutate_upper_bound,
            mutate_rate,
        }
    }
    fn step(&mut self) {
        // Parent Selection
        let mut parents = self.pool.individuals.iter().rev();

        let mut offspring = vec![];

        for _ in 0..(2 * self.n_offsprings) {
            // Recombination
            let mother = parents
                .next()
                .expect("Not enough individuals left in population to select a mother.");
            let father = parents
                .next()
                .expect("Not enough individuals left in population to select a father.");
            let mut child = mother.pair(father, self.pair_alpha);

            // Mutation
            child.mutate(
                self.mutate_lower_bound,
                self.mutate_upper_bound,
                self.mutate_rate,
            );

            offspring.push(child);
        }

        // Survivor selection mechanism
        self.pool.replace(offspring);
    }
}

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    #[arg(long, default_value_t = 10)]
    pool_size: usize,
    #[arg(long, default_value_t = 2)]
    n_offsprings: u64,
    #[arg(long, default_value_t = 50)]
    n_epochs: u64,
    #[arg(long, default_value_t = 0.5)]
    pair_alpha: f64,
    #[arg(long, default_value_t = 0.)]
    mutate_lower_bound: f64,
    #[arg(long, default_value_t = 4.)]
    mutate_upper_bound: f64,
    #[arg(long, default_value_t = 0.25)]
    mutate_rate: f64,
    #[arg(long, default_value_t = 0.)]
    init_lower_bound: f64,
    #[arg(long, default_value_t = 4.)]
    init_upper_bound: f64,
}

fn main() {
    // Evaluation function
    fn fitness(opt: f64) -> f64 {
        -opt * (opt - 1.) * (opt - 2.) * (opt - 3.) * (opt - 4.)
    }

    let args = Args::parse();

    let mut evo = Evolution::new(
        fitness,
        args.pool_size,
        args.n_offsprings,
        args.pair_alpha,
        args.mutate_lower_bound,
        args.mutate_upper_bound,
        args.mutate_rate,
        args.init_lower_bound,
        args.init_upper_bound,
    );

    for _ in 0..args.n_epochs {
        evo.step();
    }

    println!("{}", evo.pool.individuals[0].value);
}
