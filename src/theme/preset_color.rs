use std::str::FromStr;

use anyhow::{Ok, Result};
use array_macro::array;
use palette::{convert::IntoColorUnclamped, Hsv, IntoColor, Srgb};

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

pub struct PresetColorPalettes {
    pub blue: [String; 10],
    pub purple: [String; 10],
    pub cyan: [String; 10],
    pub green: [String; 10],
    pub magenta: [String; 10],
    pub pink: [String; 10],
    pub red: [String; 10],
    pub orange: [String; 10],
    pub yellow: [String; 10],
    pub volcano: [String; 10],
    pub geekblue: [String; 10],
    pub gold: [String; 10],
    pub lime: [String; 10],
}

impl PresetColorPalettes {
    pub fn new(preset: PresetColor, dark: bool, bg_color: &str) -> Result<Self> {
        Ok(Self {
            blue: generate(&preset.blue, dark, bg_color)?,
            purple: generate(&preset.purple, dark, bg_color)?,
            cyan: generate(&preset.cyan, dark, bg_color)?,
            green: generate(&preset.green, dark, bg_color)?,
            magenta: generate(&preset.magenta, dark, bg_color)?,
            pink: generate(&preset.pink, dark, bg_color)?,
            red: generate(&preset.red, dark, bg_color)?,
            orange: generate(&preset.orange, dark, bg_color)?,
            yellow: generate(&preset.yellow, dark, bg_color)?,
            volcano: generate(&preset.volcano, dark, bg_color)?,
            geekblue: generate(&preset.geekblue, dark, bg_color)?,
            gold: generate(&preset.gold, dark, bg_color)?,
            lime: generate(&preset.lime, dark, bg_color)?,
        })
    }

    pub fn light() -> Self {
        Self::new(PresetColor::default(), false, "").unwrap()
    }

    pub fn dark() -> Self {
        Self::new(PresetColor::default(), true, "").unwrap()
    }
}

///
///  https://github.com/ant-design/ant-design-colors/blob/master/src/generate.ts
///
const HUE_STEP: f32 = 2.0; // 色相阶梯
const SATURATION_STEP: f32 = 0.16; // 饱和度阶梯，浅色部分
const SATURATION_STEP2: f32 = 0.05; // 饱和度阶梯，深色部分
const BRIGHTNESS_STEP1: f32 = 0.05; // 亮度阶梯，浅色部分
const BRIGHTNESS_STEP2: f32 = 0.15; // 亮度阶梯，深色部分
const LIGHT_COLOR_COUNT: usize = 5; // 浅色数量，主色上
const DARK_COLOR_COUNT: usize = 4; // 深色数量，主色下
const DARK_COLOR_MAP: [(usize, f32); 10] = [
    // 暗色主题颜色映射关系表
    (7, 0.15),
    (6, 0.25),
    (5, 0.3),
    (5, 0.45),
    (5, 0.65),
    (5, 0.85),
    (4, 0.9),
    (3, 0.95),
    (2, 0.97),
    (1, 0.98),
];

pub fn generate(color: &str, dark: bool, bg_color: &str) -> Result<[String; 10]> {
    let mut patterns = array!(String::new(); 10);

    for (index, iter) in patterns[0..LIGHT_COLOR_COUNT].iter_mut().enumerate() {
        let level = LIGHT_COLOR_COUNT - index;

        let srgb = Srgb::from_str(color)?;
        let hsv: Hsv = srgb.into_format().into_color();

        let hue = get_hue(&hsv, level, true);
        let saturation = get_saturation(&hsv, level, true);
        let value = get_value(&hsv, level, true);
        let hsv = Hsv::new(hue, saturation, value);

        let srgb: Srgb = hsv.into_color_unclamped();
        let srgb: Srgb<u8> = srgb.into_format();

        *iter = format!("#{srgb:02X}");
    }

    patterns[5] = color.to_string();

    for (index, iter) in (patterns[6..DARK_COLOR_COUNT + 6]).iter_mut().enumerate() {
        let level = index + 1;

        let srgb = Srgb::from_str(color)?;
        let hsv: Hsv = srgb.into_format().into_color();

        let hue = get_hue(&hsv, level, false);
        let saturation = get_saturation(&hsv, level, false);
        let value = get_value(&hsv, level, false);
        let hsv = Hsv::new(hue, saturation, value);

        let srgb: Srgb = hsv.into_color_unclamped();
        let srgb: Srgb<u8> = srgb.into_format();

        *iter = format!("#{srgb:02X}");
    }

    if dark {
        let mut bg_color = bg_color;
        if bg_color.is_empty() {
            bg_color = "#141414"
        }
        let mut dark_patterns = patterns.clone();
        for (d_index, (index, opacity)) in DARK_COLOR_MAP.iter().enumerate() {
            dark_patterns[d_index] = get_mix(bg_color, &patterns[*index], opacity * 100.0)?
        }
        return Ok(dark_patterns);
    }

    Ok(patterns)
}

fn get_hue(hsv: &Hsv, level: usize, light: bool) -> f32 {
    let orig_hue = hsv.hue.to_positive_degrees();

    let hue = if orig_hue > 60.0 && orig_hue <= 240.0 {
        if light {
            orig_hue - HUE_STEP * level as f32
        } else {
            orig_hue + HUE_STEP * level as f32
        }
    } else if light {
        orig_hue + HUE_STEP * level as f32
    } else {
        orig_hue - HUE_STEP * level as f32
    };

    ((hue + 360.0) % 360.0).round()
}

fn get_saturation(hsv: &Hsv, level: usize, light: bool) -> f32 {
    let mut saturation = hsv.saturation;
    let orig_hue = hsv.hue.to_positive_degrees();

    if orig_hue == 0.0 && saturation == 0.0 {
        return saturation;
    }

    if light {
        saturation = hsv.saturation - SATURATION_STEP * level as f32;
    } else if level == DARK_COLOR_COUNT {
        saturation = hsv.saturation + SATURATION_STEP;
    } else {
        saturation = hsv.saturation + SATURATION_STEP2 * level as f32;
    }

    // 边界值修正
    if saturation > 1.0 {
        saturation = 1.0
    }

    if light && level == LIGHT_COLOR_COUNT && saturation > 0.1 {
        saturation = 0.1;
    }
    if saturation < 0.06 {
        saturation = 0.06;
    }

    (saturation * 100.0).round() / 100.0
}

fn get_value(hsv: &Hsv, level: usize, light: bool) -> f32 {
    let mut value = if light {
        hsv.value + BRIGHTNESS_STEP1 * level as f32
    } else {
        hsv.value - BRIGHTNESS_STEP2 * level as f32
    };
    if value > 1.0 {
        value = 1.0;
    }
    (value * 100.0).round() / 100.0
}

fn get_mix(bg_color: &str, color: &str, amount: f32) -> Result<String> {
    let p = amount / 100.0;

    let bg_srgb = Srgb::from_str(bg_color)?;
    let srgb = Srgb::from_str(color)?;

    let red = ((srgb.red as f32 - bg_srgb.red as f32) * p) as u8 + bg_srgb.red;
    let green = ((srgb.green as f32 - bg_srgb.green as f32) * p) as u8 + bg_srgb.green;
    let blue = ((srgb.blue as f32 - bg_srgb.blue as f32) * p) as u8 + bg_srgb.blue;
    let srgb = Srgb::new(red, green, blue);

    Ok(format!("#{srgb:02X}"))
}

#[test]
fn it_works() -> Result<()> {
    let colors = generate("#2F54EB", false, "")?;
    println!("{colors:?}");

    Ok(())
}
