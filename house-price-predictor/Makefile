# Define the API port in one place, and use it in all commands
PORT_API=8090

# Run the training script in dev mode
run-training-dev:
	cargo run --bin train -- \
		--bucket-name-s3 "house-price-prediction-let-s-rust" \
		--key-s3 "boston_housing_model.bin"

# Run the API in dev mode
run-api-dev:
	cargo run --bin api -- \
		--bucket-name-s3 "house-price-prediction-let-s-rust" \
		--key-s3 "boston_housing_model.bin" \
		--port $(PORT_API)

# Request the health endpoint in dev mode
request-health:
	curl http://localhost:$(PORT_API)/health

# Request the predict endpoint in dev mode
request-predict:
	curl -X POST http://localhost:$(PORT_API)/predict \
		-H "Content-Type: application/json" \
		-d '{ \
			"crim": 0.00632, \
			"zn": 18.0, \
			"indus": 2.31, \
			"chas": 0, \
			"nox": 0.538, \
			"rm": 6.575, \
			"age": 65.2, \
			"dis": 4.0900, \
			"rad": 1, \
			"tax": 296, \
			"ptratio": 15.3, \
			"b": 396.90, \
			"lstat": 4.98 \
		}'

build-api:
	docker build -t house-price-predictor .

run-api: build-api
	docker run \
		-p $(PORT_API):8080 \
		-e BUCKET_NAME_S3="house-price-prediction-let-s-rust" \
		-e KEY_S3="boston_housing_model.bin" \
		-e PORT=8080 \
		--env-file .aws.env \
		house-price-predictor


run-train-binary:
	cargo build --bin train
	./target/debug/train \
		--bucket-name-s3 "house-price-prediction-let-s-rust" \
		--key-s3 "boston_housing_model.bin"

remove-docker-cache:
	docker builder prune -f
