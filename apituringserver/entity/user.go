package entity

type User struct {
	ID         uint64       `gorm:"primary_key:auto_increment" json:"id"`
	Name       string       `gorm:"type:varchar(255)" json:"name"`
	Email      string       `gorm:"uniqueIndex;type:varchar(255)" json:"email"`
	Password   string       `gorm:"->;<-;not null" json:"-"`
	Token      string       `gorm:"type:varchar(255)" json:"token,omitempty"`
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
