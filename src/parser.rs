pub enum BreakType {
    SoftLineBreak, // softbreak
    HardLineBreak, // Hard line breaks
}

pub struct Config {
    break_type: BreakType,
}
