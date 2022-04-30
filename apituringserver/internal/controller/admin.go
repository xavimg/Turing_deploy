package controller

import (
	"fmt"
	"net/http"

	"github.com/gin-gonic/gin"
	"github.com/xavimg/Turing/apituringserver/internal/dto"
	"github.com/xavimg/Turing/apituringserver/internal/entity"
	"github.com/xavimg/Turing/apituringserver/internal/helper"
	"github.com/xavimg/Turing/apituringserver/internal/service"
)

type AdminController interface {
	AdminLogin(ctx *gin.Context)
	ListAllUsersByParameter(ctx *gin.Context)
	BanUser(ctx *gin.Context)
	UnbanUser(ctx *gin.Context)
	NewFeature(ctx *gin.Context)
}

type adminController struct {
	adminService service.AdminService
	authService  service.AuthService
	jwtService   service.JWTService
}

func NewAdminController(adminService service.AdminService, authService service.AuthService, jwtService service.JWTService) AdminController {
	return &adminController{
		adminService: adminService,
		authService:  authService,
		jwtService:   jwtService,
	}
}

func (c *adminController) AdminLogin(context *gin.Context) {
	var loginDTO dto.LoginDTO
	if errDTO := context.ShouldBindJSON(&loginDTO); errDTO != nil {
		response := helper.BuildErrorResponse("admin login failed", errDTO.Error(), helper.EmptyObj{})
		context.AbortWithStatusJSON(http.StatusBadRequest, response.Message)
		return
	}

	authResult := c.authService.VerifyCredential(loginDTO.Email, loginDTO.Password)
	if v, ok := authResult.(entity.User); ok {
		if v.TypeUser != "admin" {
			context.JSON(http.StatusBadRequest, "admin doesn't exists")
			return
		}

		generateToken := c.jwtService.GenerateTokenLogin(v.ID)
		v.Token = fmt.Sprintf("Bearer %v", generateToken)
		c.authService.SaveToken(v, fmt.Sprintf("Bearer %v", generateToken))

		response := helper.BuildResponseSession(true, "admin login successfully", generateToken)
		context.JSON(http.StatusOK, response)
		return
	}

	response := helper.BuildErrorResponse("admin login failed", "Invalid credential", helper.EmptyObj{})
	context.AbortWithStatusJSON(http.StatusUnauthorized, response)

}

func (c *adminController) ListAllUsersByParameter(ctx *gin.Context) {
	tUser := ctx.Param("typeUser")
	var users []entity.User

	switch tUser {
	case "all":
		users = c.adminService.ListAllUsers()
	case "ban":
		users = c.adminService.ListAllUsersByActive()
	case "admin":
		users = c.adminService.ListAllUsersByTypeAdmin()
	case "user":
		users = c.adminService.ListAllUsersByTypeUser()
	default:
		ctx.JSON(http.StatusBadRequest, nil)
	}

	ctx.JSON(http.StatusOK, users)
}

// BanUser godoc
// @Title BanUser
// @Description  Admin ban user for X time.
// @Param Authorization header string true "Token acces admin"
// @Param id path string true "ID from query"
// @Tags Admin
// @Success      200 {object} helper.Response
// @Failure      400 body is empty or missing param
// @Failure      500 "internal server error"
// @Router       /api/admin/ban/{id} [put]
func (c *adminController) BanUser(ctx *gin.Context) {
	userID := ctx.Param("id")

	c.adminService.BanUser(userID)

	res := helper.BuildResponse(true, "User has been banned !", helper.EmptyObj{})
	ctx.JSON(http.StatusOK, res)

}

// UnbanUser godoc
// @Title UnbanUser
// @Description  Admin unban user.
// @Param Authorization header string true "Token acces admin"
// @Param id path string true "ID from query"
// @Tags Admin
// @Success      200 {object} helper.Response
// @Failure      400 body is empty or missing param
// @Failure      500 "internal server error"
// @Router       /api/admin/unban/{id} [put]
func (c *adminController) UnbanUser(ctx *gin.Context) {
	userID := ctx.Param("id")

	c.adminService.UnbanUser(userID)

	res := helper.BuildResponse(true, "User has been unbanned !", helper.EmptyObj{})
	ctx.JSON(http.StatusOK, res)

}

// NewFeature godoc
// @Title NewFeature
// @Description  Admin add new feature to show in version of game info.
// @Param Authorization header string true "Token acces admin"
// @Param request body dto.FeatureDTO true "Body to write new features"
// @Tags Admin
// @Success      200 {object} helper.Response
// @Failure      400 body is empty or missing param
// @Failure      500 "internal server error"
// @Router       /api/admin/newfeature [post]
func (c *adminController) NewFeature(ctx *gin.Context) {
	var feature dto.FeatureDTO

	if err := ctx.ShouldBind(&feature); err != nil {
		res := helper.BuildErrorResponse(
			"Feature not created", err.Error(),
			helper.EmptyObj{})
		ctx.AbortWithStatusJSON(http.StatusBadRequest, res)
		return
	}

	featureCreated := c.adminService.NewFeature(feature)

	response := helper.BuildResponse(true, "Feature has been created", featureCreated)

	ctx.JSON(http.StatusCreated, response.Data)
}
