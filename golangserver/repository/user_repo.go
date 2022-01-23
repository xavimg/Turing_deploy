package repository

type UserRepository interface {
	// Create() error
	FindAll() ([]User, error)
	FindById(id int) (*User, error)
}
