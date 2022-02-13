package config

import (
	"github.com/joho/godotenv"
	"github.com/xavimg/Turing/apituringserver/entity"
	"gorm.io/driver/postgres"
	"gorm.io/gorm"

	_ "github.com/lib/pq"
)

// SetupDatabaseConnection is creating a new connection to our database
func SetupDatabaseConnection() *gorm.DB {
	errEnv := godotenv.Load()

	if errEnv != nil {
		panic("Failed to load env file")
	}

	dsn := "host=db user=postgres password=v6vpxdkd dbname=alanturing port=5432 sslmode=disable"

	db, err := gorm.Open(postgres.Open(dsn), &gorm.Config{})

	if err != nil {
		panic("Failed to create a connection to database")
	}

	db.AutoMigrate(
		&entity.User{},
		&entity.Character{},
	)

	return db
}

// CLoseDatabaseConnection method is closing a connection between your app and your database
func CloseDatabaseConnection(db *gorm.DB) {
	dbSQL, err := db.DB()

	if err != nil {
		panic("Failed to close connection from database")
	}

	dbSQL.Close()
}
