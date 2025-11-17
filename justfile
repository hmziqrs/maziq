# MazIQ - Just commands

# Default recipe to display help information
default:
    @just --list

# Build the binary
build:
    go build -o maziq cmd/maziq/main.go

# Build with version info
build-release VERSION:
    go build -ldflags="-X main.version={{VERSION}}" -o maziq cmd/maziq/main.go

# Run the application
run:
    go run cmd/maziq/main.go

# Run with race detector
run-race:
    go run -race cmd/maziq/main.go

# Install dependencies
deps:
    go mod download
    go mod tidy

# Update dependencies
update-deps:
    go get -u ./...
    go mod tidy

# Run tests
test:
    go test ./...

# Run tests with coverage
test-coverage:
    go test -coverprofile=coverage.out ./...
    go tool cover -html=coverage.out -o coverage.html
    @echo "Coverage report generated: coverage.html"

# Run tests with verbose output
test-verbose:
    go test -v ./...

# Format code
fmt:
    go fmt ./...

# Run linter
lint:
    @command -v golangci-lint >/dev/null 2>&1 || { echo "golangci-lint not installed. Install: go install github.com/golangci/golangci-lint/cmd/golangci-lint@latest"; exit 1; }
    golangci-lint run

# Vet code
vet:
    go vet ./...

# Run all checks (fmt, vet, lint, test)
check: fmt vet lint test
    @echo "✅ All checks passed!"

# Clean build artifacts
clean:
    rm -f maziq
    rm -f coverage.out coverage.html
    go clean

# Install the binary to $GOPATH/bin
install:
    go install cmd/maziq/main.go

# Uninstall the binary from $GOPATH/bin
uninstall:
    rm -f $(go env GOPATH)/bin/maziq

# Watch and rebuild on file changes (requires entr)
watch:
    @command -v entr >/dev/null 2>&1 || { echo "entr not installed. Install: brew install entr"; exit 1; }
    find . -name "*.go" | entr -r just run

# Generate module documentation
doc:
    @echo "Starting documentation server at http://localhost:6060"
    godoc -http=:6060

# Show dependency tree
deps-tree:
    go mod graph

# Check for outdated dependencies
deps-outdated:
    go list -u -m all

# Benchmark tests
bench:
    go test -bench=. -benchmem ./...

# Profile CPU usage
profile-cpu:
    go test -cpuprofile=cpu.prof -bench=.
    go tool pprof -http=:8080 cpu.prof

# Profile memory usage
profile-mem:
    go test -memprofile=mem.prof -bench=.
    go tool pprof -http=:8080 mem.prof

# Build for multiple platforms
build-all:
    @echo "Building for multiple platforms..."
    GOOS=darwin GOARCH=amd64 go build -o dist/maziq-darwin-amd64 cmd/maziq/main.go
    GOOS=darwin GOARCH=arm64 go build -o dist/maziq-darwin-arm64 cmd/maziq/main.go
    GOOS=linux GOARCH=amd64 go build -o dist/maziq-linux-amd64 cmd/maziq/main.go
    GOOS=linux GOARCH=arm64 go build -o dist/maziq-linux-arm64 cmd/maziq/main.go
    @echo "✅ Binaries built in dist/"

# Print Go environment info
info:
    @echo "Go version:"
    @go version
    @echo "\nGo environment:"
    @go env GOOS GOARCH GOPATH GOROOT
    @echo "\nModule info:"
    @go list -m

# Initialize new module (if needed)
init MODULE:
    go mod init {{MODULE}}

# Verify dependencies
verify:
    go mod verify
