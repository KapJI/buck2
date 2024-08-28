FROM ubuntu@sha256:b2175cd4cfdd5cdb1740b0e6ec6bbb4ea4892801c0ad5101a81f694152b6c559
RUN apt-get update -yq
RUN apt-get install -yq python3
RUN apt-get install -yq clang curl
RUN echo 'PATH="/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/usr/games:/usr/local/games:/snap/bin:/home/engflow/.cargo/bin"' > /etc/environment
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
USER engflow
RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly-2024-06-08 -y

