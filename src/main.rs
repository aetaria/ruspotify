use rspotify::{
    AuthCodeSpotify, ClientCredsSpotify, Config, Credentials, OAuth, 
    prelude::*, 
    scopes
};

use anyhow::Result;

// initializes Spotify client with authorization code flow
pub async fn init_client() -> Result<AuthCodeSpotify> {
    // load credentials from .env
    let creds = Credentials::from_env().unwrap();

    // define scopes we need
    let oauth = OAuth {
        redirect_uri: "http://localhost:8888/callback".to_string(),
        scopes: scopes!(
            "user-read-playback-state",
            "user-modify-playback-state",
            "user-read-currently-playing"
        ),
        ..Default::default()
    };

    let config = Config {
        token_cached: true,
        token_refreshing: true,
        ..Default::default()
    };

    let spotify = AuthCodeSpotify::with_config(
        creds,
        oauth,
        config
    );

    // get the authorization URL
    let url = spotify.get_authorize_url(false).unwrap();

    // prompt user to visit the URL
    println!("Please visit this URL to authorize: {}", url);
    println!("After authorizing, paste the full redirect URL here:");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    // Parse the code from the redirect URL
    spotify.request_token(&input.trim()).await?;
    
    Ok(spotify)
}

fn main() {
    println!("Hello, world!");
}