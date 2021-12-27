use std::fmt::Display;

fn main() {
    //total_population
    let input = include_str!("day6.txt");
    let initial_fish = input
        .split(",")
        .map(|fish| fish.parse::<usize>())
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap())
        .collect::<Vec<usize>>();
    part1(&initial_fish);
    part2(&initial_fish);
}

const BIRTH_SEED: usize = 8;
const MOLT_SEED: usize = 6;

#[derive(Debug, Clone, Copy)]
struct Fish {
    birth_seed: usize,
    molt_seed: usize,
    timer: usize,
}

impl Fish {
    fn new(seed_number: usize) -> Self {
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
    pub fn age(&self) -> usize {
        self.timer
    }
}

#[derive(Clone)]
struct FishState {
    count: usize,
    population: [usize; 9],
}

impl FishState {
    fn new(last_pop: [usize; 9]) -> Self {
        FishState {
            count: last_pop.clone().len() as usize,
            population: last_pop.clone(),
        }
    }

    fn update(&mut self) {
        let last_pop = self.population.clone();
        let mut current_pop: [usize; 9] = [0; 9];
        let gen: usize = last_pop[0];
        current_pop[0] = last_pop[1];
        current_pop[1] = last_pop[2];
        current_pop[2] = last_pop[3];
        current_pop[3] = last_pop[4];
        current_pop[4] = last_pop[5];
        current_pop[5] = last_pop[6];
        current_pop[6] = last_pop[7] + gen;
        current_pop[7] = last_pop[8];
        current_pop[8] = gen;
        self.population = current_pop.clone();
        self.count = current_pop.iter().sum::<usize>();
    }

    fn count_fish(&self) -> usize {
        self.count
    }
}
impl Display for FishState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fishstate = self
            .population
            .iter()
            .map(|fish| fish.to_string())
            .collect::<Vec<String>>()
            .join(",");
        write!(f, "{}", fishstate);
        Ok(())
    }
}
struct Universe {
    history: Vec<FishState>,
    lifetime: usize,
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
    fn new(inital_fish: Vec<usize>, lifetime: usize) -> Self {
        let mut history = Vec::<FishState>::new();
        let mut population: [usize; 9] = [0, 0, 0, 0, 0, 0, 0, 0, 0];
        for fish in inital_fish {
            population[fish as usize] += 1;
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
        let last_pop: [usize; 9] = self.history[self.history.len() - 1].population.clone();
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

fn part1(initial_fish: &Vec<usize>) {
    let mut universe = Universe::new(initial_fish.clone(), 80);
    universe.run();
    println!(
        "total pop: {}",
        universe.history.last().unwrap().count_fish()
    );
}

fn part2(initial_fish: &Vec<usize>) {
    let mut universe = Universe::new(initial_fish.clone(), 256);
    universe.run();
    println!(
        "total pop: {}",
        universe.history.last().unwrap().count_fish()
    );
}

mod test {
    use super::*;
    #[test]
    fn test_part1() {
        let input = include_str!("day6.txt");
        let initial_fish = input
            .split(",|\r\n|\n")
            .map(|fish| fish.parse::<usize>())
            .filter(|x| x.is_ok())
            .map(|x| x.unwrap())
            .collect::<Vec<usize>>();
        part1(&initial_fish);
    }

    #[test]
    fn test_fish() {
        let f1 = Fish::new(3);
        let f2 = Fish::new(4);
        assert_eq!(f1.age(), 3);
        assert_eq!(f2.age(), 4);
    }

    #[test]
    fn test_universe18() {
        let mut universe = Universe::new(vec![3, 4, 3, 1, 2], 18);
        universe.run();
        assert_eq!(universe.history.last().unwrap().count_fish(), 26);
    }

    #[test]
    fn test_universe80() {
        let mut universe = Universe::new(vec![3, 4, 3, 1, 2], 80);
        universe.run();
        assert_eq!(universe.history.last().unwrap().count_fish(), 5934);
    }

    #[test]
    fn test_universe256() {
        let mut universe = Universe::new(vec![3, 4, 3, 1, 2], 256);
        universe.run();
        assert_eq!(universe.history.last().unwrap().count_fish(), 26984457539);
    }
}
