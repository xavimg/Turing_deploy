package service

import (
	"log"

	"github.com/mashingan/smapping"
	"github.com/xavimg/Turing/apituringserver/dto"
	"github.com/xavimg/Turing/apituringserver/entity"
	"github.com/xavimg/Turing/apituringserver/repository"
	"golang.org/x/crypto/bcrypt"
)

type AuthService interface {
	CreateUser(user dto.RegisterDTO) entity.User
	VerifyCredential(email, password string) interface{}
	FindByEmail(email string) entity.User
	IsDuplicateEmail(email string) bool
}

type authService struct {
	userRepository repository.UserRepository
}

func NewAuthService(userRepo repository.UserRepository) AuthService {
	return &authService{
		userRepository: userRepo,
	}
}

func (service *authService) CreateUser(user dto.RegisterDTO) entity.User {
	userToCreate := entity.User{}

	err := smapping.FillStruct(&userToCreate, smapping.MapFields(&user))
	if err != nil {
		log.Fatalf("Failed map %v", err)
	}

	res := service.userRepository.InsertUser(userToCreate)

	return res
}

func (service *authService) VerifyCredential(email, password string) interface{} {
	res := service.userRepository.VerifyCredential(email, password)

	if v, ok := res.(entity.User); ok {

		comparedPassword := comparePassword(v.Password, []byte(password))

		if v.Email == email && comparedPassword {
			return res
		}
		return false
	}

	return false
}

func (service *authService) IsDuplicateEmail(email string) bool {
	res := service.userRepository.IsDuplicateEmail(email)

	return !(res.Error == nil)
}

func comparePassword(hashedPwd string, plainPassword []byte) bool {
	byteHash := []byte(hashedPwd)

	err := bcrypt.CompareHashAndPassword(byteHash, plainPassword)

	if err != nil {
		log.Println(err)
		return false
	}

	return true
}

func (service *authService) FindByEmail(email string) entity.User {
	return service.userRepository.FindByEmail(email)
}
