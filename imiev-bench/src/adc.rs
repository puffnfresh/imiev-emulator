//! ADC channel assignments, by firmware type.

const ADC_MIDSCALE: u16 = 0x800; // an undriven analog input floats to mid-scale
const SHUNT_ZERO_A: u16 = 0x200; // BMU main-current shunt at 2.5V -> 0 A
const SUPPLY_5V: u16 = 0x330; // a healthy ~5V sensor supply rail
const SUPPLY_12V_OK: u16 = 0x300; // an in-range reading for a 12V control rail

pub(crate) mod bmu_adc {
    pub const CONTROL_SUPPLY: usize = 0; // 12V control rail (DTC P1A4C out of 8..16V)
    pub const TEMP_SENSOR_1: usize = 1; // pack temperature sensor 1
    pub const PACK_CURRENT_HI: usize = 2; // main shunt, HIGH range (0x200 = 0 A)
    pub const PACK_CURRENT_LO: usize = 3; // main shunt, LOW range  (0x200 = 0 A)
    pub const RELAY_SUPPLY: usize = 4; // EV-control-relay 12V rail (DTC P1A4D)
    pub const TEMP_SENSOR_2: usize = 9; // pack temperature sensor 2 (same sample task as 1)
    pub const TEMP_SENSOR_3: usize = 0xB; // pack temperature sensor 3
}

pub(crate) mod ev_ecu_adc {
    pub const CONDENSER: usize = 0; // HV DC-link voltage sense
    pub const BRAKE_SUPPLY: usize = 1; // brake-stroke sensor 5V supply
    pub const ACCEL_1_SIGNAL: usize = 2; // accelerator sensor 1 (main) signal
    pub const BRAKE_SIGNAL: usize = 3; // brake-stroke sensor signal
    pub const VACUUM_SUPPLY: usize = 4; // brake-booster vacuum sensor 5V supply
    pub const ACCEL_2_SIGNAL: usize = 5; // accelerator sensor 2 (sub) signal
    pub const VACUUM_OUTPUT: usize = 6; // brake-booster vacuum sensor output
    pub const ACCEL_1_SUPPLY: usize = 10; // accelerator sensor 1 (main) 5V supply
    pub const ACCEL_2_SUPPLY: usize = 11; // accelerator sensor 2 (sub) 5V supply
    pub const CHARGE_PORT: usize = 12; // charge-port connection sense
}

pub(crate) const BMU_BOOT_ADC: &[(usize, u16)] = &[
    (bmu_adc::CONTROL_SUPPLY, SUPPLY_12V_OK),
    (bmu_adc::RELAY_SUPPLY, SUPPLY_12V_OK),
    (bmu_adc::TEMP_SENSOR_1, 0x330), // ~mid-range temperature
    (bmu_adc::TEMP_SENSOR_2, ADC_MIDSCALE),
    (bmu_adc::TEMP_SENSOR_3, ADC_MIDSCALE),
    (bmu_adc::PACK_CURRENT_HI, SHUNT_ZERO_A),
    (bmu_adc::PACK_CURRENT_LO, SHUNT_ZERO_A),
];

pub(crate) const EV_ECU_BOOT_ADC: &[(usize, u16)] = &[
    (ev_ecu_adc::CONDENSER, 0x000),       // HV DC-link discharged at power-on; charges during precharge
    (ev_ecu_adc::BRAKE_SUPPLY, SUPPLY_5V),
    (ev_ecu_adc::ACCEL_1_SIGNAL, 0x0c0),  // released accelerator, main
    (ev_ecu_adc::BRAKE_SIGNAL, 0x130),    // released brake (~1.5V)
    (ev_ecu_adc::VACUUM_SUPPLY, SUPPLY_5V),
    (ev_ecu_adc::ACCEL_2_SIGNAL, 0x060),  // released accelerator, sub (~1/2 of main)
    (ev_ecu_adc::VACUUM_OUTPUT, 0x200),   // mid-range vacuum reading (~2.5V)
    (ev_ecu_adc::ACCEL_1_SUPPLY, SUPPLY_5V),
    (ev_ecu_adc::ACCEL_2_SUPPLY, SUPPLY_5V),
    (ev_ecu_adc::CHARGE_PORT, 0x380),     // charge port disconnected
];
