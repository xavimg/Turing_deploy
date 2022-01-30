package controller

import (
	"fmt"
	"net/http"

	"github.com/dgrijalva/jwt-go"
	"github.com/gin-gonic/gin"
	"github.com/xavimg/Turing/apituringserver/helper"
	"github.com/xavimg/Turing/apituringserver/service"
)

type UserController interface {
	Profile(context *gin.Context)
	// Update(context *gin.Context)
}

type userController struct {
	userService service.UserService
	jwtService  service.JWTService
}

func NewUserController(userService service.UserService, jwtService service.JWTService) UserController {
	return &userController{
		userService: userService,
		jwtService:  jwtService,
	}
}

func (c *userController) Profile(ctx *gin.Context) {
	authHeader := ctx.GetHeader("Authorization")
	token, err := c.jwtService.ValidateToken(authHeader)
	if err != nil {
		panic(err.Error())
	}

	claims := token.Claims.(jwt.MapClaims)
	userID := fmt.Sprintf("%v", claims["user_id"])

	// throw to service
	user := c.userService.Profile(userID)

	// response
	res := helper.BuildResponse(true, "Get user profile successfully", user)
	ctx.JSON(http.StatusOK, res)
}

// func (c *userController) Update(ctx *gin.Context) {
// 	var userUpdateDTO dto.UserUpdateDTO

// }
