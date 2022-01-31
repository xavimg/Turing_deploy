package repository

import (
	"github.com/xavimg/Turing/apituringserver/entity"
	"gorm.io/gorm"
)

type CharacterRepository interface {
	AllCharacters() []entity.Character
	UserCharacter(usernameUser string) []entity.Character
	FindCharacterByUsername(username string) entity.Character
	CreateCharacter(character entity.Character) entity.Character
	DeleteCharacter(character entity.Character)
}

type characterConnection struct {
	connection *gorm.DB
}

func NewCharacterRepository(dbConn *gorm.DB) CharacterRepository {
	return &characterConnection{
		connection: dbConn,
	}
}

func (db *characterConnection) AllCharacters() []entity.Character {
	var characters []entity.Character

	db.connection.Preload("User").Find(&characters)

	return characters
}

func (db *characterConnection) UserCharacter(usernameUser string) []entity.Character {
	var characters []entity.Character

	db.connection.Preload("User").Where("username = ?", usernameUser).Take(&characters)

	return characters
}

func (db *characterConnection) FindCharacterByUsername(usernameUser string) entity.Character {
	var character entity.Character

	db.connection.Preload("User").Find(&character)

	return character
}

func (db *characterConnection) CreateCharacter(character entity.Character) entity.Character {
	db.connection.Save(&character)
	db.connection.Preload("User").Find(&character)

	return character
}

func (db *characterConnection) DeleteCharacter(character entity.Character) {
	db.connection.Delete(&character)
}
