package entity

import "time"

type Player struct {
	ID       uint64 `json:"id"`
	Username string `json:"username" binding:"required,min=2,max=15" validate:"is-cool" gorm:"type:varchar(32)"`
	Email    string `json:"email" binding:"required,email" gorm:"type:varchar(32)"`
	Password string `json:"password" binding:"required,min=8,max=20" gorm:"type:varchar(20)"`
}

type Character struct {
	ID        uint64    `json:"id" gorm:"primary_key;auto_increment"`
	Alias     string    `json:"alias" binding:"required" gorm:"type:varchar(20)"`
	Origin    string    `json:"origin" binding:"required" gorm:"type:varchar(10)"`
	Owner     Player    `json:"owner" binding:"required" gorm:"foreignkey:PersonID"`
	PersonID  uint64    `json:"-"`
	CreatedAt time.Time `json:"-" gorm:"default:CURRENT_TIMESTAMP" json:"created_at"`
	UpdatedAt time.Time `json:"-" gorm:"default:CURRENT_TIMESTAMP" json:"updated_at"`
}
