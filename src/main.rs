use twitch_irc::message::ServerMessage;
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::TwitchIRCClient;
use twitch_irc::{ClientConfig, SecureTCPTransport};
use std::time::Duration;
use tokio::time;
use tokio::sync::mpsc;
use futures::future;

mod spotify;

#[tokio::main]
pub async fn main() {
    // default configuration is to join chat as anonymous.

    let login_name = std::env::var("TWITCH_NAME").ok().unwrap();
    let oauth_token = std::env::var("TWITCH_TOKEN").ok().unwrap();

    let config = ClientConfig::new_simple(
        StaticLoginCredentials::new(login_name, Some(oauth_token))
    );
    let (incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

    // join a channel
    // This function only returns an error if the passed channel login name is malformed,
    // so in this simple case where the channel name is hardcoded we can ignore the potential
    // error with `unwrap`.
    client.join("rockzombie2".to_owned()).unwrap();

    let song = spotify::get_current_song().await.unwrap();
    if song.is_playing.is_some() && song.is_playing.unwrap() {
        let item = song.item.unwrap();
        let name = item.name;
        let ref artists = &item.album.artists[0];
        let artist = artists.name.as_str();
        println!("Song: {} - {}", name, artist);
    }

    let x = client.clone();

    let (_x, _y) = future::join(read_messages(incoming_messages, client), reminder(x)).await;
}

async fn read_messages(mut incoming_messages: mpsc::UnboundedReceiver<ServerMessage>, client: TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>) {
    // first thing you should do: start consuming incoming messages,
    // otherwise they will back up.
    while let Some(message) = incoming_messages.recv().await {
        //println!("Received message: {:?}", message);

        match message {
            ServerMessage::Privmsg(msg) => {
                println!("(#{}) {}: {}", msg.channel_login, msg.sender.name, msg.message_text);

                // Discord
                if msg.message_text.to_lowercase().contains("!discord") {
                    client.say("rockzombie2".to_owned(), "Join the Space Pirates Discord! https://discord.gg/ErM35TPK2D".to_owned()).await.unwrap();
                }

                // Operating System
                if msg.message_text.to_lowercase().contains("!os") {
                    client.say("rockzombie2".to_owned(), "I run Windows as my host OS (for gaming) and screencapture my !laptop which dual-boots FreeBSD and Windows 11.".to_owned()).await.unwrap();
                }

                // Laptop
                if msg.message_text.to_lowercase().contains("!laptop") {
                    client.say("rockzombie2".to_owned(), "I dual boot FreeBSD and Windows 11 on a ThinkPad P15v Gen 3 AMD (15\") laptop.".to_owned()).await.unwrap();
                }

                // Window Manager
                if msg.message_text.to_lowercase().contains("!wm") {
                    client.say("rockzombie2".to_owned(), "I use the suckless window manager, DWM.".to_owned()).await.unwrap();
                }

                // dotfiles
                if msg.message_text.to_lowercase().contains("!dotfiles") {
                    client.say("rockzombie2".to_owned(), "You can find my dotfiles on !github: https://github.com/cpadilla/dotfiles".to_owned()).await.unwrap();
                }

                // Github
                if msg.message_text.to_lowercase().contains("!github") {
                    client.say("rockzombie2".to_owned(), "https://github.com/cpadilla".to_owned()).await.unwrap();
                }

                // socials
                if msg.message_text.to_lowercase().contains("!socials") ||
                    msg.message_text.to_lowercase().contains("!tiktok") ||
                    msg.message_text.to_lowercase().contains("!youtube") ||
                    msg.message_text.to_lowercase().contains("!twitter") {
                    client.say("rockzombie2".to_owned(), "Follow me on social media and checkout my !blog! https://twitter.com/rockzombie2 https://www.tiktok.com/@rockzombie2 https://youtube.com/rockzombie2".to_owned()).await.unwrap();
                }

                // blog
                if msg.message_text.to_lowercase().contains("!website") || msg.message_text.to_lowercase().contains("!blog") {
                    client.say("rockzombie2".to_owned(), "Reflections - https://christofer.rocks/".to_owned()).await.unwrap();
                }

                // commands
                if msg.message_text.to_lowercase().contains("!commands") {
                    client.say("rockzombie2".to_owned(), "See https://github.com/cpadilla/rockzombot2".to_owned()).await.unwrap();
                }

            },
            ServerMessage::Whisper(msg) => {
                println!("(w) {}: {}", msg.sender.name, msg.message_text);
            },
            _ => {}
        }

    }
}

async fn reminder(client: TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>) {
    let mut socials = false;
    let mut interval = time::interval(Duration::from_secs(900));
    loop {
        // Wait 15 minutes
        interval.tick().await;
        if socials {
            client.say("rockzombie2".to_owned(), "Follow me on social media and checkout my !blog! https://twitter.com/rockzombie2 https://www.tiktok.com/@rockzombie2 https://youtube.com/rockzombie2".to_owned()).await.unwrap();
            socials = !socials;
        } else {
            client.say("rockzombie2".to_owned(), "Join the Space Pirates Discord! https://discord.gg/ErM35TPK2D".to_owned()).await.unwrap();
            socials = !socials;
        }
    }
}
