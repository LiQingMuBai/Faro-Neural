# Rust Telegram Bot

1. Open `@BotFather` in Telegram and run `/newbot`.
2. Copy `.env.example` to `.env`.
3. Put the BotFather token in `TELEGRAM_BOT_TOKEN`.
4. Set `PUBLIC_SITE_URL` to the public HTTPS URL of this website. Telegram users cannot open `localhost` on your computer.
5. Install a current Rust toolchain and run `npm run bot`, or run `cargo run --release --manifest-path bot/Cargo.toml` directly.

The bot is implemented entirely in Rust using Tokio, Reqwest, Serde, and the Telegram HTTPS API. It does not store message content. Language preference is held only in memory and resets when the process restarts.
