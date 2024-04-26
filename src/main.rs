pub mod heaters;
use std::ops::Deref;

use as_any::{AsAny, Downcast};
use heaters::HeatersController;
use tokio::sync::mpsc;

use crate::heaters::HeaterAction;

pub trait DeviceTrait {
    type Action: DeviceActionTrait;

    fn handle_action(&mut self, action: &Self::Action) -> <Self::Action as DeviceActionTrait>::Result;
}

pub trait DeviceActionTrait: Send + AsAny {
    type Result: DeviceActionResultTrait;
}

pub trait DeviceActionWrapperTrait: Send + AsAny {}
impl<T: DeviceActionTrait> DeviceActionWrapperTrait for T {}

pub trait DeviceActionResultTrait: Send + AsAny {}

async fn receiver_loop<D: DeviceTrait>(
    device: &mut D,
    mut q_receiver: mpsc::Receiver<Box<dyn DeviceActionWrapperTrait>>,
    r_sender: mpsc::Sender<Box<dyn DeviceActionResultTrait>>,
) {
    while let Some(message) = q_receiver.recv().await {
        if let Some(action) = message.deref().downcast_ref::<D::Action>() {
            let result = device.handle_action(action);
            r_sender.send(Box::new(result)).await.unwrap();
        }
    }
}

async fn sender_loop<A: DeviceActionTrait>(
    q_sender: mpsc::Sender<Box<dyn DeviceActionWrapperTrait>>,
    mut r_receiver: mpsc::Receiver<Box<dyn DeviceActionResultTrait>>,
    actions: Vec<A>,
) {
    for action in actions {
        q_sender.send(Box::new(action)).await.unwrap();
    }

    while let Some(result) = r_receiver.recv().await {
        if let Some(result) = result.deref().downcast_ref::<heaters::HeaterResult>() {
            println!("RESULT Heater power is: {}", result.power);
        }
    }
}

#[tokio::main]
async fn main() {
    let (q_sender, q_receiver) = mpsc::channel(10);
    let (r_sender, r_receiver) = mpsc::channel(10);

    let actions = vec![
        HeaterAction::GetStatus,
        HeaterAction::SetStatus(true)
    ];

    let h1 = tokio::spawn(async move {
        sender_loop(q_sender, r_receiver, actions).await;
    });

    let mut heater = HeatersController { power: false };

    let h2 = tokio::spawn(async move {
        receiver_loop(&mut heater, q_receiver, r_sender).await;
    });

    h1.await.unwrap();
    h2.await.unwrap();
}
