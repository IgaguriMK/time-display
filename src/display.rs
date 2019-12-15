use std::collections::BTreeMap;

use image::{load_from_memory, RgbaImage};
use image::imageops::overlay;
use tiny_fail::{Fail, FailExt};

#[derive(Debug, Clone)]
pub struct Display {
    null: RgbaImage,
    colon: RgbaImage,
    dot: RgbaImage,
    digits: BTreeMap<u64, RgbaImage>,
}

impl Display {
    pub fn builtin() -> Display {
        let null = parse_memory(include_bytes!("../digits/null.png"), "null");
        let colon = parse_memory(include_bytes!("../digits/min.png"), "min");
        let dot = parse_memory(include_bytes!("../digits/sec.png"), "sec");

        let mut digits = BTreeMap::new();
        digits.insert(0, parse_memory(include_bytes!("../digits/0.png"), "0"));
        digits.insert(1, parse_memory(include_bytes!("../digits/1.png"), "1"));
        digits.insert(2, parse_memory(include_bytes!("../digits/2.png"), "2"));
        digits.insert(3, parse_memory(include_bytes!("../digits/3.png"), "3"));
        digits.insert(4, parse_memory(include_bytes!("../digits/4.png"), "4"));
        digits.insert(5, parse_memory(include_bytes!("../digits/5.png"), "5"));
        digits.insert(6, parse_memory(include_bytes!("../digits/6.png"), "6"));
        digits.insert(7, parse_memory(include_bytes!("../digits/7.png"), "7"));
        digits.insert(8, parse_memory(include_bytes!("../digits/8.png"), "8"));
        digits.insert(9, parse_memory(include_bytes!("../digits/9.png"), "9"));

        Display {
            null,
            colon,
            dot,
            digits,
        }
    }

    pub fn print(&self, s: &str) -> Result<RgbaImage, Fail> {
        let mut images = Vec::new();

        for ch in s.chars() {
            let img = self.get_ch(ch)?;
            images.push(img);
        }

        let (mut w, h) = images.get(0).context("no chars to print")?.dimensions();
        for &img in images.iter().skip(1) {
            let (wx, hx) = img.dimensions();
            if hx != hx {
                return Err(Fail::new("image height mismatch"));
            }
            w += wx;
        }

        let mut res = RgbaImage::new(w, h);

        let mut offset = 0u32;
        for img in images {
            overlay(&mut res, img, offset, 0);
            offset += img.dimensions().0;
        }

        Ok(res)
    }

    fn get_ch(&self, ch: char) -> Result<&RgbaImage, Fail> {
        match ch {
            ' ' => Ok(&self.null),
            ':' => Ok(&self.colon),
            '.' => Ok(&self.dot),
            ch if ch.is_ascii_digit() => {
                let d = ch.to_digit(10).unwrap() as u64;
                self.digits.get(&d).context(format!("digit {} not exists.", d))
            }
            ch => Err(Fail::new(format!("char '{}' not exists.", ch))),
        }
    }
}

fn parse_memory(bs: &[u8], name: &str) -> RgbaImage {
    load_from_memory(bs)
        .context(format!("parsing {}.png", name))
        .unwrap()
        .to_rgba()
}
