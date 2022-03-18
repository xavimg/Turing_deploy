package controller

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"log"
	"net/http"
	"strconv"
	"sync"

	"github.com/gin-gonic/gin"
	"github.com/xavimg/Turing/apituringserver/internal/dto"
	"github.com/xavimg/Turing/apituringserver/internal/entity"
	"github.com/xavimg/Turing/apituringserver/internal/helper"
	"github.com/xavimg/Turing/apituringserver/internal/service"
)

// AuthController interface is a contract what this controller can do
type AuthController interface {
	Register(context *gin.Context)
	Login(context *gin.Context)
	Logout(context *gin.Context)
	VerifyCode(context *gin.Context)
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
	if errDTO := context.ShouldBindJSON(&loginDTO); errDTO != nil {
		response := helper.BuildErrorResponse("User login failed", errDTO.Error(), helper.EmptyObj{})
		context.AbortWithStatusJSON(http.StatusBadRequest, response)
		return
	}

	// Verify of credentials exists
	authResult := c.authService.VerifyCredential(loginDTO.Email, loginDTO.Password)
	if v, ok := authResult.(entity.User); ok {
		if strconv.FormatBool(v.Active) == "false" {
			context.JSON(http.StatusBadRequest, "User has been banned")
			return

		}

		generateToken := c.jwtService.GenerateTokenLogin(v.ID)
		v.Token = fmt.Sprintf("Bearer %v", generateToken)
		c.authService.SaveToken(v, fmt.Sprintf("Bearer %v", generateToken))

		/*json_data, err := json.Marshal(fmt.Sprintf("Bearer %v", generateToken))
		if err != nil {
			log.Fatal(err)
		}*/

		client := &http.Client{}
		req, err := http.NewRequest("POST", "http://192.168.195.80:8080/player/signin", bytes.NewReader([]byte(generateToken)))
		req.Header.Add("Authorization", fmt.Sprintf("Bearer %v", generateToken))
		resp, err := client.Do(req)

		/*resp, err := http.Post("http://192.168.195.80:8080/player/signin", "application/json", bytes.NewReader(json_data))
		if err != nil {
			log.Fatal(err)
		}
		defer resp.Body.Close()*/

		bodyBytes, err := io.ReadAll(resp.Body)
		if err != nil {
			log.Fatal(err)
		}
		bodyString := string(bodyBytes)
		fmt.Println("debug", bodyString)

		response := helper.BuildResponseSession(true, "User login successfully", generateToken)
		context.JSON(http.StatusOK, response)
		return
	}

	response := helper.BuildErrorResponse("User login failed", "Invalid credential", helper.EmptyObj{})
	context.AbortWithStatusJSON(http.StatusUnauthorized, response)

}

func (c *authController) Register(context *gin.Context) {
	var registerDTO dto.RegisterDTO

	if errDTO := context.ShouldBind(&registerDTO); errDTO != nil {
		response := helper.BuildErrorResponse("User register failed", errDTO.Error(), helper.EmptyObj{})
		context.AbortWithStatusJSON(http.StatusBadRequest, response)
		return
	}

	if !c.authService.IsDuplicateEmail(registerDTO.Email) {
		response := helper.BuildErrorResponse("User register failed", "Duplicate email", helper.EmptyObj{})
		context.JSON(http.StatusConflict, response)
		return
	} else {

		getCode := service.SendEmailCodeVerify(registerDTO.Name, registerDTO.Email)

		createdUser := c.authService.CreateUser(registerDTO, getCode)

		token := c.jwtService.GenerateTokenRegister(createdUser.ID)
		createdUser.Token = fmt.Sprintf("Bearer %v", token)

		// Action where I send to Alex ID from user, so he can knows.
		var infoJson dto.DataAlex

		infoJson.ID = createdUser.ID
		infoJson.Token = createdUser.Token

		json_data, err := json.Marshal(createdUser.ID)
		if err != nil {
			return
		}
		resp, err := http.Post("http://192.168.195.80:8080/player/signup", "application/json", bytes.NewReader(json_data))
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

		var routine sync.Mutex
		routine.Lock()
		go service.SendEmail(registerDTO.Name, registerDTO.Email)
		routine.Unlock()

		response := helper.BuildResponse(true, "Check your email !", createdUser)

		context.JSON(http.StatusCreated, response)
	}
}

func (c *authController) Logout(ctx *gin.Context) {

	id := ctx.Param("id")

	authResult := c.authService.VerifyUserExist(id)
	if v, ok := authResult.(entity.User); ok {

		response := c.authService.GetToken(id)

		json_data, _ := json.Marshal(response.Token)

		resp, err := http.Post("http://192.168.195.80:8080/player/signout", "application/json", bytes.NewReader(json_data))
		if err != nil {
			log.Fatal(err)
		}
		defer resp.Body.Close()

		bodyBytes, err := io.ReadAll(resp.Body)
		if err != nil {
			log.Fatal("Error reading response", err)
		}

		bodyString := string(bodyBytes)
		if bodyString != "true" {

			log.Fatal()
			return

		}

		c.authService.DeleteToken(v, "")

	}
}

func (c *authController) VerifyCode(ctx *gin.Context) {
	var req dto.CodeVerifyDTO

	if err := ctx.ShouldBindJSON(&req); err != nil {
		log.Fatal("Error binding")
		return
	}

	if req.Email == "" {
		log.Println("email and code are required")
		return
	}
	if req.Code <= 0 {
		log.Println("email and code are required")
		return
	}

	exist, err := c.authService.VerifyCode(req.Email, req.Code)
	if err != nil {
		log.Println("Error: ", err)
		return
	}

	if !exist {
		ctx.JSON(http.StatusBadRequest, "invalid code !")
		return
	}

	ctx.JSON(http.StatusOK, "you've been verified !")
}