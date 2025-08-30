API_DIR=../fyn-api/
FRONT_DIR=${PWD}
CLIENT_NAME="fyn_api"

source ../venv_development/bin/activate
cd ${API_DIR}
python manage.py spectacular --file ${FRONT_DIR}/fyn_api_client.yaml
cd ${FRONT_DIR}
openapi-generator-cli generate -i fyn_api_client.yaml -g rust -o ./${CLIENT_NAME}  --additional-properties=packageName=${CLIENT_NAME},packageVersion=1.0.0
${FRONT_DIR}/scripts/fix_generated_rust.sh