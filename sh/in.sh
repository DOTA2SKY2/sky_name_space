echo "GRT Installing toolchain finish"ars@hos:~/sky/succinct-v2024-08-01.1$ cat install-toolchains.sh
#!/usr/bin/env bash
# 代替安装 "https://raw.githubusercontent.com/succinctlabs/sp1/main/sp1up/sp1up"

set -x
echo "GRT Installing sp1 toolchains local..."
BASE_DIR=$HOME
SP1_DIR=${SP1_DIR-"$BASE_DIR/.sp1"}

echo "GRT Installing sp1up && cargo-prove..."
SP1_BIN_DIR="$SP1_DIR/bin"

mkdir -p "$SP1_BIN_DIR"
rm "$SP1_BIN_DIR/sp1up"
rm "$SP1_BIN_DIR/cargo-prove"

cp "$PWD/bin/sp1up" "$SP1_BIN_DIR/sp1up"
cp "$PWD/bin/cargo-prove" "$SP1_BIN_DIR/cargo-prove"

chmod +x "$SP1_BIN_DIR/sp1up"
chmod +x "$SP1_BIN_DIR/cargo-prove"

# Store the correct profile file (i.e. .profile for bash or .zshenv for ZSH).
case $SHELL in
*/zsh)
    PROFILE=${ZDOTDIR-"$HOME"}/.zshenv
    PREF_SHELL=zsh
    ;;
*/bash)
    PROFILE=$HOME/.bashrc
    PREF_SHELL=bash
    ;;
*/fish)
    PROFILE=$HOME/.config/fish/config.fish
    PREF_SHELL=fish
    ;;
*/ash)
    PROFILE=$HOME/.profile
    PREF_SHELL=ash
    ;;
*)
    echo "sp1up: could not detect shell, manually add ${SP1_BIN_DIR} to your PATH."
    exit 1
esac

if [[ ":$PATH:" != *":${SP1_BIN_DIR}:"* ]]; then
    # Add the sp1up directory to the path and ensure the old PATH variables remain.
    echo >> $PROFILE && echo "export PATH=\"\$PATH:$SP1_BIN_DIR\"" >> $PROFILE
fi
echo "GRT Install sp1up && cargo-prove finished"

echo "GRT Installing toolchain ..."

TOOLCHAINS_VERSION="succinct-v2024-08-01.1"
TARGET_OS="x86_64-unknown-linux-gnu"
TOOLCHAINS_NAME="rust-toolchain-$TARGET_OS.tar.gz"
rm -rf "$SP1_DIR/toolchains"

mkdir -p "$SP1_DIR/toolchains/$TOOLCHAINS_VERSION"
tar -xvf "$PWD/$TOOLCHAINS_NAME" -C "$SP1_DIR/toolchains/$TOOLCHAINS_VERSION" >/dev/null 2>&1

rustup toolchain remove succinct
#rustup toolchain link succinct /home/ars/.sp1/toolchains/succinct-v2024-08-01.1
rustup toolchain link succinct "$SP1_DIR/toolchains/$TOOLCHAINS_VERSION"
chmod -R 755 "$SP1_DIR/toolchains/$TOOLCHAINS_VERSION/bin"

# 遍历文件夹中的所有文件
for bin in "$SP1_DIR/toolchains/$TOOLCHAINS_VERSION/lib/rustlib/$TARGET_OS/bin"/*
do
    # 判断文件是否存在
    if [ -f "$bin" ]; then
#        echo "处理文件：$bin"
        chmod 755 "$bin"
        # 在这里可以进行文件的操作，例如拷贝、移动、重命名等
    fi
done

echo && echo "Detected your preferred shell is ${PREF_SHELL} and added sp1up to PATH. Run 'source ${PROFILE}' or start a new terminal session to use sp1up."
echo "GRT Installing toolchain finish"ars@hos:~/sky/succinct-v2024-08-01.1$