# Build
FROM rust:1.70 as planner
RUN cargo install cargo-chef

# Set work directory
WORKDIR /usr/src/spiralover-mailer
COPY . .

# Prepare a build plan ("recipe")
RUN cargo chef prepare --recipe-path recipe.json

FROM rust:1.70 as build
RUN cargo install cargo-chef

# Copy the build plan from the previous Docker stage
COPY --from=planner /usr/src/spiralover-mailer/recipe.json recipe.json

# Build dependencies - this layer is cached as long as `recipe.json`
# doesn't change.
RUN cargo chef cook --recipe-path recipe.json

# Build the whole project
COPY . .

# Setup working directory
WORKDIR /mailer-server

# Build application
RUN cargo build --release

# BUILD
FROM rust:1.70 AS runtime

COPY .env .env

# Copy our built binary
COPY --from=build /target/release/mailer-server /usr/local/bin/mailer-server

CMD ["mailer-server"]