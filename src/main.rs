use anyhow::{anyhow, Context};
use std::env;
use std::io::{prelude::*, BufReader};
use std::os::unix::net::UnixStream;

use clap::Parser;


/// IPC eventlistner to record hyperland events for further processing
#[derive(Parser)]
#[command(version, about, long_about= None)]
struct Cli {

    /// Target hyprland-event to listen to
    event: String,
}

#[derive(Debug)]
struct Message {
    event: String,
    payload: String,
}

fn read_message(input: String) -> anyhow::Result<Message> {

    let parts: Vec<&str> = input.split(">>").collect();
    if parts.len() != 2 {
        return Err(anyhow!("message does not comply to format: EVENT>>DATA"));
    }
    let event = parts[0].to_owned();
    let payload = parts[1].to_owned();

    return Ok(Message { event, payload });
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let target_event = cli.event;

    let his = env::var("HYPRLAND_INSTANCE_SIGNATURE")
        .context("Environmenvariable not set: HYPRLAND_INSTANCE_SIGNATURE")?;
    let stream = UnixStream::connect(format!("/tmp/hypr/{}/.socket2.sock", his))
        .context("Failed to connect to hyprland-unix socket")?;
    let reader = BufReader::new(stream);
    for line in reader.lines() {
        let line_str = line.context("Failed to read line from socket stream")?;
        let message = read_message(line_str)?;
        if message.event == target_event {
            println!("{}", message.payload);
        }
    }
    Ok(())
}
