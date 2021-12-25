use std::fmt::Display;

fn main() {
    //total_population
    let input = include_str!("day6.txt");
    let initial_fish = input
        .split(",")
        .map(|fish| fish.parse::<i32>())
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap())
        .collect::<Vec<i32>>();
    dbg!(&initial_fish);
    part1(initial_fish);
}

const BIRTH_SEED: i32 = 8;
const MOLT_SEED: i32 = 6;

#[derive(Debug, Clone, Copy)]
struct Fish {
    birth_seed: i32,
    molt_seed: i32,
    timer: i32,
}

impl Fish {
    fn new(seed_number: i32) -> Self {
        Fish {
            timer: seed_number,
            birth_seed: BIRTH_SEED,
            molt_seed: MOLT_SEED,
        }
    }

    pub fn update(&mut self) -> Option<Fish> {
        let mut age = self.timer.clone();
        match age {
            0 => {
                age = 6;
                let child = self.create_child();
                self.timer = age;
                return Some(child);
            }
            _ => age -= 1,
        }
        self.timer = age;
        None
    }

    fn create_child(&self) -> Fish {
        let child = Fish::new(8);
        child
    }
    pub fn age(&self) -> i32 {
        self.timer
    }
}

#[derive(Clone)]
struct FishState {
    count: i32,
    population: Vec<Fish>,
}

impl FishState {
    fn new(last_pop: Vec<Fish>) -> Self {
        FishState {
            count: last_pop.clone().len() as i32,
            population: last_pop.clone(),
        }
    }

    fn update(&mut self) {
        let last_pop = self.population.clone();
        let mut old_pop = Vec::<Fish>::new();
        let mut new_pop = Vec::<Fish>::new();
        let mut current_pop = last_pop.len();
        for f in 0..current_pop {
            let mut fish = last_pop[f].clone();

            let child = fish.update();
            old_pop.push(fish);
            if let Some(child) = child {
                new_pop.push(child);
            }
        }
        let combined_pop = old_pop
            .iter()
            .chain(new_pop.iter())
            .cloned()
            .collect::<Vec<_>>();
        self.population = combined_pop.clone();
    }

    fn count_fish(&self) -> i32 {
        self.count
    }
}
impl Display for FishState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fishstate = self
            .population
            .iter()
            .map(|fish| fish.age().to_string())
            .collect::<Vec<String>>()
            .join(",");
        write!(f, "{}", fishstate);
        Ok(())
    }
}
struct Universe {
    history: Vec<FishState>,
    lifetime: i32,
}
/***
 * The universe will keep the total population of all fish as a Vector of Fish
 *
 * The History will be a Vector of States of the universe
 *
 * A state will contain a Vector of Fish
 * Each tick of the universe will generate a new State that is a copy of the previous state
 *
 * let universe = Univserse::new(initial_fish, lifetime);
 * universe.run();
 * The Universe will have a constructor to generate the initial state
 * The only pub method will be run()
 *
 * Run will loop until the lifetime supplied is reached
 * Tick will generate a new state from the previous or initial state
 *
*/
impl Universe {
    fn new(inital_fish: Vec<i32>, lifetime: i32) -> Self {
        let mut history = Vec::<FishState>::new();
        let mut population = Vec::<Fish>::new();
        for fish in inital_fish {
            dbg!(&fish);
            population.push(Fish::new(fish));
        }
        let first_state = FishState::new(population);
        history.push(first_state);
        Universe { history, lifetime }
    }
    fn run(&mut self) {
        println!("Running...");
        while self.history.len() <= self.lifetime.try_into().unwrap() {
            self.tick();
        }
    }

    fn tick(&mut self) {
        let last_pop: Vec<Fish> = self.history[self.history.len() - 1].population.clone();
        let mut new_state = FishState::new(last_pop);
        new_state.update();
        self.history.push(new_state);
    }
}

impl Display for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fishstate = self
            .history
            .iter()
            .map(|fs| fs.to_string())
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{}", fishstate);
        Ok(())
    }
}

fn part1(initial_fish: Vec<i32>) {
    let mut universe = Universe::new(initial_fish, 80);
    universe.run();
    println!("{}", universe.history.len());
    println!(
        "total pop: {}",
        universe.history.last().unwrap().population.len()
    );
}

fn part2(initial_fish: Vec<i32>) {
    let mut universe = Universe::new(initial_fish, 256);
    universe.run();
    println!("{}", universe.history.len());
    println!(
        "total pop: {}",
        universe.history.last().unwrap().population.len()
    );
}

mod test {
    use super::*;
    #[test]
    fn test_part1() {
        let input = include_str!("day6.txt");
        let initial_fish = input
            .split(",|\r\n|\n")
            .map(|fish| fish.parse::<i32>())
            .filter(|x| x.is_ok())
            .map(|x| x.unwrap())
            .collect::<Vec<i32>>();
        part1(initial_fish);
    }

    #[test]
    fn test_fish() {
        let f1 = Fish::new(3);
        let f2 = Fish::new(4);
        assert_eq!(f1.age(), 3);
        assert_eq!(f2.age(), 4);
    }

    #[test]
    fn test_universe() {
        let mut universe = Universe::new(vec![3, 4, 3, 1, 2], 10);
        universe.run();
        assert_eq!(universe.history.len(), 4); // lifetime + 1 for initial state
                                               //dbg!(&universe.history);
        assert_eq!(universe.history.last().unwrap().population.len(), 6);
    }

    #[test]
    fn test_universe18() {
        let mut universe = Universe::new(vec![3, 4, 3, 1, 2], 18);
        universe.run();
        println!("{}", universe);
        assert_eq!(universe.history.len(), 19); // lifetime + 1 for initial state
                                                //dbg!(&universe.history);
        assert_eq!(universe.history.last().unwrap().population.len(), 26);
    }

    #[test]
    fn test_universe80() {
        let mut universe = Universe::new(vec![3, 4, 3, 1, 2], 80);
        universe.run();
        println!("{}", universe);
        assert_eq!(universe.history.len(), 81); // lifetime + 1 for initial state
                                                //dbg!(&universe.history);
        assert_eq!(universe.history.last().unwrap().population.len(), 5934);
    }

    #[test]
    fn test_universe256() {
        let mut universe = Universe::new(vec![3, 4, 3, 1, 2], 256);
        universe.run();
        assert_eq!(
            universe.history.last().unwrap().population.len(),
            26984457539
        );
    }
}
