FROM ubuntu:18.04

ENV DOWNLOAD https://github.com/pfnet-research/menoh/releases/download
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
    build-essential \
    ca-certificates \
    curl \
    pkg-config \
    python3-dev \
    python3-pip \
    python3-setuptools \
    && curl -LO $DOWNLOAD/v1.1.1/ubuntu1804_mkl-dnn_0.16-1_amd64.deb \
    && curl -LO $DOWNLOAD/v1.1.1/ubuntu1804_menoh_1.1.1-1_amd64.deb \
    && curl -LO $DOWNLOAD/v1.1.1/ubuntu1804_menoh-dev_1.1.1-1_amd64.deb \
    && apt install -y ./*.deb \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/* \
    && rm *.deb

COPY . menoh-yolo/

RUN pip3 install --no-cache-dir \
    chainer==4.4 \
    chainercv==0.10 \
    onnx-chainer==1.2.2a3
RUN cd menoh-yolo \
    && python3 convert.py --out /usr/local/share/YOLOv2.onnx

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain=stable
RUN cd menoh-yolo \
    && sed -e 's#YOLOv2.onnx#/usr/local/share/YOLOv2.onnx#' -i src/main.rs \
    && PATH=$HOME/.cargo/bin:$PATH cargo build --release -j $(nproc) \
    && install -m 755 target/release/menoh-yolo /usr/local/bin/

RUN curl -LO https://github.com/pjreddie/darknet/raw/master/data/dog.jpg
