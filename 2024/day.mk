.PHONY: run
run: input
	go run main.go < input

.PHONY: test
test: example
	go run main.go < example
