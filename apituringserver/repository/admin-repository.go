package repository

import (
	"github.com/xavimg/Turing/apituringserver/entity"
	"gorm.io/gorm"
)

type AdminRepository interface {
	BanUser(userID string)
	UnbanUser(userID string) entity.User
	NewFeature(feature entity.Feature) entity.Feature
}

type adminConnection struct {
	connection *gorm.DB
}

func NewAdminRepository(dbConn *gorm.DB) AdminRepository {
	return &adminConnection{
		connection: dbConn}
}

func (db *adminConnection) BanUser(userID string) {
	var user entity.User
	db.connection.Model(user).Where("id = ?", userID).Update("active", false)
}

func (db *adminConnection) UnbanUser(userID string) entity.User {
	return entity.User{}
}

func (db *adminConnection) NewFeature(feature entity.Feature) entity.Feature {
	return entity.Feature{}
}
