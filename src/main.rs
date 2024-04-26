pub mod heaters;
use std::ops::Deref;

use as_any::{AsAny, Downcast};
use heaters::HeatersController;
use tokio::sync::mpsc;

use crate::heaters::HeaterAction;

pub trait DeviceActionTrait: Send + AsAny {}

async fn receiver_loop(mut receiver: mpsc::Receiver<Box<dyn DeviceActionTrait>>) {
    let mut heater = HeatersController { power: false };

    while let Some(message) = receiver.recv().await {
        if let Some(message) = message.deref().downcast_ref::<HeaterAction>() {
            match message {
                HeaterAction::SetStatus(status) => {
                    heater.power = *status;
                    println!("Heater power set to: {}", heater.power);
                }
                HeaterAction::GetStatus => {
                    println!("Heater power is: {}", heater.power);
                }
            }
        }
    }
}

async fn sender_loop(sender: mpsc::Sender<Box<dyn DeviceActionTrait>>) {
    let a1 = HeaterAction::GetStatus;
    let a2 = HeaterAction::SetStatus(true);

    let vec: Vec<Box<dyn DeviceActionTrait>> = vec![Box::new(a1), Box::new(a2)];

    for action in vec {
        sender.send(action).await.unwrap();
    }
}

#[tokio::main]
async fn main() {
    let (sender, receiver) = mpsc::channel(10);

    let h1 = tokio::spawn(async move {
        sender_loop(sender).await;
    });

    let h2 = tokio::spawn(async move {
        receiver_loop(receiver).await;
    });

    h1.await.unwrap();
    h2.await.unwrap();
}
