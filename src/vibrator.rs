use async_std::io;
use std::sync::{Arc, Mutex};
use std::{thread, time::Duration};
use buttplug::{client::{device::ScalarValueCommand, ButtplugClientEvent, ButtplugClient}, util::in_process_client, core::connector::new_json_ws_client_connector};
use futures::StreamExt;
use tokio::io::{AsyncBufReadExt, BufReader};

async fn wait_for_input() {
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.read_line(&mut line).await;
}

pub async fn vibrate() -> anyhow::Result<()> {
  let connector = new_json_ws_client_connector("ws://127.0.0.1:12345/buttplug");
  let client = ButtplugClient::new("cargo-buttplug");
  client.connect(connector).await?;
  let mut events = client.event_stream();

  tokio::spawn(async move {
    while let Some(event) = events.next().await {
      match event {
        ButtplugClientEvent::DeviceAdded(device) => {
          println!("device {} connected", device.name());
        }
        ButtplugClientEvent::DeviceRemoved(info) => {
          println!("device {} removed", info.name());
        }
        ButtplugClientEvent::ScanningFinished => {
          println!("device scanning is finished!");
        }
        _ => {}
      }
    }
  });

  client.start_scanning().await?;

client.stop_scanning().await?;
thread::sleep(Duration::from_secs(3));
// maybe make it wait a bit to scan or smth

// if no devices are connected, we can't do anything gg
if client.devices().is_empty() {
    println!("No devices connected, exiting");
    return Ok(());
}

println!("Vibrating:");
for device in client.devices() {
    println!("- {}", device.name());
}

let stop = Arc::new(Mutex::new(false));
let clone: Arc<Mutex<bool>> = Arc::clone(&stop);

thread::spawn(move || {
    thread::sleep(Duration::from_secs(10));
    let mut s = clone.lock().unwrap();
    *s = true;
});

// we will have flying cars in 2024
// 2024:
// HHAHHAAH MULTITHREADED BUTTPLUGGING

let test_device = &client.devices()[0];
loop { // this needs to be 10s only
    test_device
        .vibrate(&ScalarValueCommand::ScalarValue(1.0))
        .await?;
    let should_stop = stop.lock().unwrap();
    if *should_stop {
        break;
    }
}

  client.disconnect().await?;

  Ok(())
}