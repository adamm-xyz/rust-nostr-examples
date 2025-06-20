//! NIP01, publishing a basic text note
use nostr::prelude::*;
use tungstenite::{connect, Message};
use std::error::Error;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let keys = Keys::generate();
    
    // Convert public key to bech32
    println!("Public key: {}", keys.public_key().to_bech32()?);
    
    // New text note
    let event: Event = EventBuilder::text_note("Hello world").sign_with_keys(&keys)?;
    
    // Convert client message to JSON
    let json: Value = serde_json::from_str(&ClientMessage::event(event.clone()).as_json()).unwrap();
    let pretty_json = serde_json::to_string_pretty(&json).unwrap();
    println!("{}",pretty_json);
    
    // Send to relay and wait for response
    let accepted = send_event_to_relay(event, "wss://relay.damus.io")?;
    
    if accepted {
        println!("Event was accepted by the relay!");
    } else {
        println!("Event was rejected by the relay.");
    }
    
    Ok(())
}

fn send_event_to_relay(event: Event, relay_url: &str) -> Result<bool, Box<dyn Error>> {
    // Connect to the relay
    let (mut socket, _response) = connect(relay_url)?;
    
    // Convert client message to JSON
    let json = ClientMessage::event(event).as_json();
    
    // Send the event (use `send` instead of deprecated `write_message`, and convert String to bytes)
    socket.send(Message::Text(json.into()))?;
    
    // Wait for response
    loop {
        let msg = socket.read()?; // Use `read` instead of deprecated `read_message`
        match msg {
            Message::Text(text) => {
                println!("Received: {}", text);
                
                // Parse the relay message
                match RelayMessage::from_json(&text) {
                    Ok(relay_msg) => {
                        match relay_msg {
                            RelayMessage::Ok { event_id, status, message } => {
                                println!("Event {} - Status: {}", event_id, status);
                                // `message` is a Cow<str>, not Option<String>
                                if !message.is_empty() {
                                    println!("Message: {}", message);
                                }
                                return Ok(status);
                            }
                            // Notice is a tuple variant, not struct variant
                            RelayMessage::Notice(message) => {
                                println!("Notice from relay: {}", message);
                            }
                            _ => {
                                // Handle other message types if needed
                            }
                        }
                    }
                    Err(e) => {
                        println!("Failed to parse relay message: {}", e);
                    }
                }
            }
            Message::Close(_) => {
                println!("Connection closed by relay");
                break;
            }
            _ => {}
        }
    }
    
    Ok(false)
}
