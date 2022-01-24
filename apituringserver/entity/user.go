package entity

import "gorm.io/gorm"

type User struct {
	gorm.Model
	Name       string       `gorm:"type:varchar(255)" json:"name"`
	Email      string       `gorm:"uniqueIndex;type:varchar(255)" json:"email"`
	Password   string       `gorm:"->;<-;not null" json:"-"`
	Token      string       `gorm:"-" json:"token,omitempty"`
	Characters *[]Character `json:"characters,omitempty"`
	// Creditcard CreditCard `gorm:"foreingKey:UserName"`
	// Languages []Language  `gorm:"many2many:user_languages"`
	// Character []Character `gorm:"many2many:user_character"`
}

// type CreditCard struct {
// 	Number   string
// 	UserName string
// }

// type Language struct {
// 	gorm.Model
// 	Name string
// }
// type Character struct {
// 	gorm.Model
// 	Name string
// 	Type string
// }
