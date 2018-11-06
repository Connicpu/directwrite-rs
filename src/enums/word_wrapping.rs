#[auto_enum(u32, checked)]
pub enum WordWrapping {
    Wrap = 0,
    NoWrap = 1,
    EmergencyBreak = 2,
    WholeWord = 3,
    Character = 4,
}
