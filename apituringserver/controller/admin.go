package controller

import (
	"net/http"

	"github.com/gin-gonic/gin"
	"github.com/xavimg/Turing/apituringserver/helper"
	"github.com/xavimg/Turing/apituringserver/service"
)

type AdminController interface {
	BanUser(ctx *gin.Context)
	UnbanUser(ctx *gin.Context)
}

type adminController struct {
	adminService service.AdminService
}

func NewAdminController(adminService service.AdminService) AdminController {
	return &adminController{
		adminService: adminService,
	}
}

func (c *adminController) BanUser(ctx *gin.Context) {
	userID := ctx.Param("id")

	c.adminService.BanUser(userID)

	res := helper.BuildResponse(true, "User has been banned !", helper.EmptyObj{})
	ctx.JSON(http.StatusOK, res)

}

func (c *adminController) UnbanUser(ctx *gin.Context) {
	userID := ctx.Param("id")

	c.adminService.UnbanUser(userID)

	res := helper.BuildResponse(true, "User has been unbanned !", helper.EmptyObj{})
	ctx.JSON(http.StatusOK, res)

}
