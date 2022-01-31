package repository

import (
	"log"

	"github.com/xavimg/Turing/apituringserver/entity"
	"golang.org/x/crypto/bcrypt"
	"gorm.io/gorm"
)

// UserRepository is a contract what UserRepository can do to db.
type UserRepository interface {
	InsertUser(user entity.User) entity.User
	UpdateUser(user entity.User, path string) entity.User
	VerifyCredential(email, password string) interface{}
	IsDuplicateEmail(email string) (ctx *gorm.DB)
	FindByEmail(username string) entity.User
	ProfileUser(userID string) entity.User
	SaveToken(user entity.User, token string)
}

type userConnection struct {
	connection *gorm.DB
}

// NewUserRepository is creates a new instance of UserRepository
func NewUserRepository(db *gorm.DB) UserRepository {
	return &userConnection{
		connection: db,
	}
}

func (db *userConnection) InsertUser(user entity.User) entity.User {
	user.Password = hashAndSalt([]byte(user.Password))

	db.connection.Save(&user)
	db.connection.Preload("Characters").Find(&user)

	return user
}

func (db *userConnection) UpdateUser(user entity.User, path string) entity.User {
	if user.Password != "" {
		user.Password = hashAndSalt([]byte(user.Password))
	} else {
		var tempUser entity.User
		db.connection.Find(&tempUser, user.ID)
		user.Password = tempUser.Password
	}

	db.connection.Save(&user)
	db.connection.Preload("Characters").Preload("Characters.User").Find(&user)

	return user
}

func hashAndSalt(pwd []byte) string {
	hash, err := bcrypt.GenerateFromPassword(pwd, bcrypt.MinCost)

	if err != nil {
		log.Println(err)
		panic("Failed to hash a password")
	}

	return string(hash)
}

func (db *userConnection) VerifyCredential(email string, password string) interface{} {
	var user entity.User

	res := db.connection.Where("email = ?", email).Take(&user)

	if res == nil {
		return res.Error
	}
	return user
}

func (db *userConnection) IsDuplicateEmail(email string) (tx *gorm.DB) {
	var user entity.User

	return db.connection.Where("email = ?", email).Take(&user)
}

func (db *userConnection) ProfileUser(userID string) entity.User {
	var user entity.User

	db.connection.Find(&user, userID)

	return user
}

func (db *userConnection) FindByEmail(username string) entity.User {
	var user entity.User

	db.connection.Where("email = ? ", username).Take(&user)

	return user
}

func (db *userConnection) SaveToken(user entity.User, token string) {

	user.Token = token

	db.connection.Save(&user)
}
