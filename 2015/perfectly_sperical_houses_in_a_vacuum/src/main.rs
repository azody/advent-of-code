use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {
    println!("Starting Perfectly Spehrical House In A Vacuum");

    let mut input = String::new();
    if let Ok(lines) = read_lines("input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(s) = line {
               input = s.clone();
            }
        }
    }
    
    let answer = fly_santa(&input);

    println!("Santa Visited {} houses", answer);
}

#[derive(Copy, Clone, Debug)]
struct House {
    x: i32,
    y: i32,
    visits: i32
}

fn fly_santa(input : &str) -> usize {
   let flight_path = get_flight_path(&input);
   let mut unique_houses : Vec<House> = Vec::new();

   for h in flight_path {
    
        let mut unique = true;

        for u in &unique_houses {
            if h.x == u.x && h.y == u.y {
                unique = false;
                break;
            }
        }  
        if unique { 
            unique_houses.push(h);
        } 
   }

   return unique_houses.len();
}

fn get_flight_path(input : &str) -> Vec<House> {
    let mut current_house : House = House { x: 0, y: 0, visits: 0 };
    let mut houses : Vec<House> = Vec::new();
    houses.push(current_house.clone());

    for c in input.chars() {
        if c == '^' {
            current_house.y = current_house.y + 1;
        } else if c == '>' {
            current_house.x = current_house.x + 1;
        } else if c == 'v' {
            current_house.y = current_house.y - 1;
        } else {
            current_house.x = current_house.x - 1
        }

        houses.push(current_house.clone());
    }
    return houses;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {

    use super::*;

    // > delivers presents to 2 houses: one at the starting location, and one to the east.
    #[test]
    fn one_east() {
        assert_eq!(2, fly_santa(">"));
    }
    //^>v< delivers presents to 4 houses in a square, including twice to the house at his starting/ending location.
    #[test]
    fn square() {
        assert_eq!(4, fly_santa("^>v<"));
    }
    // ^v^v^v^v^v delivers a bunch of presents to some very lucky children at only 2 houses
    #[test]
    fn lucky_children() {
        assert_eq!(2, fly_santa("^v^v^v^v^v"));
    }
}
