package database

import (
	"database/sql"

	_ "github.com/go-sql-driver/mysql"
)

type MySqlClient struct {
	*sql.DB
}

func MySqlClient() *MySqlClient {
	db, err := sql.Open("mysql", "root:root@tcp(localhost:3306)/alanturing")

	if err != nil {
		panic(err)
	}

	err = db.Ping()

	if err != nil {
	}
}
