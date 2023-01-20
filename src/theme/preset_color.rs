#[derive(Debug)]
pub struct PresetColor {
    pub blue: String,
    pub purple: String,
    pub cyan: String,
    pub green: String,
    pub magenta: String,
    pub pink: String,
    pub red: String,
    pub orange: String,
    pub yellow: String,
    pub volcano: String,
    pub geekblue: String,
    pub gold: String,
    pub lime: String,
}

impl Default for PresetColor {
    fn default() -> Self {
        Self {
            blue: "#1677ff".into(),
            purple: "#722ED1".into(),
            cyan: "#13C2C2".into(),
            green: "#52C41A".into(),
            magenta: "#EB2F96".into(),
            pink: "#eb2f96".into(),
            red: "#F5222D".into(),
            orange: "#FA8C16".into(),
            yellow: "#FADB14".into(),
            volcano: "#FA541C".into(),
            geekblue: "#2F54EB".into(),
            gold: "#FAAD14".into(),
            lime: "#A0D911".into(),
        }
    }
}
