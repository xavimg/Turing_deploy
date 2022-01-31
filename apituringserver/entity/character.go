package entity

type Character struct {
	ID     uint64 `gorm:"primary_key:auto_increment" json:"id"`
	Alias  string `gorm:"type:varchar(255)" json:"alias"`
	UserID uint64 `gorm:"not null" json:"-"`
	User   User   `gorm:"foreignKey:UserID;constraint:onUpdate:CASCADE,onDelete:CASCADE" json:"user"`
}
