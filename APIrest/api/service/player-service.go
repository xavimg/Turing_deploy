package service

import "github.com/xavimg/Turing/APIrest/entity"

type PlayerService interface {
	Save(entity.Player) entity.Player
	FindAll() []entity.Player
}

type playerService struct {
	players []entity.Player
}

func NewConstructorService() PlayerService {
	return &playerService{
		players: []entity.Player{},
	}
}

func (p *playerService) Save(player entity.Player) entity.Player {
	p.players = append(p.players, player)
	return player
}
func (p *playerService) FindAll() []entity.Player {
	return p.players
}
