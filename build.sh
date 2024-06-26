#!/usr/bin/env zsh

docker run -it -d --name "purifier" -v /mnt/sdb/zfq/purifier:/work quay.io/pypa/manylinux_2_28_x86_64:latest /bin/bash