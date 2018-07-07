FROM base/archlinux AS build

RUN pacman -Syu --noconfirm \
    base-devel \
    git

RUN useradd user -m \
    && echo 'user ALL=(ALL) NOPASSWD: /usr/bin/pacman' > /etc/sudoers.d/user
USER user
WORKDIR /home/user

RUN git clone https://github.com/Hakuyume/menoh-PKGBUILDs.git --depth=1 \
    && cd menoh-PKGBUILDs \
    && cd mkl-dnn && makepkg -si --noconfirm --nocheck && cd .. \
    && cd menoh && makepkg -si --noconfirm --nocheck

RUN sudo pacman -Syu --noconfirm \
    clang \
    rustup\
    && rustup default stable \
    && rustup update
COPY --chown=user:user Cargo.* menoh-yolo/
COPY --chown=user:user src/* menoh-yolo/src/
RUN cd menoh-yolo && cargo build --release

RUN sudo pacman -Syu --noconfirm \
    python \
    python-pip \
    && pip install --user \
    chainercv==0.10 \
    onnx-chainer==1.1.1a2
COPY --chown=user:user convert.py menoh-yolo/
RUN cd menoh-yolo && python convert.py

FROM base/archlinux AS deploy
RUN pacman -Syu --noconfirm
WORKDIR /root
COPY --from=build \
     /home/user/menoh-PKGBUILDs/menoh/menoh-*-x86_64.pkg.tar.xz \
     /home/user/menoh-PKGBUILDs/mkl-dnn/mkl-dnn-*-x86_64.pkg.tar.xz \
     /home/user/menoh-yolo/target/release/menoh-yolo \
     /home/user/menoh-yolo/YOLOv2.onnx \
     ./
RUN pacman -U --noconfirm \
    menoh-*-x86_64.pkg.tar.xz \
    mkl-dnn-*-x86_64.pkg.tar.xz
