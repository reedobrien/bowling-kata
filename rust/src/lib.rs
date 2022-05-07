#![allow(unused)]
use std::sync::Arc;

const FRAMES_IN_GAME: i32 = 10;
const PINS_IN_FRAME: i32 = 10;
const ROLLS_IN_FRAME: usize = 2;
const ROLLS_IN_MARK: usize = 3;
const ROLLS_IN_STRIKE: usize = 1;

// 1. All gutter balls -> 0
// 2. All ones -> 20
// 3. Spare + 3 + remaining 0 -> 16
// 4. Strike + 3 + 4 + remaining 0 -> 24
// 5. Perfect Game -> 300

#[derive(Clone)]
struct Game {
    rolls: Vec<i32>,
}

impl Game {
    fn new() -> Game {
        Game { rolls: Vec::new() }
    }

    fn roll(&self, pins: i32) -> Game {
        // let mut rolls = self.rolls.clone();
        // rolls.push(pins);
        // Game { rolls }

        Game {
            rolls: vec![self.rolls.clone(), vec![pins]].concat(),
        }
    }

    fn score(&self) -> i32 {
        let idx_score = (0, 0);
        let f = self.calculate_score(self.rolls.clone());

        let idx_score = (0..FRAMES_IN_GAME).fold(idx_score, |idx_score, _| f(idx_score));

        idx_score.1
    }

    fn calculate_score(&self, scores: Vec<i32>) -> impl Fn((usize, i32)) -> (usize, i32) + '_ {
        let scores = Arc::new(scores);

        move |idx_score: (usize, i32)| {
            let scores = scores.to_vec();

            if self.is_spare(idx_score.0, scores.clone()) {
                return self.spare_score(idx_score, scores.clone());
            }

            if self.is_strike(idx_score.0, scores.clone()) {
                return self.strike_score(idx_score, scores.clone());
            }

            self.sum_scores(idx_score, scores.clone())
        }
    }

    fn is_spare(&self, idx: usize, scores: Vec<i32>) -> bool {
        let points: i32 = scores
            .clone()
            .into_iter()
            .skip(idx)
            .take(ROLLS_IN_FRAME)
            .sum();

        points == PINS_IN_FRAME
    }

    fn is_strike(&self, idx: usize, scores: Vec<i32>) -> bool {
        scores.get(idx) == Some(&PINS_IN_FRAME)
    }

    fn spare_score(&self, idx_score: (usize, i32), scores: Vec<i32>) -> (usize, i32) {
        let points: i32 = scores
            .clone()
            .into_iter()
            .skip(idx_score.0)
            .take(ROLLS_IN_MARK)
            .sum();

        (idx_score.0 + ROLLS_IN_FRAME, idx_score.1 + points)
    }

    fn strike_score(&self, idx_score: (usize, i32), scores: Vec<i32>) -> (usize, i32) {
        let points: i32 = scores
            .clone()
            .into_iter()
            .skip(idx_score.0)
            .take(ROLLS_IN_MARK)
            .sum();

        (idx_score.0 + ROLLS_IN_STRIKE, idx_score.1 + points)
    }

    fn sum_scores(&self, idx_score: (usize, i32), scores: Vec<i32>) -> (usize, i32) {
        let points: i32 = scores
            .clone()
            .into_iter()
            .skip(idx_score.0)
            .take(ROLLS_IN_FRAME)
            .sum();

        (idx_score.0 + ROLLS_IN_FRAME, idx_score.1 + points)
    }
}

mod tests {
    use super::*;

    impl Game {
        fn roll_frame(&self, fst: i32, snd: i32) -> Game {
            self.roll(fst).roll(snd)
        }
    }

    #[test]
    fn all_gutters() {
        let mut g = Game::new();

        for _ in 0..FRAMES_IN_GAME {
            g = g.roll_frame(0, 0)
        }

        assert_eq!(g.score(), 0);
    }

    #[test]
    fn all_ones() {
        let mut g = Game::new();

        for _ in 0..FRAMES_IN_GAME {
            g = g.roll_frame(1, 1)
        }

        assert_eq!(g.score(), 20)
    }

    #[test]
    fn one_spare() {
        let mut g = Game::new();

        g = g.roll_frame(5, 5);
        g = g.roll_frame(3, 0);

        for _ in 0..FRAMES_IN_GAME - 2 {
            g = g.roll_frame(0, 0)
        }

        assert_eq!(g.score(), 16);
    }

    #[test]
    fn one_strike() {
        let mut g = Game::new();

        g = g.roll(PINS_IN_FRAME);
        g = g.roll_frame(3, 4);

        for _ in 0..FRAMES_IN_GAME - 2 {
            g = g.roll_frame(0, 0);
        }

        assert_eq!(g.score(), 24);
    }

    #[test]
    fn all_strikes() {
        let mut g = Game::new();
        const BONUS_ROLLS: i32 = 2;

        for _ in 0..FRAMES_IN_GAME + BONUS_ROLLS {
            g = g.roll(PINS_IN_FRAME)
        }

        assert_eq!(g.score(), 300);
    }
}
