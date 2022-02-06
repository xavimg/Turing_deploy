package service

import (
	"github.com/xavimg/Turing/apituringserver/repository"
)

type AdminService interface {
	BanUser(userID string)
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
