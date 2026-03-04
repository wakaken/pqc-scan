package main

import (
	"log"

	"example.com/insecure-go/auth"
	apphttp "example.com/insecure-go/http"
)

func main() {
	signer := auth.NewTokenSigner("RS512")
	server := apphttp.NewServer(signer)
	log.Println("starting gateway with", server.TLSProfile())
}
