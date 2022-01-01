package api

type Services struct {
	search PlayerSearch
}

func NewServices() Services {
	return Services{
		search: &PlayerService{},
	}
}

type WebServices struct {
	s Services
}

func start() *WebServices {
	return &WebServices{s: NewServices()}
}
