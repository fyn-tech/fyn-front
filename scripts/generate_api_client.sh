#!/bin/bash
# Generate API client from Django OpenAPI spec
# Now using JWT authentication - no cookie workarounds needed!

API_DIR=../fyn-api/
FRONT_DIR=${PWD}
CLIENT_NAME="fyn_api"

echo "Generating API client..."

# Activate Python venv and generate OpenAPI spec
source ../venv_fyn/bin/activate
cd ${API_DIR}
python manage.py spectacular --file ${FRONT_DIR}/fyn_api_client.yaml

# Generate Rust client from spec
cd ${FRONT_DIR}
openapi-generator-cli generate \
  -i fyn_api_client.yaml \
  -g rust \
  -o ./${CLIENT_NAME} \
  --additional-properties=packageName=${CLIENT_NAME},packageVersion=1.0.0

# Fix the models::models bug in generated code
echo "Fixing models::models bug..."
find ./${CLIENT_NAME}/src -name "*.rs" -type f -exec sed -i 's/models::models::/models::/g' {} \;

echo "API client generated successfully!"
echo "✅ No modifications needed - JWT bearer tokens work natively!"