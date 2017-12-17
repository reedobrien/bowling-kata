package bowling

// NewGame is the Game constructor.
func NewGame() Game {
	return Game{}
}

// Game is the main type for the bowling game.
type Game struct {
	currentRoll int
	rolls       [21]int
}

// Roll takes the number of pins knocked over in a turn.
func (g *Game) Roll(p int) {
	g.rolls[g.currentRoll] = p
	g.currentRoll++
}

// Score calculates the score from the current state.
func (g *Game) Score() int {
	var (
		score, frameIdx int
	)
	for frame := 0; frame < 10; frame++ {
		if g.isStrike(frameIdx) {
			score += 10 + g.strikeBonus(frameIdx)
			frameIdx++
			continue
		}
		if g.isSpare(frameIdx) {
			score += 10 + g.rolls[frameIdx+2]
			frameIdx += 2
			continue
		}
		score += g.sumOfBallsInFrame(frameIdx)
		frameIdx += 2
	}
	return score
}

func (g *Game) strikeBonus(i int) int {
	return g.rolls[i+1] + g.rolls[i+2]
}

func (g *Game) sumOfBallsInFrame(i int) int {
	return g.rolls[i] + g.rolls[i+1]
}

func (g *Game) isStrike(i int) bool {
	return g.rolls[i] == 10
}

func (g *Game) isSpare(i int) bool {
	return g.rolls[i]+g.rolls[i+1] == 10
}
