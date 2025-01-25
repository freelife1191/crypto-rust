https://support.atlassian.com/bitbucket-cloud/docs/configure-ssh-and-two-step-verification/

https://ghkim.net/151/

```shell
sudo apt-get update && sudo apt-get install -y openjdk-21-jdk build-essential cmake clang

cd /usr/lib/jvm

ssh-keygen -t ed25519 -b 256

cat ~/.ssh/id_ed25519.pub

git clone git@bitbucket.org:freelife/crypto-rust.git

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

rustup toolchain install stable

sudo apt-get install build-essential cmake clang


echo "export JAVA_HOME=/usr/lib/jvm/java-21-openjdk-arm64" >> ~/.bashrc
source ~/.bashrc
```

```shell
# docker 설치
curl -s https://get.docker.com/ | sudo sh

sudo usermod -aG docker $USER && sudo service docker restart

# cross build
cargo install cross --git https://github.com/cross-rs/cross
```