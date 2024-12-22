# Use the official Rust image as a parent image
FROM rust:latest

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the current directory contents into the container at /usr/src/app
COPY . .

# Build the Rust application
RUN cargo build --release

# Run the executable
CMD ["cargo", "run", "--release"]
