//! Battery insulation leak detector

use mh8106f::System;

use crate::{CanBus, Part};

const PORT: u32 = 0x0080_0709; // P9DATA
const TEST: u8 = 0x02; // P9.1 output: the BMU switches in a test leak (~3.5 s to ~9 s after power-on)
const ALARM: u8 = 0x10; // P9.4 input, active low: leak detected

#[derive(Default)]
pub struct LeakDetector;

impl Part for LeakDetector {
    fn update(&mut self, chip: &mut System, _bus: &CanBus) {
        let testing = chip.gpio_level(PORT) & TEST != 0;
        chip.set_gpio_input(PORT, ALARM, if testing { 0 } else { ALARM });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::BMU_FW;

    #[test]
    fn alarms_only_for_the_self_test() {
        let mut bmu = System::new(BMU_FW);
        let bus = CanBus::default();
        LeakDetector.update(&mut bmu, &bus);
        assert_eq!(bmu.gpio_level(PORT) & ALARM, ALARM);
        bmu.set_gpio_input(PORT, TEST, TEST);
        LeakDetector.update(&mut bmu, &bus);
        assert_eq!(bmu.gpio_level(PORT) & ALARM, 0);
    }
}
