#!/bin/bash

# Create SSL directory if it doesn't exist
mkdir -p ssl

# Generate SSL certificate and key without prompts
openssl req -x509 \
    -newkey rsa:4096 \
    -keyout ssl/key.pem \
    -out ssl/cert.pem \
    -days 365 \
    -nodes \
    -subj "/C=US/ST=State/L=City/O=Organization/OU=Unit/CN=localhost"

# Set correct permissions
chmod 600 ssl/key.pem
chmod 644 ssl/cert.pem

echo "✅ SSL certificates generated successfully in the ssl directory!"
echo "📝 Update your .env file with:"
echo "SSL_CERT_PATH=ssl/cert.pem"
echo "SSL_KEY_PATH=ssl/key.pem"
