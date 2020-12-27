use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    println!("Starting I Was Told THee Wouldn't Be Math");

    let mut total_paper = 0;
    let mut total_ribbon = 0;


    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(s) = line {
                let mut split = s.split("x");
                let l = split.next().unwrap().parse::<u64>().unwrap();
                let w = split.next().unwrap().parse::<u64>().unwrap();
                let h = split.next().unwrap().parse::<u64>().unwrap();

                total_paper = total_paper + total_wrapping_paper_needed(l, w, h);
                total_ribbon = total_ribbon + total_ribbon_needed(l, w, h);
            }
        }
    }

    println!("Total Wrapping Paper Needed: {}", total_paper);
    println!("Total Ribbon Needed: {}", total_ribbon);
}

fn total_wrapping_paper_needed(l : u64, w : u64, h : u64) -> u64 {
    return compute_surface_area(l, w, h) + smallest_side(l, w, h);
} 

fn total_ribbon_needed(l : u64, w : u64, h : u64) -> u64 {
    return calculate_ribbon_bow(l, w, h) + calculate_ribbon_wrap(l, w, h);
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

fn calculate_ribbon_bow(l : u64, w : u64, h : u64) -> u64 {
    return l * w * h;
}

fn calculate_ribbon_wrap(l : u64, w : u64, h : u64) -> u64 {
    if l >= w && l >= h {
        return 2 * (w + h);
    }
    if w >= l && w >=h {
        return 2 * (l + h);
    }
    return 2 * (l + w);
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
    fn test_2_3_4_paper() {
        assert_eq!(total_wrapping_paper_needed(2, 3, 4), 58);
    }

    #[test]
    fn test_1_1_10_paper() {
        assert_eq!(total_wrapping_paper_needed(1, 1, 10), 43);
    }

    #[test]
    fn test_2_3_4_ribbon() {
        assert_eq!(total_ribbon_needed(2, 3, 4), 34);
    }

    #[test]
    fn test_1_1_10_ribbon() {
        assert_eq!(total_ribbon_needed(1, 1, 10), 14);
    }
}

