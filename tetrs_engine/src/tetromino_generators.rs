use std::num::NonZeroU32;

use rand::{
    self,
    distributions::{Distribution, WeightedIndex},
    Rng,
};

use crate::Tetromino;

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[allow(dead_code)]
pub enum TetrominoGenerator {
    Uniform,
    Bag {
        pieces_left: [u32; 7],
        multiplicity: NonZeroU32,
    },
    Recency {
        last_generated: [u32; 7],
    },
    TotalRelative {
        relative_counts: [u32; 7],
    },
}

#[allow(dead_code)]
impl TetrominoGenerator {
    pub fn uniform() -> Self {
        Self::Uniform
    }

    pub fn bag(multiplicity: NonZeroU32) -> Self {
        Self::Bag {
            pieces_left: [multiplicity.get(); 7],
            multiplicity,
        }
    }

    pub fn recency() -> Self {
        Self::Recency {
            last_generated: [1; 7],
        }
    }

    pub fn total_relative() -> Self {
        Self::TotalRelative {
            relative_counts: [0; 7],
        }
    }
}

impl Iterator for TetrominoGenerator {
    type Item = Tetromino;

    fn next(&mut self) -> Option<Self::Item> {
        let mut rng = rand::thread_rng();
        match self {
            TetrominoGenerator::Uniform => Some(rng.gen_range(0..=6).try_into().unwrap()),
            TetrominoGenerator::Bag {
                pieces_left,
                multiplicity,
            } => {
                let weights = pieces_left.iter().map(|&c| if c > 0 { 1 } else { 0 });
                // SAFETY: Struct invariant.
                let idx = WeightedIndex::new(weights).unwrap().sample(&mut rng);
                // Update individual tetromino number and maybe replenish bag (ensuring invariant).
                pieces_left[idx] -= 1;
                if pieces_left.iter().sum::<u32>() == 0 {
                    *pieces_left = [multiplicity.get(); 7];
                }
                // SAFETY: 0 <= idx <= 6.
                Some(idx.try_into().unwrap())
            }
            TetrominoGenerator::TotalRelative { relative_counts } => {
                let weighing = |&x| 1.0 / f64::from(x).exp(); // Alternative weighing function: `1.0 / (f64::from(x) + 1.0);`
                let weights = relative_counts.iter().map(weighing);
                // SAFETY: `weights` will always be non-zero due to `weighing`.
                let idx = WeightedIndex::new(weights).unwrap().sample(&mut rng);
                // Update individual tetromino counter and maybe rebalance all relative counts
                relative_counts[idx] += 1;
                // SAFETY: `self.relative_counts` always has a minimum.
                let min = *relative_counts.iter().min().unwrap();
                if min > 0 {
                    for x in relative_counts.iter_mut() {
                        *x -= min;
                    }
                }
                // SAFETY: 0 <= idx <= 6.
                Some(idx.try_into().unwrap())
            }
            TetrominoGenerator::Recency { last_generated } => {
                /* Choice among these weighing functions:
                 * `|x| x; // x -> x`
                 * `|&x| f64::from(x).powf(1.5); // x -> x^1.5`
                 * `|x| x * x; // x -> x^2`
                 * `|&x| f64::from(x).exp() - 1.0; // x -> exp x - 1`
                 */
                let weighing = |x| x * x;
                let weights = last_generated.iter().map(weighing);
                // SAFETY: `weights` will always be non-zero due to `weighing`.
                let idx = WeightedIndex::new(weights).unwrap().sample(&mut rng);
                // Update all tetromino last_played values and maybe rebalance all relative counts..
                for x in last_generated.iter_mut() {
                    *x += 1;
                }
                last_generated[idx] = 0;
                // SAFETY: 0 <= idx <= 6.
                Some(idx.try_into().unwrap())
            }
        }
    }
}
