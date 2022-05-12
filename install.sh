# good job for looking at this
# you can probably do it yourself now

cargo build --release

echo "Moving binary to /usr/bin"
sudo mv target/release/macchanger-rs /usr/bin