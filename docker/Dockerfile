FROM ubuntu@sha256:b2175cd4cfdd5cdb1740b0e6ec6bbb4ea4892801c0ad5101a81f694152b6c559
RUN apt-get update -yq && apt-get install -yq python3 \
  clang \
  curl \
  lld \
  unzip \
  && apt-get autoremove
RUN groupadd \
  --gid 114 \
  engflow && \
  useradd \
  --home-dir /home/engflow \
  --create-home \
  --uid 108 \
  --gid 114 \
  engflow
ENV HOME=/home/engflow
ENV PATH "$PATH:$HOME/.cargo/bin"
USER engflow
RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly-2024-06-08 -y
