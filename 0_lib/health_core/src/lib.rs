#![no_std]
pub struct HealthMonitor { pub integrity: u8 }
impl HealthMonitor {
    pub fn new() -> Self { HealthMonitor { integrity: 255 } }
}