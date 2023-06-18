FROM python:3.10.10-bullseye

RUN apt update 
RUN apt install -y curl

# codenames
RUN git clone https://github.com/kikefdezl/codenames-AI.git /codenames
WORKDIR /codenames
RUN pip install .

# rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN cargo build

ENTRYPOINT cargo run
