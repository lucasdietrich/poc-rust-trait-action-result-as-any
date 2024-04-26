use crate::{DeviceActionResultTrait, DeviceActionTrait, DeviceTrait};

pub struct HeatersController {
    pub power: bool,
}

pub struct HeaterResult {
    pub power: bool,
}

pub enum HeaterAction {
    SetStatus(bool),
    GetStatus,
}

impl DeviceActionTrait for HeaterAction {
    type Result = HeaterResult;
}

impl DeviceActionResultTrait for HeaterResult {}

impl DeviceTrait for HeatersController {
    type Action = HeaterAction;

    fn handle_action(&mut self, action: &HeaterAction) -> HeaterResult {
        match action {
            HeaterAction::SetStatus(status) => {
                self.power = *status;
                println!("ACTION Heater power set to: {}", self.power);
            }
            HeaterAction::GetStatus => {
                println!("ACTION Heater power is: {}", self.power);
            }
        }

        HeaterResult { power: self.power }
    }
}
