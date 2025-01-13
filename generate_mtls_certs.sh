#!/bin/bash

# Create directories
mkdir -p ssl/ca
mkdir -p ssl/server
mkdir -p ssl/client

# Generate CA private key and certificate
openssl genpkey -algorithm RSA -out ssl/ca/ca_key.pem -pkeyopt rsa_keygen_bits:4096
openssl req -x509 -new -nodes -key ssl/ca/ca_key.pem -sha256 -days 1825 -out ssl/ca/ca_cert.pem \
    -subj "/C=US/ST=State/L=City/O=Organization/OU=CA/CN=My CA"

# Generate server private key and CSR
openssl genpkey -algorithm RSA -out ssl/server/server_key.pem -pkeyopt rsa_keygen_bits:4096
openssl req -new -key ssl/server/server_key.pem -out ssl/server/server.csr \
    -subj "/C=US/ST=State/L=City/O=Organization/OU=Server/CN=localhost"

# Generate client private key and CSR
openssl genpkey -algorithm RSA -out ssl/client/client_key.pem -pkeyopt rsa_keygen_bits:4096
openssl req -new -key ssl/client/client_key.pem -out ssl/client/client.csr \
    -subj "/C=US/ST=State/L=City/O=Organization/OU=Client/CN=client"

# Create server certificate extensions file
cat > ssl/server/server_ext.cnf << EOF
basicConstraints = CA:FALSE
keyUsage = digitalSignature, keyEncipherment
extendedKeyUsage = serverAuth
subjectAltName = @alt_names

[alt_names]
DNS.1 = localhost
IP.1 = 127.0.0.1
EOF

# Create client certificate extensions file
cat > ssl/client/client_ext.cnf << EOF
basicConstraints = CA:FALSE
keyUsage = digitalSignature, keyEncipherment
extendedKeyUsage = clientAuth
EOF

# Sign server CSR
openssl x509 -req -in ssl/server/server.csr \
    -CA ssl/ca/ca_cert.pem -CAkey ssl/ca/ca_key.pem -CAcreateserial \
    -out ssl/server/server_cert.pem -days 825 -sha256 \
    -extfile ssl/server/server_ext.cnf

# Sign client CSR
openssl x509 -req -in ssl/client/client.csr \
    -CA ssl/ca/ca_cert.pem -CAkey ssl/ca/ca_key.pem -CAcreateserial \
    -out ssl/client/client_cert.pem -days 825 -sha256 \
    -extfile ssl/client/client_ext.cnf

# Set correct permissions
chmod 600 ssl/ca/ca_key.pem
chmod 644 ssl/ca/ca_cert.pem
chmod 600 ssl/server/server_key.pem
chmod 644 ssl/server/server_cert.pem
chmod 600 ssl/client/client_key.pem
chmod 644 ssl/client/client_cert.pem

# Clean up temporary files
rm ssl/server/server.csr ssl/server/server_ext.cnf
rm ssl/client/client.csr ssl/client/client_ext.cnf
rm ssl/ca/ca_cert.srl

echo "✅ Certificates generated successfully!"
echo
echo "📁 Certificate locations:"
echo "CA Certificate:     ssl/ca/ca_cert.pem"
echo "CA Private Key:     ssl/ca/ca_key.pem"
echo "Server Certificate: ssl/server/server_cert.pem"
echo "Server Private Key: ssl/server/server_key.pem"
echo "Client Certificate: ssl/client/client_cert.pem"
echo "Client Private Key: ssl/client/client_key.pem"
echo
echo "📝 Update your .env file with:"
echo "SSL_CERT_PATH=ssl/server/server_cert.pem"
echo "SSL_KEY_PATH=ssl/server/server_key.pem"
echo "SSL_CLIENT_CA_CERT_PATH=ssl/ca/ca_cert.pem"
