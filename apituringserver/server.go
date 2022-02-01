package main

import (
	"github.com/gin-gonic/gin"
	"github.com/xavimg/Turing/apituringserver/config"
	"github.com/xavimg/Turing/apituringserver/controller"
	"github.com/xavimg/Turing/apituringserver/middleware"
	"github.com/xavimg/Turing/apituringserver/repository"
	"github.com/xavimg/Turing/apituringserver/service"
	"gorm.io/gorm"
)

var (
	db             *gorm.DB                  = config.SetupDatabaseConnection()
	userRepository repository.UserRepository = repository.NewUserRepository(db)

	jwtService  service.JWTService  = service.NewJWTService()
	userService service.UserService = service.NewUserService(userRepository)

	authService    service.AuthService       = service.NewAuthService(userRepository)
	authController controller.AuthController = controller.NewAuthController(authService, jwtService)
	userController controller.UserController = controller.NewUserController(userService, jwtService)
)

func main() {
	r := gin.Default()

	r.MaxMultipartMemory = 8 << 20

	// public routes
	authRoutes := r.Group("api/auth")
	{
		authRoutes.POST("/login", authController.Login)
		authRoutes.POST("/register", authController.Register)
		authRoutes.POST("/logout/:id", authController.Logout)
	}

	// private/tokenized routes
	userRoutes := r.Group("api/user", middleware.AuthorizeJWT(jwtService))
	{
		userRoutes.GET("/profile", userController.Profile)
	}

	r.Run(":3000")

}
