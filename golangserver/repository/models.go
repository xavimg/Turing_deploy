package repository

import "gorm.io/gorm"

type BaseModel struct {
	gorm.Model
}

type User struct {
	BaseModel
	Username string `gorm:"column:username"`
	Email    string `gorm:"column:email"`
	Password string `gorm:"column:password"`
}
