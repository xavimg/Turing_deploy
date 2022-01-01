package api

import "github.com/xavimg/Turing/BackenWebV2/internal/database"

type Services struct {
	search PlayerSearch
	users  UserGateway
}

func NewServices() Services {
	client := database.NewMySQLClient()
	return Services{
		search: &PlayerService{client},
		users:  &UserService{client},
	}
}

type WebServices struct {
	Services
}

func start() *WebServices {
	return &WebServices{NewServices()}
}
