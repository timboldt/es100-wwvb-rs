pub const ES100_I2C_DEFAULT_ADDRESS: u8 = 0x32;

//
// ES100 API register addresses
//

// TODO: Document these.
pub const ES100_CONTROL_0_REG: u8 = 0x0;
pub const ES100_CONTROL_1_REG: u8 = 0x1;
pub const ES100_IRQ_STATUS_REG: u8 = 0x2;
pub const ES100_STATUS_0_REG: u8 = 0x3;
pub const ES100_YEAR_REG: u8 = 0x4;
pub const ES100_MONTH_REG: u8 = 0x5;
pub const ES100_DAY_REG: u8 = 0x6;
pub const ES100_HOUR_REG: u8 = 0x7;
pub const ES100_MINUTE_REG: u8 = 0x8;
pub const ES100_SECOND_REG: u8 = 0x9;
pub const ES100_NEXT_DST_MONTH_REG: u8 = 0xA;
pub const ES100_NEXT_DST_DAY_REG: u8 = 0xB;
pub const ES100_NEXT_DST_HOUR_REG: u8 = 0xC;
pub const ES100_DEVICE_ID_REG: u8 = 0xD;

//
// CONTROL_0 register bits.
//

// TODO: Document these.
pub const ES100_CONTROL_0_REG_START: u8 = 0;
pub const ES100_CONTROL_0_REG_ANT1_OFF: u8 = 1;
pub const ES100_CONTROL_0_REG_ANT2_OFF: u8 = 2;
pub const ES100_CONTROL_0_REG_START_ANTENNA: u8 = 3;
pub const ES100_CONTROL_0_REG_TRACKING_ENABLE: u8 = 4;

//
// IRQ_STATUS register bits.
//

// TODO: Document these.
pub const ES100_IRQ_STATUS_REG_RX_COMPLETE: u8 = 0;
pub const ES100_IRQ_STATUS_REG_CYCLE_COMPLETE: u8 = 2;

//
// STATUS_0 register bits.
//

/// Data in the struct is only valid when rxOk = 1.
// 0 (0x0)  Indicates that a successful reception has not occured.
// 1 (0x1)  Indicated that a successful reception has occured.
pub const ES100_STATUS_0_REG_RX_OK: u8 = 0;

// 0 (0x0)  Indicates that the reception occured on Antenna 1.
// 1 (0x1)  Indicates that the reception occured on Antenna 2.
pub const ES100_STATUS_0_REG_ANT: u8 = 1;

// 0 (0x00) Indicates that the current month WILL NOT have a leap second.
// 2 (0x10) Indicates that the current month WILL have a negative leap second.
// 3 (0x11) Indicates that the current month WILL have a positive leap second.
pub const ES100_STATUS_0_REG_LSW0: u8 = 3;
pub const ES100_STATUS_0_REG_LSW1: u8 = 4;

// 0 (0x00) Indicates that Daylight Savings Time (DST) is not in effect.
// 1 (0x01) Indicates that DST ends today.
// 2 (0x10) Indicates that DST begins totay.
// 3 (0x11) Indicates that DST is in effect.
pub const ES100_STATUS_0_REG_DST0: u8 = 5;
pub const ES100_STATUS_0_REG_DST1: u8 = 6;

// 0 (0x0)  Indicates that the reception attenpt was a 1-minute frame operation.
// 1 (0x1)  Indicates that the reception attemps was a tracking operation.
pub const ES100_STATUS_0_REG_TRACKING: u8 = 7;
