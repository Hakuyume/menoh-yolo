FROM debian:stretch AS build

RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    ca-certificates \
    clang \
    cmake \
    curl \
    git \
    libprotobuf-dev \
    pkg-config \
    protobuf-compiler \
    python3-dev \
    python3-pip \
    python3-setuptools \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*
RUN echo '/usr/local/lib' > /etc/ld.so.conf.d/local.conf \
    && ldconfig
RUN git clone https://github.com/intel/mkl-dnn.git --branch=v0.14 --depth=1 \
    && cd mkl-dnn/scripts \
    && ./prepare_mkl.sh \
    && cd .. \
    && mkdir build \
    && cd build \
    && cmake .. \
    -DCMAKE_BUILD_TYPE=Release \
    -DWITH_EXAMPLE=OFF \
    -DWITH_TEST=OFF \
    && make -j $(nproc) \
    && make install
RUN git clone https://github.com/pfnet-research/menoh.git --branch v1.0.2 --depth=1 \
    && cd menoh \
    && mkdir build \
    && cd build \
    && cmake .. \
    -DCMAKE_BUILD_TYPE=Release \
    -DENABLE_BENCHMARK=OFF \
    -DENABLE_EXAMPLE=OFF \
    -DENABLE_TEST=OFF \
    -DENABLE_TOOL=OFF \
    && make -j $(nproc) \
    && make install

RUN pip3 install --no-cache-dir \
    chainer==4.2 \
    chainercv==0.10 \
    onnx-chainer==1.1.1a2 \
    setuptools
COPY convert.py menoh-yolo/
RUN cd menoh-yolo \
    && python3 convert.py --out /usr/local/share/YOLOv2.onnx

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain=1.27.0
COPY Cargo.* menoh-yolo/
COPY src menoh-yolo/src
RUN cd menoh-yolo \
    && sed -e 's#YOLOv2.onnx#/usr/local/share/YOLOv2.onnx#' -i src/main.rs \
    && PATH=$HOME/.cargo/bin:$PATH \
    PKG_CONFIG_PATH=/usr/local/share/pkgconfig \
    cargo build --release -j $(nproc) \
    && install -m 755 target/release/menoh-yolo /usr/local/bin/

FROM debian:stretch AS deploy
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    libprotobuf10 \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*
COPY --from=build /usr/local/bin/menoh-yolo /usr/local/bin/
COPY --from=build /usr/local/lib/lib*.so* /usr/local/lib/
COPY --from=build /usr/local/share/YOLOv2.onnx /usr/local/share/
RUN ln -sf libmkldnn.so.0.14.0 /usr/local/lib/libmkldnn.so.0 \
    && ln -sf libmkldnn.so.0 /usr/local/lib/libmkldnn.so \
    && echo '/usr/local/lib' > /etc/ld.so.conf.d/local.conf \
    && ldconfig
RUN curl -LO https://github.com/pjreddie/darknet/raw/master/data/dog.jpg
