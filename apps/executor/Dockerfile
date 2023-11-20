# Build
FROM rust:1.74 as planner
RUN cargo install cargo-chef

# Set work directory
WORKDIR /mailer-server/apps/executor
COPY . .

# Prepare a build plan ("recipe")
RUN cargo chef prepare --recipe-path recipe.json

FROM rust:1.74 as build
RUN cargo install cargo-chef

# Install postgres library
RUN apt-get update && apt-get install libpq5 -y

# Copy the build plan from the previous Docker stage
COPY --from=planner /mailer-server/apps/executor/recipe.json recipe.json

# Build dependencies - this layer is cached as long as `recipe.json`
# doesn't change.
RUN cargo chef cook --release --recipe-path recipe.json

# Build the whole project
COPY . .

# Setup working directory
WORKDIR /mailer-server/apps/executor

# Build application
RUN cargo build --release --bin executor

# BUILD
FROM rust:1.74 AS runtime

# Install dependency (Required by diesel)
RUN apt-get update && apt-get install libpq5 -y

# Install Diesel CLI
RUN cargo install diesel_cli --no-default-features --features postgres

# Setup working directory
WORKDIR /var/www/mailer/apps/executor

# copy files
COPY apps/executor/.env.example .env

RUN rm apps core .github -rf

# Copy our built binary
COPY --from=build /target/release/executor /usr/local/bin/executor

CMD ["executor"]