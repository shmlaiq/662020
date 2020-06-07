#[derive(Debug)]
enum Direction {
    forward,
    backward,
    left,
    right,
} 

fn direct(a:u8) {
   match a {
          1 => println!("{:?}", Direction::forward),
          2 => println!("{:?}", Direction::backward),
          3 => println!("{:?}", Direction::left),
          4 => println!("{:?}", Direction::right),
          5 => println!("{:?}", Direction::forward),
          6 => println!("{:?}", Direction::backward),
          7 => println!("{:?}", Direction::left),
          8 => println!("{:?}", Direction::right),
          9 => println!("{:?}", Direction::forward),
          10 => println!("{:?}", Direction::backward),
          _ => println!("Not in 10 numbers ")         
           };
}


fn main() {
           println!("{:?}", Direction::forward);
           for i in 1..11 {println!("{:?}", direct(i))}
          }
