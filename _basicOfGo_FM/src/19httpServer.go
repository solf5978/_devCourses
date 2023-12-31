package main

import (
	"fmt"
	"html/template"
	"net/http"
)

func handleHello(w http.ResponseWriter, r *http.Request) {
	w.Write([]byte("Hello From A Go Program"))
}

func handleTemplate(w http.ResponseWriter, r *http.Request) {
	html, err := template.ParseFiles("templates.index.tmpl")
	if err != nil {
		w.Write([]byte("Internal Server Error"))
		w.WriteHeader(http.StatusInternalServerError)
		return
	} else {
		html.Execute(w, List[0])
	}
}

func main() {
	server := http.NewServeMux()
	server.HandleFunc(
		"/", handleHello)
	server.HandleFunc("/template", handleTemplate)
	server.HandleFunc("/exhibitions", Get)
	server.HandleFunc(("/exhibitions/new", Post))
	fs := http.FileServer(http.Dir("./public"))
	server.Handle("/", fs)
	err := http.ListenAndServe(":3333", nil)
	if err != nil {
		return
	} else {
		fmt.Println("No Response From The Server")
	}
}
