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
	// UpdateUser(user entity.User, path string) entity.User
	VerifyCredential(email, password string) interface{}
	// IsDuplcatedEmail(email string) (ctx *gorm.DB)
	// FindByUsername(username string) entity.User
	// ProfileUser(userID string) entity.User
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

	if res != nil {
		return res.Error
	}

	return res
}
