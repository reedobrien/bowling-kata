package bowling_test

import (
	"testing"

	"github.com/reedobrien/checkers"
	"gitlab.com/reedobrien/bowling"
)

func TestGutterGame(t *testing.T) {
	g := bowling.NewGame()
	rollMany(&g, 20, 0)
	checkers.Equals(t, g.Score(), 0)
}

func TestAllOnes(t *testing.T) {
	g := bowling.NewGame()
	rollMany(&g, 20, 1)
	checkers.Equals(t, g.Score(), 20)
}

func TestOneSpare(t *testing.T) {
	g := bowling.NewGame()
	rollSpare(&g)
	g.Roll(3)
	rollMany(&g, 17, 0)
	checkers.Equals(t, g.Score(), 16)
}

func TestRollStrike(t *testing.T) {
	g := bowling.NewGame()
	rollStrike(&g)
	g.Roll(3)
	g.Roll(4)
	rollMany(&g, 16, 0)
	checkers.Equals(t, g.Score(), 24)
}

func TestPerfectGame(t *testing.T) {
	g := bowling.NewGame()
	rollMany(&g, 12, 10)
	checkers.Equals(t, g.Score(), 300)
}

func rollStrike(g *bowling.Game) {
	g.Roll(10)
}

func rollSpare(g *bowling.Game) {
	g.Roll(5)
	g.Roll(5)
}

func rollMany(g *bowling.Game, n, p int) {
	for i := 0; i < n; i++ {
		g.Roll(p)
	}
}
