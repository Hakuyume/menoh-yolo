FROM ubuntu:18.04

ENV DOWNLOAD https://github.com/pfnet-research/menoh/releases/download
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
    build-essential \
    ca-certificates \
    curl \
    pkg-config \
    && curl -LO $DOWNLOAD/v1.1.1/ubuntu1804_mkl-dnn_0.16-1_amd64.deb \
    && curl -LO $DOWNLOAD/v1.1.1/ubuntu1804_menoh_1.1.1-1_amd64.deb \
    && curl -LO $DOWNLOAD/v1.1.1/ubuntu1804_menoh-dev_1.1.1-1_amd64.deb \
    && apt install -y ./*.deb \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/* \
    && rm *.deb

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain=stable

COPY . menoh-yolo/

RUN cd menoh-yolo \
    && sed -e 's#yolo_v2_voc0712.onnx#/usr/local/share/yolo_v2_voc0712.onnx#' -i src/main.rs \
    && sed -e 's#yolo_v2_voc0712.json#/usr/local/share/yolo_v2_voc0712.json#' -i src/main.rs \
    && PATH=$HOME/.cargo/bin:$PATH cargo build --release -j $(nproc) \
    && install -m 755 target/release/menoh-yolo /usr/local/bin/

RUN cd /usr/local/share \
    && curl -LO https://github.com/Hakuyume/menoh-yolo/releases/download/assets/yolo_v2_voc0712.onnx \
    && curl -LO https://github.com/Hakuyume/menoh-yolo/releases/download/assets/yolo_v2_voc0712.json

RUN curl -LO https://github.com/pjreddie/darknet/raw/master/data/dog.jpg
