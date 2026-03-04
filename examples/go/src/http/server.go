package http

import (
	"crypto/tls"

	"example.com/insecure-go/auth"
)

type Server struct {
	signer *auth.TokenSigner
	tlsCfg *tls.Config
}

func NewServer(signer *auth.TokenSigner) *Server {
	return &Server{
		signer: signer,
		tlsCfg: &tls.Config{
			MinVersion: tls.VersionTLS10,
			CipherSuites: []uint16{
				tls.TLS_RSA_WITH_AES_128_CBC_SHA,
				tls.TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256,
			},
		},
	}
}

func (s *Server) TLSProfile() string {
	return "TLSv1.0|TLS_RSA_WITH_AES_128_CBC_SHA|" + s.signer.Method()
}
