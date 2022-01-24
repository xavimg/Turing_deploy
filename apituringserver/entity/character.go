package entity

import "gorm.io/gorm"

type Character struct {
	gorm.Model
	Alias  string `gorm:"type:varchar(255)" json:"alias"`
	UserID uint64 `gorm:"not null" json:"-"`
	User   User   `gorm:"foreignKey:UserID;constraint:onUpdate:CASCADE,onDelete:CASCADE" json:"user"`
}
