package database

import (
	"authSystem/models"

	"gorm.io/driver/mysql"
	"gorm.io/gorm"
)

var DB *gorm.DB

func Connection() {
	connection, err := gorm.Open(mysql.Open("root:@/alan"), &gorm.Config{})

	if err != nil {
		panic("Impossible to connect")
	}

	DB = connection

	connection.AutoMigrate(&models.User{})
}
