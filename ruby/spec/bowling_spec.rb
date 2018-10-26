RSpec.describe Bowling do
  it "Constructs a new game" do
    expect(Bowling::Game.new).not_to be nil
  end

  it "Scores all gutters as 0" do
    g = make_one
    roll_many(g, 20, 0)

    expect(g.score).to eq(0)
  end

  it "Scores all ones as 20" do
    g = make_one
    roll_many(g, 20, 1)

    expect(g.score).to eq(20)
  end

  it "scores spare + 3 + remaining as gutters -> 16" do
    g = make_one

    roll_spare g
    g.roll 3
    roll_many(g, 17, 0)

    expect(g.score).to eq(16)
  end

  it "scores strike + 3 + 4 as 24" do
    g = make_one

    g.roll 10
    g.roll 3
    g.roll 4
    roll_many(g, 17, 0)

    expect(g.score).to eq(24)
  end

  it "scores a perfect game as 300" do
    g = make_one

    roll_many(g, 12, 10)

    expect(g.score).to eq(300)
  end

  def make_one
    Bowling::Game.new
  end

  def roll_many(game, rolls, pins)
    rolls.times do
      game.roll pins
    end
  end

  def roll_spare(game)
    2.times do
      game.roll 5
    end
  end
end
