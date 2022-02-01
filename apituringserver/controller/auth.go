package controller

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"log"
	"net/http"

	"github.com/gin-gonic/gin"
	"github.com/xavimg/Turing/apituringserver/dto"
	"github.com/xavimg/Turing/apituringserver/entity"
	"github.com/xavimg/Turing/apituringserver/helper"
	"github.com/xavimg/Turing/apituringserver/service"
)

// AuthController interface is a contract what this controller can do
type AuthController interface {
	Register(context *gin.Context)
	Login(context *gin.Context)
	Logout(context *gin.Context)
}

type authController struct {
	authService service.AuthService
	jwtService  service.JWTService
}
type JsonAndreba struct {
	Isvalid bool `json:"isvalid"`
}

// NewAuthController creates a new instance of AuthController
func NewAuthController(authService service.AuthService, jwtService service.JWTService) AuthController {
	return &authController{
		authService: authService,
		jwtService:  jwtService,
	}
}

func (c *authController) Login(context *gin.Context) {
	var loginDTO dto.LoginDTO

	// validation from request
	errDTO := context.ShouldBind(&loginDTO)
	if errDTO != nil {
		response := helper.BuildErrorResponse("User login failed", errDTO.Error(), helper.EmptyObj{})
		context.AbortWithStatusJSON(http.StatusBadRequest, response)
		return
	}

	// login successful
	authResult := c.authService.VerifyCredential(loginDTO.Email, loginDTO.Password)

	// var infoJson dto.DataAlex

	if v, ok := authResult.(entity.User); ok {
		generateToken := c.jwtService.GenerateTokenLogin(v.ID)
		v.Token = generateToken

		c.authService.SaveToken(v, generateToken)

		json_data, err := json.Marshal(generateToken)
		if err != nil {
			log.Fatal(err)
		}

		resp, err := http.Post("http://192.168.192.221:8080/internal/user/signin", "application/json", bytes.NewReader(json_data))
		if err != nil {
			log.Fatal(err)
		}
		defer resp.Body.Close()

		bodyBytes, err := io.ReadAll(resp.Body)
		if err != nil {
			log.Fatal(err)
		}
		bodyString := string(bodyBytes)
		fmt.Println("debug", bodyString)

		// context.SetCookie(generateToken, "testtoken", 3600, "/", "localhost", false, false)

		response := helper.BuildResponseSession(true, "User login successfully", generateToken)
		context.JSON(http.StatusOK, response)
		return
	}

	// login failed
	response := helper.BuildErrorResponse("User login failed", "Invalid credential", helper.EmptyObj{})
	context.AbortWithStatusJSON(http.StatusUnauthorized, response)

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
		// token := c.jwtService.GenerateTokenRegister(createdUser.ID)
		// createdUser.Token = token

		// Action where I send to Alex ID from user, so he can knows.
		// var infoJson dto.DataAlex

		// infoJson.ID = createdUser.ID
		// infoJson.Token = createdUser.Token

		json_data, err := json.Marshal(createdUser.ID)
		if err != nil {
			return
		}

		resp, err := http.Post("http://192.168.192.221:8080/internal/user/signup", "application/json", bytes.NewReader(json_data))
		if err != nil {
			log.Fatal(err)
		}
		defer resp.Body.Close()

		bodyBytes, err := io.ReadAll(resp.Body)
		if err != nil {
			log.Fatal(err)
		}
		bodyString := string(bodyBytes)
		fmt.Println("debug", bodyString)
		// Ending connection with Alex.

		response := helper.BuildResponse(true, "User register successfully", createdUser)

		context.JSON(http.StatusCreated, response.Data)
	}
}

func (c *authController) Logout(ctx *gin.Context) {

	id := ctx.Param("id")
	authResult := c.authService.VerifyUserExist(id)

	if v, ok := authResult.(entity.User); ok {

		c.authService.DeleteToken(v, "")

		// resp := c.authService.GetToken(id)
		// fmt.Println(resp.Token)
		// fmt.Println(resp.Name)
		// resp.Token = "null"

		// fmt.Println(resp.Token)
		// json_data, _ := json.Marshal(resp)

		//http.Post("http://192.168.192.221:8080/internal/user/signup", "application/json", bytes.NewReader(json_data))

	}
}
