package dto

type RegisterDTO struct {
	Name     string `json:"name" form:"name" binding:"required,min=5,max=6"`
	Email    string `json:"email" form:"email" binding:"required,email"`
	Password string `json:"password" form:"password" binding:"required,min=6"`
}