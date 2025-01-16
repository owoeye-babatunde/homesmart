# Let's Rust!

## Session 3

#### Table of contents


### Goals
- [x] Install actix-web
- [x] Build a simple web app with one health endpoint
- [x] Add a predict endpoint that accepts a JSON payload
- [x] Function to download the model from S3
- [x] A bit of refactorings
    - [x] Split `lib.rs` into 3 files: `data.rs`, `aws.rs` and `model.rs`
    - [x] Avoid code duplication in the AWS functions
    - [x] Pass bucket name and model file in S3 as input parameters to our training and api binaries.
- [x] Load the model binary file into memory so we can use it to generate predictions
- [ ] Use the model and the client sent features to generate and return a prediction.
- [ ] dev containers for painless development


### Using `Arc` for thread safe data accesing
TODO

### Understand better why we are forced to use `move` for our closure
Why do we need to move the `model_path` variable into the closure?

### Dev containers

First, create a .devcontainer directory in your project root:
```bash
mkdir devcontainer
```
Create a `devcontainer.json` file:
```json
{
    "name": "Rust Development",
    "build": {
        "dockerfile": "Dockerfile"
    },
    "customizations": {
        "vscode": {
            "extensions": [
                "rust-lang.rust-analyzer",
                "serayuzgur.crates",
                "tamasfe.even-better-toml"
            ]
        }
    },
    "remoteUser": "vscode",
    "features": {
        "ghcr.io/devcontainers/features/common-utils:2": {
            "installZsh": true,
            "username": "vscode",
            "upgradePackages": true
        }
    }
}
```
Create a Dockerfile:
```Dockerfile
FROM mcr.microsoft.com/devcontainers/rust:1-bullseye

# Install additional OS packages
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends \
    pkg-config \
    libssl-dev
```

Now you can use this dev container by:
* Installing the "Dev Containers" extension in VS Code if you haven't already
* Opening the command palette (Ctrl/Cmd + Shift + P)
* Selecting "Dev Containers: Reopen in Container"

This configuration:
* Uses the official Microsoft Rust dev container image
* Includes essential Rust-related VS Code extensions
* Adds common utilities and development tools
* Installs necessary system packages for Rust web development
* Provides a consistent development environment across different machines
* The container includes:
* Rust toolchain (rustc, cargo, etc.)
* rust-analyzer for IDE support
* Git
* Basic Linux tools
* SSL development libraries (useful for web development)

You can customize the container further by:
* Adding more VS Code extensions in devcontainer.json
* Installing additional packages in the Dockerfile
* Modifying environment variables
* Adding more features from the dev container feature catalog