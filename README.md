# Telegram Ngrok setWebhookUrl

Parses your opened ngrok tunnels and sets Telegram webhook to HTTPS.

## Installation
```shell
git clone git@github.com:Filipponik/tg-ngrok-rs.git
cargo build --release
```

## Usage
```shell
./target/release/tg-ngrok-rs <RELATIVE_PATH> <TELEGRAM_TOKEN>
```

### Example:
```shell
./target/release/tg-ngrok-rs /webhooks/telegram 1234567890:AAbbccddeeffgghhiijjkk1234567890
```