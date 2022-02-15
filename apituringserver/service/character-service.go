package service

import "github.com/xavimg/Turing/apituringserver/entity"

type CharacterService interface {
	AllCharacters() []entity.Character
	UserCharacter(usernameUser string) []entity.Character
	FindCharacterByUsername(username string) entity.Character
	CreateCharacter(character entity.Character) entity.Character
	DeleteCharacter(character entity.Character)
}
