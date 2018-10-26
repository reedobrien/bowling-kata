require "bowling/version"

module Bowling
  class Game
    def initialize()
      @current_roll = 0
      @rolls = Array.new(21, 0)
    end

    def roll(pins)
      @rolls[@current_roll] = pins
      @current_roll += 1
    end

    def score
      score, frame, frame_idx = 0, 0, 0
      while frame < 10
        if is_strike frame_idx
          score += 10 + strike_bonus(frame_idx)
          frame_idx += 1
        elsif is_spare(frame_idx)
          score += 10 + spare_bonus(frame_idx)
          frame_idx += 2
        else
          score += sum_of_balls_in_frame(frame_idx)
          frame_idx += 2
        end

        frame += 1
      end
      score
    end

    def is_spare(idx)
      @rolls[idx] + @rolls[idx + 1] == 10
    end

    def is_strike(idx)
      @rolls[idx] == 10
    end

    def spare_bonus(idx)
      @rolls[idx + 2]
    end

    def strike_bonus(idx)
      @rolls[idx + 1] + @rolls[idx + 2]
    end

    def sum_of_balls_in_frame(idx)
      @rolls[idx] + @rolls[idx + 1]
    end
  end
end
