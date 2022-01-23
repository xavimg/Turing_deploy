package user

import "github.com/xavimg/Turing/golangserver/repository"

type UserService interface {
	AllUser() ([]UserRes, error)
}

type userService struct {
	userRepo repository.UserRepository
}

func NewUserService(u repository.UserRepository) *userService {
	return &userService{
		userRepo: u,
	}
}

func (s *userService) AllUser() ([]UserRes, error) {

	users, err := s.userRepo.FindAll()
	if err != nil {
		return nil, err
	}

	res := make([]UserRes, 0)
	for _, user := range users {
		item := UserRes{
			Id:       user.ID,
			Username: user.Username,
			Email:    user.Email,
			Password: user.Password,
		}
		res = append(res, item)
	}

	return res, nil
}
