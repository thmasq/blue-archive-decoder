use fontdue::{Font, FontSettings};
use std::sync::OnceLock;
use unicode_linebreak::{BreakOpportunity, linebreaks};

const FONT_EN_BYTES: &[u8] = include_bytes!("../fonts/NotoSans-Regular.ttf");
const FONT_JP_BYTES: &[u8] = include_bytes!("../fonts/NotoSansJP-Regular.ttf");
const FONT_KR_BYTES: &[u8] = include_bytes!("../fonts/NotoSansKR-Regular.ttf");

struct FontSet {
    en: Font,
    jp: Font,
    kr: Font,
}

static FONTS: OnceLock<FontSet> = OnceLock::new();

fn get_fonts() -> &'static FontSet {
    FONTS.get_or_init(|| {
        let settings = FontSettings::default();
        FontSet {
            en: Font::from_bytes(FONT_EN_BYTES, settings.clone()).unwrap(),
            jp: Font::from_bytes(FONT_JP_BYTES, settings.clone()).unwrap(),
            kr: Font::from_bytes(FONT_KR_BYTES, settings).unwrap(),
        }
    })
}

pub fn measure_text_height(text: &str, max_width: f32, font_size: f32) -> f32 {
    if text.is_empty() || max_width <= 0.5 {
        return 30.0;
    }

    let fonts = get_fonts();

    let line_height = font_size * 1.25;
    let vertical_padding = 12.0;

    let mut lines = 1;
    let mut current_line_width = 0.0;
    let mut last_offset = 0;

    let text_len = text.len();

    for (offset, opportunity) in linebreaks(text) {
        let chunk = &text[last_offset..offset];
        let chunk_width = measure_chunk_width(chunk, fonts, font_size);

        if current_line_width + chunk_width > max_width {
            if current_line_width > 0.0 {
                lines += 1;
            }

            if chunk_width > max_width {
                let chunk_lines = (chunk_width / max_width).ceil() as usize;

                if chunk_lines > 0 {
                    lines += chunk_lines - 1;
                }

                let remainder = chunk_width % max_width;
                current_line_width = if remainder == 0.0 {
                    max_width
                } else {
                    remainder
                };
            } else {
                current_line_width = chunk_width;
            }
        } else {
            current_line_width += chunk_width;
        }

        if let BreakOpportunity::Mandatory = opportunity {
            if offset < text_len {
                lines += 1;
                current_line_width = 0.0;
            }
        }

        last_offset = offset;
    }

    (lines as f32 * line_height) + vertical_padding
}

fn measure_chunk_width(text: &str, fonts: &FontSet, px: f32) -> f32 {
    let mut width = 0.0;
    for c in text.chars() {
        if c.is_control() {
            continue;
        }

        let (metrics, _) = get_char_metrics(c, fonts, px);
        width += metrics.advance_width;
    }
    width
}

fn get_char_metrics(c: char, fonts: &FontSet, px: f32) -> (fontdue::Metrics, &Font) {
    if let Some(idx) = fonts.en.chars().get(&c) {
        return (fonts.en.metrics_indexed(idx.get(), px), &fonts.en);
    }

    if let Some(idx) = fonts.jp.chars().get(&c) {
        return (fonts.jp.metrics_indexed(idx.get(), px), &fonts.jp);
    }

    if let Some(idx) = fonts.kr.chars().get(&c) {
        return (fonts.kr.metrics_indexed(idx.get(), px), &fonts.kr);
    }

    let idx = fonts.en.chars().get(&'?').map(|nz| nz.get()).unwrap_or(0);

    (fonts.en.metrics_indexed(idx, px), &fonts.en)
}
