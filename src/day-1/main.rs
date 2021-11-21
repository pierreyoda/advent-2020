use anyhow::{anyhow, Result};

use advent_2020_common::run_with_scaffolding_integers;

const TARGET: u32 = 2020;

fn compute_solution_2(data: &[u32]) -> Result<u32> {
    // brute-force
    for datum1 in data.iter() {
        for datum2 in data.iter() {
            let sum = datum1 + datum2;
            if sum != TARGET {
                continue;
            }
            return Ok(datum1 * datum2);
        }
    }

    Err(anyhow!("No result found!"))
}

fn compute_solution_3(data: &[u32]) -> Result<u32> {
    // brute-force
    for datum1 in data.iter() {
        for datum2 in data.iter() {
            for datum3 in data.iter() {
                let sum = datum1 + datum2 + datum3;
                if sum != TARGET {
                    continue;
                }
                return Ok(datum1 * datum2 * datum3);
            }
        }
    }

    Err(anyhow!("No result found!"))
}

fn main() -> Result<()> {
    // Part 1
    run_with_scaffolding_integers("day-1", b'\n', |inputs| Ok(compute_solution_2(&inputs)?))?;

    // Part 2
    run_with_scaffolding_integers("day-1", b'\n', |inputs| Ok(compute_solution_3(&inputs)?))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{compute_solution_2, compute_solution_3};

    #[test]
    fn test_compute_solution() {
        assert_eq!(
            compute_solution_2(&[1721, 979, 366, 299, 675, 1456]).unwrap(),
            514579,
        );

        assert_eq!(
            compute_solution_3(&[1721, 979, 366, 299, 675, 1456]).unwrap(),
            241861950,
        );
    }
}
