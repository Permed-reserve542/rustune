/// Skin layout zones for bitmap-based Winamp rendering.
///
/// Defines the Winamp 2.x pixel-coordinate regions within MAIN.BMP and maps
/// them to terminal rows. Used by the bitmap renderer to paint chrome from
/// actual skin bitmaps and overlay dynamic content on top.

use std::collections::HashMap;

use ratatui::layout::Rect;

use crate::skin::{BmpImage, WinampSkin};

/// A rectangle in BMP pixel coordinates.
#[derive(Debug, Clone, Copy)]
pub struct BmpRect {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

/// Semantic zones in the Winamp main window.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ZoneKind {
    TitleBar,
    ClutterBar,
    LedTime,
    Spectrum,
    Marquee,
    SeekBar,
    Transport,
    Volume,
    Balance,
    Status,
}

/// One zone: where to crop from MAIN.BMP, which terminal row and column offset.
#[derive(Debug, Clone, Copy)]
pub struct SkinZone {
    pub src_rect: BmpRect,
    pub terminal_row: u16,
    pub col_start: u16,
}

/// Computed layout for a fully-loaded skin.
pub struct SkinLayout {
    pub zones: HashMap<ZoneKind, SkinZone>,
    /// Additional BMP images (not from MAIN.BMP).
    pub numbers: BmpImage,
    pub cbuttons: BmpImage,
    pub posbar: BmpImage,
    pub text_bmp: BmpImage,
    pub playpaus: BmpImage,
    pub titlebar_bmp: BmpImage,
    pub monoster: Option<BmpImage>,
    pub shufrep: Option<BmpImage>,
    pub volume_bmp: Option<BmpImage>,
}

impl SkinLayout {
    /// Build a layout from a loaded skin. Returns `None` if any required BMP
    /// is missing or has wrong dimensions (all-or-nothing check).
    pub fn from_skin(skin: &WinampSkin) -> Option<Self> {
        // Required BMPs from MAIN.BMP
        let main = skin.main_bitmap.as_ref()?;
        validate_dims(main, 275, 116)?;

        // Required standalone BMPs
        let numbers = skin.numbers_bitmap.as_ref()?;
        validate_dims(numbers, 9, 13)?;

        let cbuttons = skin.cbuttons_bitmap.as_ref()?;
        validate_dims(cbuttons, 22, 18)?;

        let posbar = skin.posbar_bitmap.as_ref()?;
        validate_dims(posbar, 10, 10)?;

        let text_bmp = skin.text_bitmap.as_ref()?;
        validate_dims(text_bmp, 5, 5)?;

        let playpaus = skin.playpaus_bitmap.as_ref()?;
        validate_dims(playpaus, 2, 9)?;

        let titlebar_bmp = skin.titlebar_bitmap.as_ref()?;
        validate_dims(titlebar_bmp, 2, 2)?;

        // Optional BMPs — no dimension check needed, None is fine
        let monoster = skin.monoster_bitmap.clone();
        let shufrep = skin.shufrep_bitmap.clone();
        let volume_bmp = skin.volume_bitmap.clone();

        // Build zone map with Winamp 2.x MAIN.BMP pixel coordinates
        let zones = build_zones();

        Some(Self {
            zones,
            numbers: numbers.clone(),
            cbuttons: cbuttons.clone(),
            posbar: posbar.clone(),
            text_bmp: text_bmp.clone(),
            playpaus: playpaus.clone(),
            titlebar_bmp: titlebar_bmp.clone(),
            monoster,
            shufrep,
            volume_bmp,
        })
    }

    /// Get the zone info for a given kind.
    #[allow(dead_code)]
    pub fn zone(&self, kind: ZoneKind) -> Option<SkinZone> {
        self.zones.get(&kind).copied()
    }

    /// Compute the terminal Rect for a zone within the main window area.
    #[allow(dead_code)]
    pub fn zone_rect(&self, kind: ZoneKind, main_area: Rect) -> Rect {
        let zone = self.zones.get(&kind);
        match zone {
            Some(z) => {
                let y = main_area.y + z.terminal_row;
                let x = main_area.x + z.col_start;
                Rect {
                    x,
                    y,
                    width: main_area.width.saturating_sub(z.col_start),
                    height: 1,
                }
            }
            None => Rect::default(),
        }
    }
}

fn validate_dims(bmp: &BmpImage, min_w: u32, min_h: u32) -> Option<()> {
    if bmp.width >= min_w && bmp.height >= min_h {
        Some(())
    } else {
        None
    }
}

fn build_zones() -> HashMap<ZoneKind, SkinZone> {
    let mut m = HashMap::new();
    m.insert(
        ZoneKind::TitleBar,
        SkinZone {
            src_rect: BmpRect {
                x: 0,
                y: 0,
                w: 275,
                h: 20,
            },
            terminal_row: 0,
            col_start: 0,
        },
    );
    m.insert(
        ZoneKind::ClutterBar,
        SkinZone {
            src_rect: BmpRect {
                x: 9,
                y: 6,
                w: 16,
                h: 16,
            },
            terminal_row: 1,
            col_start: 0,
        },
    );
    m.insert(
        ZoneKind::LedTime,
        SkinZone {
            src_rect: BmpRect {
                x: 9,
                y: 26,
                w: 62,
                h: 12,
            },
            terminal_row: 1,
            col_start: 3,
        },
    );
    m.insert(
        ZoneKind::Spectrum,
        SkinZone {
            src_rect: BmpRect {
                x: 78,
                y: 22,
                w: 197,
                h: 28,
            },
            terminal_row: 1,
            col_start: 11,
        },
    );
    m.insert(
        ZoneKind::Marquee,
        SkinZone {
            src_rect: BmpRect {
                x: 9,
                y: 53,
                w: 257,
                h: 12,
            },
            terminal_row: 2,
            col_start: 0,
        },
    );
    m.insert(
        ZoneKind::SeekBar,
        SkinZone {
            src_rect: BmpRect {
                x: 16,
                y: 72,
                w: 244,
                h: 6,
            },
            terminal_row: 3,
            col_start: 0,
        },
    );
    m.insert(
        ZoneKind::Transport,
        SkinZone {
            src_rect: BmpRect {
                x: 0,
                y: 57,
                w: 136,
                h: 36,
            },
            terminal_row: 4,
            col_start: 0,
        },
    );
    m.insert(
        ZoneKind::Volume,
        SkinZone {
            src_rect: BmpRect {
                x: 136,
                y: 57,
                w: 68,
                h: 36,
            },
            terminal_row: 4,
            col_start: 20,
        },
    );
    m.insert(
        ZoneKind::Balance,
        SkinZone {
            src_rect: BmpRect {
                x: 204,
                y: 57,
                w: 68,
                h: 36,
            },
            terminal_row: 4,
            col_start: 30,
        },
    );
    m.insert(
        ZoneKind::Status,
        SkinZone {
            src_rect: BmpRect {
                x: 0,
                y: 93,
                w: 275,
                h: 23,
            },
            terminal_row: 5,
            col_start: 0,
        },
    );
    m
}
