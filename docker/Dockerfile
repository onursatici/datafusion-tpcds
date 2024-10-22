FROM ubuntu:24.04
RUN apt-get update
RUN apt-get install -y gcc-9 make byacc

ARG TPC_DS_PATH
COPY $TPC_DS_PATH ./tpc_ds
WORKDIR ./tpc_ds/tools
RUN make CC=gcc-9  # tpc_ds only compiles with gcc 9 as of 3.2.0-rc1

# TODO: run dsqgen with a given scale factor
# TODO: run dsdgen with a given scale factor
