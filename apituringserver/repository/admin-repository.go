package repository

import (
	"github.com/xavimg/Turing/apituringserver/entity"
	"gorm.io/gorm"
)

type AdminRepository interface {
	BanUser(user entity.User, userID string) entity.User
	UnbanUser(userID string) entity.User
	NewFeature(feature entity.Feature) entity.Feature
}

type adminConnection struct {
	connection *gorm.DB
}

func newAdminRepository(dbConn *gorm.DB) AdminRepository {
	return &adminConnection{
		connection: dbConn}
}

func (db *adminConnection) BanUser(user entity.User, userID string) entity.User {

	user.Active = false

	db.connection.Save(&user)

	return user
}

func (db *adminConnection) UnbanUser(userID string) entity.User {
	return entity.User{}
}

func (db *adminConnection) NewFeature(feature entity.Feature) entity.Feature {
	return entity.Feature{}
}
