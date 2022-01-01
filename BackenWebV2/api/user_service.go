package api

import (
	"github.com/xavimg/Turing/BackenWebV2/internal/database"
	"github.com/xavimg/Turing/BackenWebV2/internal/logs"
)

type RegisterUserBody struct {
	Username       string `json:"username"`
	Email          string `json:"email"`
	Birthday       string `json:"birthday"`
	Genre          string `json:"genre"`
	Country        string `json:"country"`
	Password       string `json:"password"`
	RepeatPassword string `json:"repeat_password"`
}

type UserGateway interface {
	RegisterUser(u RegisterUserBody) (string, error)
	// LoginUser()
}

type UserService struct {
	*database.MySqlClient
}

func (us *UserService) RegisterUser(u RegisterUserBody) (string, error) {

	_, err := us.Exec(RegisterUserQuery(), u.Username, u.Email, u.Birthday, u.Genre, u.Country, u.Password, u.RepeatPassword)
	if err != nil {
		logs.Error("cannot insert user" + err.Error())
		return "", err
	}
	logs.Info("New user registered")
	return u.Username, nil
}

func (us *UserService) Login() {

}
