// 1. All gutter balls -> 0
// 2. All ones -> 20
// 3. Spare + 3 + remaining 0 -> 16
// 4. Strike + 3 + 4 + remaining 0 -> 24
// 5. Perfect Game -> 300
//

pub struct Game {
    current_roll: usize,
    rolls: Vec<i32>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            current_roll: 0,
            rolls: vec![0; 21],
        }
    }

    pub fn roll(&mut self, pins: i32) {
        self.rolls[self.current_roll] = pins;
        self.current_roll += 1
    }

    pub fn score(&self) -> i32 {
        let mut score = 0;
        let mut frame_idx = 0;

        for _ in 0..10 {
            if is_mark(self.rolls[frame_idx]) {
                score += 10 + self.rolls[frame_idx + 1] + self.rolls[frame_idx + 2];
                frame_idx += 1;
                continue;
            }

            if is_mark(self.rolls[frame_idx] + self.rolls[frame_idx + 1]) {
                score += 10 + self.rolls[frame_idx + 2];
                frame_idx += 2;
                continue;
            }

            score += self.rolls[frame_idx] + self.rolls[frame_idx + 1];
            frame_idx += 2;
        }

        score
    }
}

pub fn is_mark(pins: i32) -> bool {
    pins == 10
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_all_gutterballs() {
        let mut g = Game::new();
        for _ in 0..20 {
            g.roll(0);
        }
        assert_eq!(g.score(), 0)
    }

    #[test]
    fn test_all_ones() {
        let mut g = Game::new();
        for _ in 0..20 {
            g.roll(1);
        }
        assert_eq!(g.score(), 20)
    }

    #[test]
    fn test_spare() {
        let mut g = Game::new();
        g.roll(5);
        g.roll(5);
        g.roll(3);
        assert_eq!(g.score(), 16)
    }

    #[test]
    fn test_strike() {
        let mut g = Game::new();
        g.roll(10);
        g.roll(3);
        g.roll(4);
        assert_eq!(g.score(), 24)
    }

    #[test]
    fn test_perfect_game() {
        let mut g = Game::new();
        for _ in 0..12 {
            g.roll(10);
        }
        assert_eq!(g.score(), 300)
    }
}
