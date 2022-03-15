use std::thread;
use std::time::Duration;
mod iterators;


//the generic is a closure that takes a u32 and returns a u32
struct Cacher<T> where T: Fn(u32) -> u32 {
    calculation: T,
    value: Option<u32>
}
//sthe calculation is the closure, and value is none.
impl<T> Cacher<T> where T: Fn(u32) -> u32 {
    fn new(calculation: T) -> Cacher<T> {
        Cacher {calculation, value: None}
    }
//if value is none, do whats in the closure with their arg inbetween the ||
//store return value from closure in value for struct, and get it for next time
//run expensive calculation no more than once B) this is some gamer shit
    fn value(&mut self, args: u32) -> u32 {
        match self.value {
            Some(val) => val,
            None => {
                let v = (self.calculation)(args);  
                self.value = Some(v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    //make a new cacher put it into expensive result
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
     });

    if intensity < 25 {
        //do a calculation if val doesnt exist and get value and cache
        //otherwise, just do closure body calc, and return val.
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    }
    else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result.value(intensity));
        }
    }
}

fn main() {
    //honestly i dont get iterators what the fuck is going on
    let simulated_user_specified_value = 69;
    let simulated_random_number = 3;
    
    generate_workout(simulated_user_specified_value, simulated_random_number);
    //iterators::iterations()
    
   // iterators::iterations();
}

