#!/bin/bash

# Set variables
CERT_DIR="../certs"
CERT_FILE="$CERT_DIR/certificate.crt"
KEY_FILE="$CERT_DIR/private.key"
DAYS_VALID=365
COUNTRY="US"
STATE="State"
LOCALITY="City"
ORGANIZATION="Organization"
ORGANIZATIONAL_UNIT="Development"
COMMON_NAME="localhost"
EMAIL="admin@example.com"

# Create certs directory if it doesn't exist
mkdir -p $CERT_DIR

echo "🔑 Generating SSL certificates..."

# Generate private key and certificate
openssl req -x509 \
    -newkey rsa:4096 \
    -nodes \
    -days $DAYS_VALID \
    -keyout $KEY_FILE \
    -out $CERT_FILE \
    -subj "/C=$COUNTRY/ST=$STATE/L=$LOCALITY/O=$ORGANIZATION/OU=$ORGANIZATIONAL_UNIT/CN=$COMMON_NAME/emailAddress=$EMAIL"

# Check if files were created successfully
if [ -f "$CERT_FILE" ] && [ -f "$KEY_FILE" ]; then
    echo "✅ SSL certificates generated successfully!"
    echo "📁 Location:"
    echo "   - Certificate: $CERT_FILE"
    echo "   - Private Key: $KEY_FILE"

    # Set correct permissions
    chmod 644 $CERT_FILE
    chmod 600 $KEY_FILE

    echo "🔒 File permissions set correctly"
else
    echo "❌ Error generating certificates"
    exit 1
fi

# Display certificate information
echo -e "\n📋 Certificate Information:"
openssl x509 -in $CERT_FILE -text -noout | grep -E "Subject:|Issuer:|Not Before:|Not After :|Public-Key:"
