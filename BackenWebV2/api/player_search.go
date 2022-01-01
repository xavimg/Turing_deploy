package api

import "time"

type PlayerFilter struct {
	Username string `json:"username"`
	Score    int    `json:"score"`
}

type Player struct {
	Username  string    `json:"username"`
	Level     string    `json:"level"`
	Score     int       `json:"score"`
	CreatedAt time.Time `json:"created"`
}

type PlayerSearch interface {
	Search(filter PlayerFilter) ([]Player, error)
}

type PlayerService struct {
}

func (p *PlayerService) Search(filter PlayerFilter) ([]Player, error) {
	p1 := Player{
		Username:  "Werlyb",
		Level:     "Advanzed",
		Score:     5000,
		CreatedAt: time.Now(),
	}

	p2 := Player{
		Username:  "Razork",
		Level:     "Medium",
		Score:     100,
		CreatedAt: time.Now(),
	}

	var _players []Player

	_players = append(_players, p1)
	_players = append(_players, p2)

	return _players, nil

}
