package api

import (
	"github.com/xavimg/Turing/BackenWebV2/internal/database"
	"github.com/xavimg/Turing/BackenWebV2/internal/logs"
)

type PlayerFilter struct {
	Username string `json:"username"`
	Level    string `json:"level"`
}

type Player struct {
	Id       int    `json:"id"`
	Username string `json:"username"`
	Level    string `json:"level"`
	Score    int    `json:"score"`
}

type PlayerSearch interface {
	Search(filter PlayerFilter) ([]Player, error)
}

type PlayerService struct {
	*database.MySqlClient
}

func (p *PlayerService) Search(filter PlayerFilter) ([]Player, error) {
	tx, err := p.Begin()
	if err != nil {
		logs.Error("cannot create transaction")
		return nil, err
	}

	rows, err := tx.Query(getPlayersQuery(filter))
	if err != nil {
		logs.Error("cannot read players" + err.Error())
		_ = tx.Rollback()
		return nil, err
	}

	var _players []Player
	for rows.Next() {
		var player Player
		err := rows.Scan(&player.Id, &player.Username, &player.Level, &player.Score)
		if err != nil {
			logs.Error("cannot read players " + err.Error())
		}
		_players = append(_players, player)
	}
	_ = tx.Commit()

	logs.Info("DB Query")
	return _players, nil
}
