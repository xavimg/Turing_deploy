package controller

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"log"
	"net/http"
	"strconv"

	"github.com/gin-gonic/gin"
	"github.com/xavimg/Turing/apituringserver/dto"
	"github.com/xavimg/Turing/apituringserver/helper"
	"github.com/xavimg/Turing/apituringserver/service"
)

type JsonAndreba struct {
	Isvalid bool `json:"isvalid"`
}

// AuthController interface is a contract what this controller can do
type AuthController interface {
	// Login(context *gin.Context)
	Register(context *gin.Context)
}

type authController struct {
	authService service.AuthService
	jwtService  service.JWTService
}

// NewAuthController creates a new instance of AuthController
func NewAuthController(authService service.AuthService, jwtService service.JWTService) AuthController {
	return &authController{
		authService: authService,
		jwtService:  jwtService,
	}
}

func (c *authController) Register(context *gin.Context) {
	var registerDTO dto.RegisterDTO

	// validation form request
	errDTO := context.ShouldBind(&registerDTO)

	if errDTO != nil {
		response := helper.BuildErrorResponse("User register failed", errDTO.Error(), helper.EmptyObj{})
		context.AbortWithStatusJSON(http.StatusBadRequest, response)
		return
	}

	// check duplicate email
	if !c.authService.IsDuplicateEmail(registerDTO.Email) {
		response := helper.BuildErrorResponse("User register failed", "Duplicate email", helper.EmptyObj{})
		context.JSON(http.StatusConflict, response)
	} else {

		createdUser := c.authService.CreateUser(registerDTO)

		json_data, err := json.Marshal(createdUser.ID)
		if err != nil {
			return
		}

		fmt.Println(json_data)

		resp, err := http.Post("http://192.168.139.195:8080/internal/user", "application/json", bytes.NewReader(json_data))
		if err != nil {
			log.Fatal("nil1", err)
		}
		defer resp.Body.Close()

		bodyBytes, err := io.ReadAll(resp.Body)
		if err != nil {
			log.Fatal("nil2", err)
		}
		bodyString := string(bodyBytes)
		fmt.Println("debug", bodyString)

		token := c.jwtService.GenerateToken(strconv.FormatUint(uint64(createdUser.ID), 10))
		createdUser.Token = token
		response := helper.BuildResponse(true, "User register successfully", createdUser)

		context.JSON(http.StatusCreated, response.Data)
	}
}
