package db

import (
	"time"

	"github.com/xavimg/Turing/golangserver/repository"
	"gorm.io/driver/mysql"
	"gorm.io/gorm"
)

func NewMysqlDB() *gorm.DB {

	// dns config
	dns := "root:@tcp(localhost:3307)/alan?charset=utf8mb4&parseTime=True&loc=Local"

	// mysql config
	mysqlConfig := mysql.Config{
		DSN:                       dns,
		DefaultStringSize:         256,
		DisableDatetimePrecision:  false,
		DontSupportRenameIndex:    true,
		DontSupportRenameColumn:   true,
		SkipInitializeWithVersion: false,
	}

	// gorm config
	gormConfig := gorm.Config{
		SkipDefaultTransaction: true,
		DryRun:                 false,
		PrepareStmt:            true,
	}

	db, err := gorm.Open(mysql.New(mysqlConfig), &gormConfig)
	if err != nil {
		panic(err)
	}

	// connection pool config
	if sqlDB, err := db.DB(); err == nil {
		sqlDB.SetMaxIdleConns(10)
		sqlDB.SetMaxOpenConns(100)
		sqlDB.SetConnMaxLifetime(time.Hour)
	}

	// migrate table
	db.AutoMigrate(
		&repository.User{})

	// seedData(db)

	return db
}

// func seedData(db *gorm.DB) {

// 	db.Where("1 = 1").Delete(&repository.User{})

// 	db.Create(&repository.User{Username: "tirmizee", Password: "123", Email: "tirmizee@hotmail.com"})
// 	db.Create(&repository.User{Username: "kiskdifw", Password: "123", Email: "kiskdifw@hotmail.com"})

// }
