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
	db              *gorm.DB                   = config.SetupDatabaseConnection()
	userRepository  repository.UserRepository  = repository.NewUserRepository(db)
	adminRepository repository.AdminRepository = repository.NewAdminRepository(db)

	jwtService   service.JWTService   = service.NewJWTService()
	userService  service.UserService  = service.NewUserService(userRepository)
	authService  service.AuthService  = service.NewAuthService(userRepository)
	adminService service.AdminService = service.NewAdminService(adminRepository)

	authController  controller.AuthController  = controller.NewAuthController(authService, jwtService)
	userController  controller.UserController  = controller.NewUserController(userService, jwtService)
	adminController controller.AdminController = controller.NewAdminController(adminService)
)

func main() {

	r := gin.Default()

	r.MaxMultipartMemory = 8 << 20

	r.GET("/hello", func(c *gin.Context) {

		c.JSON(200, gin.H{
			"message": "Hello",
		})

	})

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
		userRoutes.PUT("/update", userController.Update)
	}

	// admin Routes
	adminRoutes := r.Group("api/admin")
	{
		adminRoutes.PUT("/ban/:id", adminController.BanUser)
		adminRoutes.PUT("/unban/:id", adminController.UnbanUser)
		adminRoutes.POST("/newfeature", adminController.NewFeature)
	}

	r.Run(":3000")

}
