package main

import (
	"io"
	"net/http"
	"os"

	"github.com/gin-gonic/gin"
	"github.com/xavimg/Turing/APIrest/controller"
	"github.com/xavimg/Turing/APIrest/middlewares"
	"github.com/xavimg/Turing/APIrest/service"
)

var (
	playerService service.PlayerService = service.NewConstructorService()
	loginService  service.LoginService  = service.NewConstructorLogin()
	jwtService    service.JWTService    = service.NewJWTService()

	playerController controller.VideoController = controller.NewConstructorController(playerService)
	loginController  controller.LoginController = controller.NewLoginController(loginService, jwtService)
)

func main() {

	f, _ := os.Create("gin.log")

	gin.DefaultWriter = io.MultiWriter(f, os.Stdout)

	server := gin.New()

	server.Static("/css", "./templates/css")
	server.LoadHTMLGlob("templates/*.html")

	server.Use(gin.Recovery(), middlewares.Logger())

	server.POST("/login", func(c *gin.Context) {
		token := loginController.Login(c)
		if token == "" {
			c.JSON(http.StatusUnauthorized, nil)
			return
		}
		c.JSON(http.StatusOK, gin.H{
			"token": token,
		})
	})

	apiRoutes := server.Group("/api", middlewares.AuthorizeJWT())
	{
		apiRoutes.GET("/players", playerController.FindAll)
		apiRoutes.POST("/players", func(c *gin.Context) {
			err := playerController.Save(c)
			if err != nil {
				c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
				return
			}
			c.JSON(http.StatusOK, gin.H{"message": "Player input is Valid!!"})

		})
	}

	viewRoutes := server.Group("/viewRoutes")
	{
		viewRoutes.GET("/players", playerController.ShowAll)
	}

	server.Run()
}
