package repository

import (
	"github.com/jinzhu/gorm"
	_ "github.com/jinzhu/gorm/dialects/mysql"
	"github.com/xavimg/Turing/APIrest/entity"
)

type PlayerRepository interface {
	Save(video entity.Player)
	Update(video entity.Player)
	Delete(video entity.Player)
	FindAll() []entity.Player
	CloseDB()
}

type database struct {
	connection *gorm.DB
}

func NewConstructorRepository() PlayerRepository {
	db, err := gorm.Open("mysql", "apiweb.db")
	if err != nil {
		panic("Failed to connect database")
	}
	db.AutoMigrate(&entity.Player{}, &entity.Character{})
	return &database{
		connection: db,
	}
}

func (db *database) CloseDB() {
	err := db.connection.Close()
	if err != nil {
		panic("Failed to close database")
	}
}

func (db *database) Save(player entity.Player) {
	db.connection.Create(&player)
}

func (db *database) Update(player entity.Player) {
	db.connection.Save(&player)
}

func (db *database) Delete(player entity.Player) {
	db.connection.Delete(&player)
}

func (db *database) FindAll() []entity.Player {
	var players []entity.Player
	db.connection.Set("gorm:auto_preload", true).Find(&players)
	return players
}
