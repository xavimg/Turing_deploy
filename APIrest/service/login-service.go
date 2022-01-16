package service

type LoginService interface {
	Login(username string, password string) bool
}

type loginService struct {
	authorizedUsername string
	authorizedPassword string
}

func NewConstructorLogin() LoginService {
	return &loginService{
		authorizedUsername: "xavi",
		authorizedPassword: "guapo",
	}
}

func (l *loginService) Login(username string, password string) bool {
	return l.authorizedUsername == username && l.authorizedPassword == password
}
