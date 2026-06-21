use super::*;

// Metadata Drawer

/// Draws the metadata drawer on the right side.
/// Matches `.metadata-drawer` CSS: 320px wide, right edge, scrollable.
pub fn paint_metadata_drawer(
    state: &mut ViewState,
    text_cache: &mut TextCache,
    size: Size,
    scene: &mut Scene,
) {
    let mut painter = Painter::new(scene);

    let drawer_w = 320.0_f64.min(size.width);
    let x = size.width - drawer_w;

    // Drawer background
    let drawer_rect = Rect::new(x, 0.0, size.width, size.height);
    painter.fill_rect(drawer_rect, PANEL_BG);

    // Left border
    painter.draw_line(
        Point::new(x, 0.0),
        Point::new(x, size.height),
        BORDER_COLOR,
        1.0,
    );

    // Clip to drawer bounds
    painter.push_clip(drawer_rect);

    let pad = 18.0;
    let mut y = 20.0;

    // Header
    painter.draw_text_cached(
        text_cache,
        "Metadata",
        x + pad,
        y + 8.0,
        TEXT_PRIMARY,
        16.0,
        FontWeight::BOLD,
        false,
        false,
    );

    // Close button
    let close_x = size.width - pad - 40.0;
    let close_rect = Rect::new(close_x, y, close_x + 40.0, y + 28.0);
    painter.fill_rounded_rect(close_rect, BTN_BG, 4.0);
    painter.stroke_rounded_rect(close_rect, BORDER_COLOR, 1.0, 4.0);
    painter.draw_text_cached(
        text_cache,
        "Close",
        close_x + 4.0,
        y + 18.0,
        TEXT_SECONDARY,
        11.0,
        FontWeight::NORMAL,
        false,
        false,
    );
    state.register_click(close_rect, ClickAction::ToggleMetadata);

    y += 40.0;

    if let Some(ref doc) = state.document {
        // Histogram panel background
        let hist_rect = Rect::new(x + pad, y, size.width - pad - 8.0, y + 80.0);
        painter.fill_rounded_rect(hist_rect, INPUT_BG, 4.0);

        // Draw real histogram from HistogramData (R/G/B channels)
        if let Some(ref hist) = state.histogram {
            let hist_w = hist_rect.width() - 8.0;
            let hist_h = hist_rect.height() - 8.0;
            let max_val = hist.max_value();
            let bar_count = 64;
            let bar_w = hist_w / bar_count as f64;
            let bin_per_bar = 256 / bar_count;

            for i in 0..bar_count {
                let bin_start = i * bin_per_bar;
                let bin_end = (bin_start + bin_per_bar).min(256);

                // Sum bins for this bar
                let r_sum: u32 = hist.r[bin_start..bin_end].iter().sum();
                let g_sum: u32 = hist.g[bin_start..bin_end].iter().sum();
                let b_sum: u32 = hist.b[bin_start..bin_end].iter().sum();

                let bx = hist_rect.x0 + 4.0 + i as f64 * bar_w;

                // Draw R/G/B bars stacked from bottom
                let r_h = (r_sum as f64 / max_val as f64 * hist_h).min(hist_h);
                let g_h = (g_sum as f64 / max_val as f64 * hist_h).min(hist_h);
                let b_h = (b_sum as f64 / max_val as f64 * hist_h).min(hist_h);

                // Blue channel (bottom)
                if b_h > 0.0 {
                    painter.fill_rect(
                        Rect::new(
                            bx,
                            hist_rect.y1 - 4.0 - b_h,
                            bx + bar_w - 1.0,
                            hist_rect.y1 - 4.0,
                        ),
                        Color::rgba8(80, 80, 255, 100),
                    );
                }
                // Green channel (middle)
                if g_h > 0.0 {
                    painter.fill_rect(
                        Rect::new(
                            bx,
                            hist_rect.y1 - 4.0 - g_h,
                            bx + bar_w - 1.0,
                            hist_rect.y1 - 4.0,
                        ),
                        Color::rgba8(80, 220, 80, 100),
                    );
                }
                // Red channel (top)
                if r_h > 0.0 {
                    painter.fill_rect(
                        Rect::new(
                            bx,
                            hist_rect.y1 - 4.0 - r_h,
                            bx + bar_w - 1.0,
                            hist_rect.y1 - 4.0,
                        ),
                        Color::rgba8(255, 80, 80, 100),
                    );
                }
            }
        }

        y += 96.0;

        // Metadata fields - matches the dl/dt/dd grid layout
        let dim_str = doc.dimensions.map_or_else(
            || "Unknown".to_string(),
            |d| format!("{} x {}", d.width, d.height),
        );
        let format_upper = doc.format.to_uppercase();
        let size_str = bytes_label(doc.file_size);
        let fields: &[(&str, &str)] = &[
            ("Name", doc.file_name.as_str()),
            ("Format", &format_upper),
            ("Dimensions", &dim_str),
            ("Size", &size_str),
        ];

        for (label, value) in fields {
            painter.draw_text_cached(
                text_cache,
                label,
                x + pad,
                y + 8.0,
                TEXT_MUTED,
                11.0,
                FontWeight::NORMAL,
                false,
                false,
            );
            painter.draw_text_cached(
                text_cache,
                value,
                x + pad + 96.0,
                y + 8.0,
                TEXT_PRIMARY,
                12.0,
                FontWeight::MEDIUM,
                false,
                false,
            );
            y += 24.0;
        }

        // EXIF tags section
        y += 8.0;
        painter.draw_text_cached(
            text_cache,
            "EXIF Tags",
            x + pad,
            y + 8.0,
            TEXT_SECONDARY,
            12.0,
            FontWeight::BOLD,
            false,
            false,
        );
        y += 24.0;

        // Real EXIF tags from tench_image_core
        if state.exif_tags.is_empty() {
            painter.draw_text_cached(
                text_cache,
                "No EXIF data available",
                x + pad,
                y + 8.0,
                TEXT_MUTED,
                11.0,
                FontWeight::NORMAL,
                false,
                false,
            );
        } else {
            for tag in &state.exif_tags {
                // Truncate long values to fit the drawer width
                let max_val_chars = 80;
                let display_value = if tag.value.chars().count() > max_val_chars {
                    let truncated: String = tag.value.chars().take(max_val_chars).collect();
                    format!("{}...", truncated)
                } else {
                    tag.value.clone()
                };

                let row_rect = Rect::new(x + pad, y, size.width - pad - 8.0, y + 28.0);
                painter.fill_rounded_rect(row_rect, Color::rgba8(0x1A, 0x1A, 0x1A, 40), 4.0);
                painter.stroke_rounded_rect(row_rect, BORDER_COLOR, 1.0, 4.0);

                painter.draw_text_cached(
                    text_cache,
                    &tag.name,
                    x + pad + 8.0,
                    y + 17.0,
                    TEXT_MUTED,
                    11.0,
                    FontWeight::NORMAL,
                    false,
                    false,
                );
                painter.draw_text_cached(
                    text_cache,
                    &display_value,
                    x + pad + 140.0,
                    y + 17.0,
                    TEXT_PRIMARY,
                    11.0,
                    FontWeight::MEDIUM,
                    false,
                    false,
                );
                y += 36.0;
            }
        }

        // --- Rating section ---
        y += 16.0;
        painter.draw_text_cached(
            text_cache,
            "Rating",
            x + pad,
            y + 8.0,
            TEXT_SECONDARY,
            12.0,
            FontWeight::BOLD,
            false,
            false,
        );
        y += 24.0;

        // Draw 5 star buttons
        let star_size = 20.0_f64;
        let star_gap = 6.0;
        for i in 1..=5u8 {
            let star_x = x + pad + (i - 1) as f64 * (star_size + star_gap);
            let star_rect = Rect::new(star_x, y, star_x + star_size, y + star_size);
            let filled = i <= state.image_rating;
            let star_color = if filled {
                Color::rgb8(0xFF, 0xC8, 0x00)
            } else {
                TEXT_MUTED
            };
            // Draw a simple star shape (filled rectangle as placeholder)
            painter.fill_rounded_rect(star_rect, star_color, 2.0);
            painter.draw_text_cached(
                text_cache,
                if filled { "\u{2605}" } else { "\u{2606}" },
                star_x + 2.0,
                y + 15.0,
                if filled {
                    Color::rgb8(0x00, 0x00, 0x00)
                } else {
                    TEXT_SECONDARY
                },
                14.0,
                FontWeight::NORMAL,
                false,
                false,
            );
            state.register_click(star_rect, ClickAction::SetRating(i));
        }

        y += 36.0;

        // --- Tags section ---
        painter.draw_text_cached(
            text_cache,
            "Tags",
            x + pad,
            y + 8.0,
            TEXT_SECONDARY,
            12.0,
            FontWeight::BOLD,
            false,
            false,
        );
        y += 24.0;

        // Quick-add tag buttons
        let quick_tags = [
            "Favorite",
            "Landscape",
            "Portrait",
            "Nature",
            "Urban",
            "Art",
        ];
        let mut tag_x = x + pad;
        for tag_name in &quick_tags {
            let is_active = state.image_tags.contains(&tag_name.to_string());
            let tw = text_cache.measure_text_width(tag_name, 10.0, FontWeight::MEDIUM);
            let btn_w = tw + 12.0;
            if tag_x + btn_w > size.width - pad - 8.0 {
                tag_x = x + pad;
                y += 26.0;
            }
            let tag_rect = Rect::new(tag_x, y, tag_x + btn_w, y + 20.0);
            let bg = if is_active { ACCENT_VIEW } else { BTN_BG };
            let text_col = if is_active {
                Color::rgb8(0x0F, 0x0F, 0x0F)
            } else {
                TEXT_SECONDARY
            };
            painter.fill_rounded_rect(tag_rect, bg, 10.0);
            painter.stroke_rounded_rect(tag_rect, BORDER_COLOR, 1.0, 10.0);
            painter.draw_text_cached(
                text_cache,
                tag_name,
                tag_x + 6.0,
                y + 13.0,
                text_col,
                10.0,
                FontWeight::MEDIUM,
                false,
                false,
            );
            state.register_click(tag_rect, ClickAction::ToggleTag(tag_name.to_string()));
            tag_x += btn_w + 6.0;
        }
    } else {
        painter.draw_text_cached(
            text_cache,
            "Open an image to view metadata",
            x + pad,
            y + 20.0,
            TEXT_MUTED,
            12.0,
            FontWeight::NORMAL,
            false,
            false,
        );
    }

    painter.pop_clip();
}
