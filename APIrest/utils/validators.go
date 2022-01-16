package validators

import (
	"strings"

	"github.com/go-playground/validator"
)

func ValidateCoolUsername(field validator.FieldLevel) bool {
	return strings.Contains(field.Field().String(), "Cool")
}
