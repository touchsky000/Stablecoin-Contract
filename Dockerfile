FROM --platform=linux/amd64 ubuntu:22.04 

# Install essential tools including curl
RUN apt-get update && apt-get install -y \
    build-essential \
    pkg-config \
    libudev-dev \
    llvm \
    libclang-dev \
    protobuf-compiler \
    libssl-dev \
    curl

# Set Bash as the default shell
SHELL ["/bin/bash", "-c"]

# Install Rustc
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && \
    export PATH="/root/.cargo/bin:$PATH" && \
    rustup install 1.85.0 && \
    rustup default 1.85.0

# Install Solana CLI
RUN sh -c "$(curl -sSfL https://release.anza.xyz/v2.0.24/install)"
RUN echo 'export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"' >> ~/.bashrc
RUN source ~/.bashrc
# Set Solana CLI path globally
ENV PATH="/root/.local/share/solana/install/active_release/bin:$PATH"

# Verify Solana installation
RUN solana --version

RUN source ~/.bashrc && solana --version
# Install Node.js and npm
RUN curl -fsSL https://deb.nodesource.com/setup_lts.x | bash - && \
    apt-get install -y nodejs

# Set working directory
WORKDIR /app

# Copy project files
COPY . .

# Install Anchor CLI
RUN source $HOME/.cargo/env && \
    cargo install --git https://github.com/coral-xyz/anchor avm --force

RUN source $HOME/.cargo/env && \
    cargo update -p bytemuck_derive --precise 1.7.0

# # Install Anchor 0.29.0
RUN source $HOME/.cargo/env && \
    avm install 0.29.0

RUN source $HOME/.cargo/env && \
    avm use 0.29.0

RUN sed -i 's/^version = 4$/version = 3/' Cargo.lock
# Build the Anchor project
RUN source $HOME/.cargo/env && \
    anchor build


# Set the default command (optional)
CMD ["bash"]
