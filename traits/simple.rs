use std::env;

/* 
 * It seems like this is just a, data-structure of sorts...
 */
struct Container(u8, u8); 

// A trait which checks if 2 items are stored inside of container. 
// Also retrieves first or last value.
/* 
 * 
 */
trait Contains<A, B> {
  fn contains(&self, &A, &B) -> bool; // Explicitly requires `A` and `B`.
  fn first(&self) -> u8;              // Doesn't explicitly require `A` or `B`.
  fn last(&self) -> u8;               // Doesn't explicitly require `A` or `B.
}

impl Contains<u8, u8> for Container {
  // True if the numbers stored are equal. 
  fn contains(&self, number_1: &u8, number_2: &u8) -> bool {
    (&self.0 == number_1) && (&self.1 == number_2)
  }

  // Grab the first number.
  fn first(&self) -> u8 { self.0 }

  // Grab the last number.
  fn last(&self) -> u8 { self.1 }
}

// `C` contains `A` and `B`. In light of that, having to express `A` and
// `B` again is a nuisance. 
fn difference<A, B, C>(container: &C) -> u8 where
  C: Contains<A, B> {
  container.last() - container.first()
}

fn main() {

  let args: Vec<_> = env::args().collect();
  if args.len() > 1 {
      println!("The first argument is {}, and {}", args[1], args[2]);
  }

  let number_1 = args[1].parse().unwrap();
  let number_2 = args[2].parse().unwrap();

  let container = Container(number_1, number_2);

  println!("Does container contain {} and {}: {}", 
      &number_1, &number_2, 
      container.contains(&number_1, &number_2));
    println!("First numer: {}", container.first());
    println!("Last number: {}", container.last());

    println!("The difference is: {}", difference(&container));
}
