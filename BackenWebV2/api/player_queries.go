package api

import "strings"

func getPlayersQuery(filter PlayerFilter) string {
	var (
		d, g, t string
		clause  = false
		query   = "select * from player"
		builder = strings.Builder{}
	)

	builder.WriteString(query)

	if filter.Username != "" {
		d = "username like '%" + filter.Username + "%'"
		clause = true
	}

	if filter.Level != "" {
		g = "level like '%" + filter.Level + "%'"
		clause = true
	}

	if clause {
		var i int
		builder.WriteString(" where ")
		if d != "" {
			builder.WriteString(d)
			i = 1
		}

		if g != "" {
			if i == 1 {
				builder.WriteString(" or ")
			}
			builder.WriteString(g)
			i = 2
		}

		if t != "" {
			if i == 1 || i == 2 {
				builder.WriteString(" or ")
			}
			builder.WriteString(t)
		}

		return builder.String()
	} else {
		return builder.String()
	}
}

func RegisterUserQuery() string {
	return "insert into user (username,email,birthday,genre,country, password, repeat_password) value (?,?,?,?,?,?,?)"
}
