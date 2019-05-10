use std::collections::HashMap;
use std::hash::Hash;
use std::thread;
use std::time::Duration;

pub fn main() {
    println!("This is closures!");
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

struct Cache<T, U, V>
where
    T: Fn(U) -> V,
    U: Copy + Hash + Eq,
    V: Copy,
{
    calculation: T,
    value: HashMap<U, V>,
}

impl<T, U, V> Cache<T, U, V>
where
    T: Fn(U) -> V,
    U: Copy + Hash + Eq,
    V: Copy,
{
    fn new(calculation: T) -> Cache<T, U, V> {
        Cache {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: U) -> V {
        match self.value.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                self.value.insert(arg, v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cache::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

#[cfg(test)]
mod tests;
