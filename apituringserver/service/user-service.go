package service

import (
	"github.com/xavimg/Turing/apituringserver/entity"
	"github.com/xavimg/Turing/apituringserver/repository"
)

// UserService is a contract about something that this service can do
type UserService interface {
	Profile(userID string) entity.User
	// Update(user dto.UserUpdateDTO, path string) entity.User
}

type userService struct {
	userRepository repository.UserRepository
}

// NewUserService creates a new instance of UserService
func NewUserService(userRepo repository.UserRepository) UserService {
	return &userService{
		userRepository: userRepo,
	}
}

func (service *userService) Profile(userID string) entity.User {
	return service.userRepository.ProfileUser(userID)
}

// func (service *userService) Update(user dto.UserUpdateDTO, path string) entity.User {
// 	userToUpdate := entity.User{}

// 	err := smapping.FillStruct(&userToUpdate, smapping.MapFields(&user))

// 	if err != nil {
// 		log.Fatalf("Failed map %v : ", err)
// 	}

// 	updatedUser := service.userRepository.UpdateUser(userToUpdate, path)

// 	return updatedUser
// }
