ARG MAILER_IMAGE=rust:1.79-slim-bookworm

# Build
FROM ${MAILER_IMAGE} as planner
RUN cargo install cargo-chef

# Set work directory
WORKDIR /spiralover/mailer-server
COPY . .

# Prepare a build plan ("recipe")
RUN cargo chef prepare --recipe-path recipe.json

FROM ${MAILER_IMAGE} as build
RUN cargo install cargo-chef

# Install postgres library
RUN apt-get update && apt-get install libssl-dev pkg-config libpq-dev -y

# Copy the build plan from the previous Docker stage
COPY --from=planner /spiralover/mailer-server/recipe.json recipe.json

# Build dependencies - this layer is cached as long as `recipe.json`
# doesn't change.
RUN cargo chef cook --release --recipe-path recipe.json

# Build the whole project
COPY . .

# Setup working directory
WORKDIR /spiralover/mailer-server

# Build application
RUN cargo build --release

# BUILD
FROM ${MAILER_IMAGE} AS runtime

# Install dependency (Required by diesel)
RUN apt-get update && apt-get install curl libpq-dev -y

# Install Diesel CLI
RUN cargo install diesel_cli --no-default-features --features postgres --locked

# Setup working directory
WORKDIR /spiralover/mailer-server

# Create uploads directory
RUN mkdir -p /spiralover/mailer-server/static/uploads

# copy files
COPY ../../diesel.toml diesel.toml
COPY ../../app-setup.sh app-setup.sh
COPY ../../app-refresh-setup.sh app-refresh-setup.sh

# copy folders
COPY resources resources

RUN rm apps cosmic docs examples .github -rf

# Copy our built binary
COPY --from=build /target/release/user /usr/local/bin/user
COPY --from=build /target/release/executor /usr/local/bin/executor

CMD ["user"]
