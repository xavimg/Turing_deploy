package mysql

import (
	"fmt"

	"github.com/xavimg/Turing/golangserver/repository"
	"gorm.io/gorm"
)

type UserRepository struct {
	DB *gorm.DB
}

func NewUserRepository(db *gorm.DB) *UserRepository {
	return &UserRepository{DB: db}
}

func (repo *UserRepository) FindAll() ([]repository.User, error) {
	var users []repository.User
	err := repo.DB.Find(&users)
	if err == nil {
		return nil, err.Error
	} else {
		fmt.Println(err.RowsAffected)
	}
	return users, nil
}

func (repo *UserRepository) FindById(id int) (*repository.User, error) {
	return nil, nil
}
