#!/bin/bash
################################HEAD############################
# Copyright 202$, xxxxxx Co. Ltd.
# All rights reserved.
# FileName:    xx.sh
# Description: first case for test.
# Author:      Sky
# http://www.xxxxxx.com
# Revision: 1.0.0
#################################################################
#set -e            	#打开异常退出功能
set -x            #打开Debug功能


######################定义变量######################
WORKDIR="$(cd $(dirname "$0") || exit 1;pwd)"		#脚本所在路径
SP1_DIR="${HOME}/.sp1"
PROFILE="${HOME}/.bashrc"


######################判断是否已安装######################
if [[ $(rustup toolchain list |grep  succinct) != "" ]];then
    echo " -> sp1 toolchain 旧版本已存在,开始卸载..."
    rustup toolchain remove succinct
     echo " -> sp1 toolchain 旧版本卸载完成"
else
    echo " -> sp1 toolchain 未安装"
fi

######################安装sp1up、cargo-prove######################
echo ""
echo " -> sp1up cargo-prove 开始安装..."
# 创建SP1_DIR/bin目录
if [[ ! -d ${SP1_DIR}/bin ]];then
    mkdir -p ${SP1_DIR}/bin
fi


if [[ -f ${HOME}/.cargo/bin/cargo-prove ]];then
    mv ${HOME}/.cargo/bin/cargo-prove ${HOME}/.cargo/bin/cargo-prove_old
fi


# 拷贝文件
cp -f "${WORKDIR}/bin/sp1up" "${SP1_DIR}/bin/sp1up"
cp -f "${WORKDIR}/bin/cargo-prove" "${SP1_DIR}/bin/cargo-prove"
chmod +x "${SP1_DIR}/bin/sp1up"
chmod +x "${SP1_DIR}/bin/cargo-prove"

if [[ ":$PATH:" != *":${SP1_DIR}/bin:"* ]]; then
    echo >> $PROFILE && echo "export PATH=\"\$PATH:${SP1_DIR}/bin\"" >> $PROFILE
fi
echo " -> sp1up cargo-prove 安装完成"


######################安装sp1 toolchain######################
TOOLCHAINS_VERSION="succinct-v2024-06-20"
TOOLCHAINS_TAR="succinct-v2024-06-20.tar.gz"

echo ""
echo " -> toolchains ${TOOLCHAINS_VERSION} 开始安装..."

if [[ -d "${SP1_DIR}/toolchains" ]];then
  rm -rf "${SP1_DIR}/toolchains"
fi

mkdir -p "${SP1_DIR}/toolchains/${TOOLCHAINS_VERSION}"

tar -xvf "${WORKDIR}/${TOOLCHAINS_TAR}" -C "${SP1_DIR}/toolchains/${TOOLCHAINS_VERSION}" >/dev/null 2>&1
rustup toolchain link succinct "${SP1_DIR}/toolchains/${TOOLCHAINS_VERSION}"
chmod -R 755 "${SP1_DIR}/toolchains/${TOOLCHAINS_VERSION}/bin"

TARGET_OS="x86_64-unknown-linux-gnu"
# 遍历文件夹中的所有文件
for bin in "${SP1_DIR}/toolchains/${TOOLCHAINS_VERSION}/lib/rustlib/${TARGET_OS}/bin"/*
do
    # 判断文件是否存在
    if [ -f "$bin" ]; then
        chmod 755 "$bin"
    fi
done

echo " -> toolchains ${TOOLCHAINS_VERSION} 安装完成"

echo ""
echo " 执行 'source ${PROFILE}' 或者新建命令行窗口 使用 cargo prove 命令"
