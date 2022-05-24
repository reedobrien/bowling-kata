use std::{sync::Arc, usize};

#[allow(dead_code)]
const FRAMES_IN_GAME: i32 = 10;
#[allow(dead_code)]
const ROLLS_IN_FRAME: usize = 2;
#[allow(dead_code)]
const ROLLS_IN_MARK: usize = 3;
#[allow(dead_code)]
const ROLLS_IN_STRIKE: usize = 1;
#[allow(dead_code)]
const PINS_IN_FRAME: i32 = 10;

// 1. All gutter balls -> 0
// 2. All ones -> 20
// 3. Spare + 3 + remaining 0 -> 16
// 4. Strike + 3 + 4 + remaining 0 -> 24
// 5. Perfect Game -> 300

#[allow(dead_code)]
struct Game {
    rolls: Vec<i32>,
}

impl Game {
    // fn new() -> Game {
    //     Game { rolls: Vec::new() }
    // }

    // fn roll(&self, pins: i32) -> Game {
    //     Game {
    //         rolls: vec![self.rolls.clone(), vec![pins]].concat(),
    //     }
    // }

    #[allow(dead_code)]
    fn score(&self) -> i32 {
        let idx_score = (0, 0);
        let f = self.calculate(self.rolls.clone());

        let idx_score = (0..FRAMES_IN_GAME).fold(idx_score, |idx_score, _| f(idx_score));

        idx_score.1
    }

    #[allow(dead_code)]
    fn calculate(&self, scores: Vec<i32>) -> impl Fn((usize, i32)) -> (usize, i32) + '_ {
        let scores = Arc::new(scores);

        move |idx_score: (usize, i32)| {
            let scores = scores.to_vec();

            if self.is_strike(idx_score.0, scores.clone()) {
                return self.strike_score(idx_score, scores.clone());
            }

            if self.is_spare(idx_score.0, scores.clone()) {
                return self.spare_score(idx_score, scores.clone());
            }

            self.sum_scores(idx_score, scores.clone())
        }
    }

    #[allow(dead_code)]
    fn spare_score(&self, idx_score: (usize, i32), scores: Vec<i32>) -> (usize, i32) {
        let points: i32 = scores
            .clone()
            .into_iter()
            .skip(idx_score.0)
            .take(ROLLS_IN_MARK)
            .sum();

        (idx_score.0 + ROLLS_IN_FRAME, idx_score.1 + points)
    }

    #[allow(dead_code)]
    fn strike_score(&self, idx_score: (usize, i32), scores: Vec<i32>) -> (usize, i32) {
        let points: i32 = scores
            .clone()
            .into_iter()
            .skip(idx_score.0)
            .take(ROLLS_IN_MARK)
            .sum();

        (idx_score.0 + ROLLS_IN_STRIKE, idx_score.1 + points)
    }

    #[allow(dead_code)]
    fn sum_scores(&self, idx_score: (usize, i32), scores: Vec<i32>) -> (usize, i32) {
        let points: i32 = scores
            .clone()
            .into_iter()
            .skip(idx_score.0)
            .take(ROLLS_IN_FRAME)
            .sum();

        (idx_score.0 + ROLLS_IN_FRAME, idx_score.1 + points)
    }

    #[allow(dead_code)]
    fn is_spare(&self, idx: usize, scores: Vec<i32>) -> bool {
        scores
            .clone()
            .into_iter()
            .skip(idx)
            .take(ROLLS_IN_FRAME)
            .sum::<i32>()
            == PINS_IN_FRAME
    }

    #[allow(dead_code)]
    fn is_strike(&self, idx: usize, scores: Vec<i32>) -> bool {
        scores.get(idx) == Some(&PINS_IN_FRAME)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn only(score: i32, times: usize) -> Vec<i32> {
        std::iter::repeat(score).take(times).collect()
    }

    fn rolls_and_rest(rolls: Vec<i32>, rest: Vec<i32>) -> Game {
        Game {
            rolls: [rolls, rest].concat(),
        }
    }

    #[test]
    fn all_gutters() {
        let g = rolls_and_rest(vec![], only(0, 20));

        assert_eq!(g.score(), 0);
    }

    #[test]
    fn all_ones() {
        let g = rolls_and_rest(vec![], only(1, 20));

        assert_eq!(g.score(), 20);
    }

    #[test]
    fn one_spare() {
        let g = rolls_and_rest(vec![6, 4, 3], only(0, 17));

        assert_eq!(g.score(), 16);
    }

    #[test]
    fn one_strike() {
        let g = rolls_and_rest(vec![10, 3, 4], only(0, 17));

        assert_eq!(g.score(), 24)
    }
    #[test]
    fn perfect_game() {
        let g = rolls_and_rest(vec![], only(10, 12));

        assert_eq!(g.score(), 300);
    }
}
