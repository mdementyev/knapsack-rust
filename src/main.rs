use std::env;
use std::fs;
use std::io;
use std::io::BufRead;
use std::mem;

const DEFAULT_FILENAME: &str = "input"; // May be overridden with an argument.

/// Recursive knapsack calculation routine
///   depth: the number of the items considered so far (the depth of recursion)
///   sum: the sum of the taken items among considered
///   mask: the bit mask of the taken items
///   returns the best sum for this branch and updates the mask
fn knapsack(limit: f64, items: &Vec<f64>, depth: usize, sum: &mut f64, mask: &mut usize) {
    if depth == items.len() {
        return;
    }

    *mask <<= 1;
    let mut sum_b = *sum + items[depth];
    let mut mask_b = *mask | 0x1;
    knapsack(limit, items, depth + 1, sum, mask);
    if sum_b <= limit {
        knapsack(limit, items, depth + 1, &mut sum_b, &mut mask_b);
        if *sum < sum_b {
            *sum = sum_b;
            *mask = mask_b;
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filename = env::args().nth(1).unwrap_or(String::from(DEFAULT_FILENAME));
    println!("Running '{}'...", filename);

    let limit: f64;
    let mut items: Vec<f64>;
    {
        let file = fs::File::open(&filename)?;
        let lines = io::BufReader::new(file).lines().flat_map(|l| l);
        let mut numbers = lines
            .flat_map(|l| l.parse::<f64>())
            .filter(|n| n.is_finite());
        limit = numbers.next().ok_or(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "No numbers found in the input file",
        ))?;
        items = numbers.collect();
    } // Drop file.
    if items.len() > 8 * mem::size_of::<usize>() {
        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "{} items found; can use {} at most",
                items.len(),
                8 * mem::size_of::<usize>()
            ),
        ))?;
    }
    // Keeping bigger items at the end allows pruning at the later stages,
    // thus balancing the tree traversal
    items.sort_unstable_by(|x, y| x.partial_cmp(y).unwrap()); // No NaNs by now.

    let mut sum: f64 = 0.0;
    let mut mask: usize = 0;
    knapsack(limit, &items, 0, &mut sum, &mut mask);

    println!("Sum: {} / {}", sum, limit);
    println!("Used items: {} / {}", mask.count_ones(), items.len());
    for i in items.iter().rev() {
        if mask & 0x1 != 0 {
            println!("{:.9}", i);
        }
        mask >>= 1;
    }

    Ok(())
}
