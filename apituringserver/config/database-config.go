package config

import (
	"fmt"

	"github.com/xavimg/Turing/apituringserver/internal/entity"
	"gorm.io/driver/postgres"
	"gorm.io/gorm"

	_ "github.com/lib/pq"
)

// SetupDatabaseConnection is creating a new connection to our database
func SetupDatabaseConnection() *gorm.DB {

	postgresInfo := fmt.Sprintf("host=%s port=%d user=%s password=%s dbname=%s sslmode=disable",
		"postgres", 5432, "postgres", "v6vpxdkd", "alanturing")

	db, err := gorm.Open(postgres.Open(postgresInfo))

	if err != nil {
		panic("Failed to create a connection to database")
	}

	db.AutoMigrate(
		&entity.User{},
		&entity.Feature{},
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
