FROM python:3.10.10-bullseye

RUN apt update 
RUN apt install -y rustc

COPY . /codenames
WORKDIR /codenames

RUN pip install -r requirements.txt
ENTRYPOINT cargo run
