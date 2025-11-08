# Spotify TUI

A terminal-based Spotify client built with Rust, featuring playback control, search, and playlist management through an intuitive keyboard-driven interface.

## Features

- 🎵 Real-time playback control (play, pause, skip, volume)
- 🔍 Search for tracks, artists, and albums
- 📋 Browse and manage playlists
- ⌨️ Fully keyboard-driven navigation
- 🎨 Clean, minimalist terminal UI
- 🔄 Async API requests for responsive performance

## Prerequisites

- Rust (1.70 or later)
- A Spotify Premium account (required for playback control)
- Spotify application running on at least one device

## Installation

1. **Clone the repository:**
   ```bash
   git clone https://github.com/yourusername/spotify-tui.git
   cd spotify-tui
   ```

2. **Set up Spotify API credentials:**
   
   a. Go to the [Spotify Developer Dashboard](https://developer.spotify.com/dashboard)
   
   b. Log in and click "Create an App"
   
   c. Fill in the app details:
      - App name: `Spotify TUI` (or your choice)
      - App description: `Terminal-based Spotify client`
      - Redirect URI: `http://127.0.0.1:8888/callback`
   
   d. Save your Client ID and Client Secret

3. **Configure environment variables:**
   
   Create a `.env` file in the project root:
   ```bash
   cp .env.example .env
   ```
   
   Edit `.env` and add your credentials:
   ```
   RSPOTIFY_CLIENT_ID=your_client_id_here
   RSPOTIFY_CLIENT_SECRET=your_client_secret_here
   RSPOTIFY_REDIRECT_URI=http://127.0.0.1:8888/callback
   ```

4. **Build and run:**
   ```bash
   cargo run
   ```

## First Run Authentication

On your first run, the application will:
1. Display an authorization URL
2. Open your browser to Spotify's login page
3. Ask you to authorize the app
4. Redirect to `http://127.0.0.1:8888/callback?code=...`
5. Prompt you to paste the full redirect URL back into the terminal

After initial authentication, your token will be cached for future sessions.

## Usage

### Keyboard Controls

**Playback:**
- `Space` - Play/Pause
- `n` - Next track
- `p` - Previous track
- `+` / `-` - Volume up/down

**Navigation:**
- `↑` / `↓` - Navigate lists
- `Enter` - Select item
- `Tab` - Switch between panels
- `Esc` - Go back

**Search:**
- `/` - Open search
- Type to search tracks/artists/albums

**General:**
- `q` - Quit application
- `?` - Show help

## Project Structure

```
spotify-tui/
├── src/
│   ├── main.rs          # Application entry point
│   ├── spotify.rs       # Spotify API client wrapper
│   ├── ui.rs            # TUI rendering logic
│   └── app.rs           # Application state and event handling
├── Cargo.toml           # Dependencies
├── .env                 # Your API credentials (not committed)
├── .env.example         # Template for environment variables
└── README.md
```

## Dependencies

- **rspotify** - Spotify Web API wrapper
- **ratatui** - Terminal UI framework
- **crossterm** - Terminal manipulation
- **tokio** - Async runtime
- **anyhow** - Error handling
- **dotenv** - Environment variable management

## Troubleshooting

### "Authentication failed"
- Verify your Client ID and Secret are correct in `.env`
- Ensure the Redirect URI matches exactly: `http://127.0.0.1:8888/callback`
- Check that the redirect URI is added in your Spotify App settings

### "Nothing currently playing"
- Make sure Spotify is open and playing on at least one device
- You need Spotify Premium for full playback control
- Try refreshing or restarting the application

### "Rate limit exceeded"
- The Spotify API has rate limits
- Wait a few moments before retrying
- Consider implementing caching if making frequent requests

## Development

### Running in development mode:
```bash
cargo run
```

### Building for release:
```bash
cargo build --release
./target/release/spotify-tui
```

### Running tests:
```bash
cargo test
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [rspotify](https://github.com/ramsayleung/rspotify) - Spotify API wrapper
- [ratatui](https://github.com/ratatui-org/ratatui) - Terminal UI framework
- [spotify-tui](https://github.com/Rigellute/spotify-tui) - Inspiration for this project

## Disclaimer

This project is not affiliated with Spotify AB. Spotify is a registered trademark of Spotify AB.