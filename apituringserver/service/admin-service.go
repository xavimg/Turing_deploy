package service

import (
	"log"

	"github.com/mashingan/smapping"
	"github.com/xavimg/Turing/apituringserver/dto"
	"github.com/xavimg/Turing/apituringserver/entity"
	"github.com/xavimg/Turing/apituringserver/repository"
)

type AdminService interface {
	BanUser(userID string)
	UnbanUser(userID string)
	NewFeature(feature dto.FeatureDTO) entity.Feature
}

type adminService struct {
	adminRepository repository.AdminRepository
}

func NewAdminService(adminRepo repository.AdminRepository) AdminService {
	return &adminService{
		adminRepository: adminRepo,
	}
}

func (service *adminService) BanUser(userID string) {

	service.adminRepository.BanUser(userID)
}

func (service *adminService) UnbanUser(userID string) {

	service.adminRepository.UnbanUser(userID)
}

func (service *adminService) NewFeature(feature dto.FeatureDTO) entity.Feature {

	featureToCreate := entity.Feature{}

	err := smapping.FillStruct(&featureToCreate, smapping.MapFields(&feature))
	if err != nil {
		log.Fatalf("Failed map %v", err)
	}

	res := service.adminRepository.NewFeature(featureToCreate)

	return res
}
