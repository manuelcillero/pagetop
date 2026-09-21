use pagetop::prelude::*;

// The background is made of `rect`s; only the logo paths use the line color.
fn line_color(bg: (u8, u8, u8)) -> &'static str {
    let svg = PageTopSvg::TileRGB(bg.0, bg.1, bg.2).markup().into_string();
    let dark = svg.contains("<path fill=\"rgb(10,11,9)\"");
    let white = svg.contains("<path fill=\"rgb(255,255,255)\"");
    assert_ne!(dark, white, "the line must use exactly one color");
    if dark { "dark" } else { "white" }
}

// **< PageTopSvg::TileRGB >************************************************************************

#[pagetop::test]
async fn tile_rgb_uses_a_white_line_on_dark_backgrounds() {
    assert_eq!(line_color((0, 82, 204)), "white");
    assert_eq!(line_color((0, 0, 0)), "white");
    assert_eq!(line_color((220, 20, 60)), "white");
}

#[pagetop::test]
async fn tile_rgb_uses_a_dark_line_on_light_backgrounds() {
    assert_eq!(line_color((255, 184, 75)), "dark");
    assert_eq!(line_color((0, 255, 0)), "dark");
    assert_eq!(line_color((255, 255, 255)), "dark");
}

#[pagetop::test]
async fn tile_rgb_switches_line_color_around_the_contrast_threshold() {
    // Relative luminance ~0.181 and ~0.188 respectively; with the default dark and light lines the
    // contrast with both is equal at ~0.186.
    assert_eq!(line_color((118, 118, 118)), "white");
    assert_eq!(line_color((120, 120, 120)), "dark");
}

#[pagetop::test]
async fn tile_keeps_a_white_line_on_its_default_dark_background() {
    let svg = PageTopSvg::Tile.markup().into_string();

    assert!(svg.contains("<path fill=\"rgb(255,255,255)\""));
}

#[pagetop::test]
async fn tile_keeps_a_square_bottom_right_corner() {
    for svg in [
        PageTopSvg::Tile.markup().into_string(),
        PageTopSvg::TileRGB(0, 82, 204).markup().into_string(),
    ] {
        // Rounded background plus a square of the same color covering the bottom-right corner.
        assert!(svg.contains("rx=\"200\""));
        assert!(svg.contains("<rect x=\"1414\" y=\"1414\" width=\"200\" height=\"200\""));
    }
}
