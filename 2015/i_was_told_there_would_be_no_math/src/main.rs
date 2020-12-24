use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    println!("Starting I Was Told THee Wouldn't Be Math");

    let mut total_paper = 0;


    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(s) = line {
                let mut split = s.split("x");
                total_paper = total_paper + total_wrapping_paper_needed(split.next().unwrap().parse::<u64>().unwrap(),
                split.next().unwrap().parse::<u64>().unwrap(),
                split.next().unwrap().parse::<u64>().unwrap())
            }
        }
    }

    println!("Total Wrapping Paper Needed: {}", total_paper)
}

fn total_wrapping_paper_needed(l : u64, w : u64, h : u64) -> u64 {
    return compute_surface_area(l, w, h) + smallest_side(l, w, h);
} 

fn compute_surface_area(l : u64, w : u64, h : u64) -> u64 {
    return 2*l*w + 2*w*h + 2*h*l;
}

fn smallest_side(l : u64, w : u64, h : u64) -> u64 {
    if l >= w && l >= h {
        return w* h;
    }
    if w >= l && w >=h {
        return l * h;
    }
    return l * w;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    
    use super::*;

    // 2 * 3 * 4 
    #[test]
    fn test_2_3_4() {
        assert_eq!(total_wrapping_paper_needed(2, 3, 4), 58);
    }

    #[test]
    fn test_1_1_10() {
        assert_eq!(total_wrapping_paper_needed(1, 1, 10), 43);
    }
}

