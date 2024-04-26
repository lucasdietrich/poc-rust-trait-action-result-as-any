use crate::DeviceActionTrait;

pub struct HeatersController {
    pub power: bool,
}

pub struct Status {
    pub power: bool,
}

pub enum HeaterAction {
    SetStatus(bool),
    GetStatus,
}

impl DeviceActionTrait for HeaterAction {}
