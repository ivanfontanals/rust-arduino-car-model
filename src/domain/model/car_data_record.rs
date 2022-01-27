#[derive(Default, Clone, Copy, Debug)]
#[repr(C, packed(1))]
pub struct DataRecord {
    pub millis: u16,
    pub us_left: u16,
    pub us_center: u16,
    pub us_right: u16,
    pub pwmleft: u8,
    pub pwmright: u8,
    pub collision: u8,
    pub status: u8,
    pub direction: u8,
}
