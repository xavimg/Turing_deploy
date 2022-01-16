package service

import (
	"github.com/xavimg/Turing/APIrest/api/entity"
	"github.com/xavimg/Turing/APIrest/api/repository"
)

type PlayerService interface {
	Save(entity.Player) entity.Player
	Update(entity.Player)
	Delete(entity.Player)
	FindAll() []entity.Player
}

type playerService struct {
	playerRepository repository.PlayerRepository
}

func NewConstructorService(repo repository.PlayerRepository) PlayerService {
	return &playerService{
		playerRepository: repo,
	}
}

func (p *playerService) Save(player entity.Player) entity.Player {
	p.playerRepository.Save(player)
	return player
}
func (p *playerService) FindAll() []entity.Player {
	return p.playerRepository.FindAll()
}

func (p *playerService) Update(player entity.Player) {
	p.playerRepository.Update(player)
}
func (p *playerService) Delete(player entity.Player) {
	p.playerRepository.Delete(player)
}
