ARG IMAGE=rust:1.74-slim-bookworm

# Build
FROM ${IMAGE} as planner
RUN cargo install cargo-chef

# Set work directory
WORKDIR /mailer-server/apps/user
COPY . .

# Prepare a build plan ("recipe")
RUN cargo chef prepare --recipe-path recipe.json

FROM ${IMAGE} as build
RUN cargo install cargo-chef

# Install postgres library
RUN apt-get update && apt-get install libssl-dev pkg-config libpq-dev -y

# Copy the build plan from the previous Docker stage
COPY --from=planner /mailer-server/apps/user/recipe.json recipe.json

# Build dependencies - this layer is cached as long as `recipe.json`
# doesn't change.
RUN cargo chef cook --release --recipe-path recipe.json

# Build the whole project
COPY . .

# Setup working directory
WORKDIR /mailer-server/apps/user

# Build application
RUN cargo build --release --bin user

# BUILD
FROM ${IMAGE} AS runtime

# Install dependency (Required by diesel)
RUN apt-get update && apt-get install curl libpq-dev -y

# Setup working directory
WORKDIR /var/www/mailer/apps/user

# copy files
COPY apps/user/.env.example .env

RUN rm apps core docs examples .github -rf

# Copy our built binary
COPY --from=build /target/release/user /usr/local/bin/user

CMD ["user"]