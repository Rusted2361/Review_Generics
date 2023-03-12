use std::fmt::Display;

// Generic Struct Definition, T is a variable for unknown type
struct Stats<T> {
    age: T,
    height:T
}

// Generic Tuple Struct that holds one value
struct Number<T> (T);

// trait with default implementation of print_stats method
trait ViewStats {
  fn print_stats(&self){
    println!("The type of age and height doesn't implement the display trait")
  }
}

// Impl blocks defining methods for different types
impl<T:Display> ViewStats for Stats<T> {
  fn print_stats(&self){
    println!("Age is {} years and Height is {}", self.age, self.height);
  }
}

//Impl block to ViewStats trait to number but use default implementation
impl<T> ViewStats for Stats<Number<T>> {}


fn main() {
    // Instantiate using i32 stats
    let alex = Stats {age: 37, height: 70};
    // Instantiate using f32 stats
    let alex2 = Stats {age: 37.0, height: 5.83};
    // Instantiate using String stats
    let alex3 = Stats {age: "37".to_string(), height: "5'10ft".to_string()};
    // Instantiate using String stats
    let alex4 = Stats {age: Number(37), height: Number(70)};
    // Call methods
    alex.print_stats();
    alex2.print_stats();
    alex3.print_stats();
    alex4.print_stats();
}