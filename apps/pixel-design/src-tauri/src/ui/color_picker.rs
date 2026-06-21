use super::*;

pub(super) fn paint_color_picker_modal(
    state: &PixelDesignState,
    p: &mut Painter<'_>,
    theme: &Theme,
    size: Size,
) {
    let overlay = Rect::new(0.0, 0.0, size.width, size.height);
    p.fill_rect(overlay, Color::rgba8(0, 0, 0, 90));

    let modal = PixelDesignApp::color_picker_modal(size);
    p.fill_rounded_rect(modal, theme.surface, 8.0);
    p.stroke_rounded_rect(modal, theme.primary, 1.0, 8.0);
    p.draw_text(
        if state.color_picker_target_fg {
            "Foreground Color"
        } else {
            "Background Color"
        },
        modal.x0 + 28.0,
        modal.y0 + 34.0,
        theme.on_surface,
        theme.font_size,
        FontWeight::BOLD,
        false,
    );

    let preview = Rect::new(
        modal.x1 - 74.0,
        modal.y0 + 22.0,
        modal.x1 - 28.0,
        modal.y0 + 58.0,
    );
    p.fill_rounded_rect(preview, state.color_picker_preview, 4.0);
    p.stroke_rounded_rect(preview, theme.disabled, 1.0, 4.0);

    let hue = PixelDesignApp::color_picker_hue_rect(modal);
    for idx in 0..12 {
        let x0 = hue.x0 + idx as f64 * hue.width() / 12.0;
        let x1 = hue.x0 + (idx + 1) as f64 * hue.width() / 12.0;
        p.fill_rect(
            Rect::new(x0, hue.y0, x1, hue.y1),
            hsv_color(idx as f32 * 30.0, 1.0, 1.0),
        );
    }
    p.stroke_rounded_rect(hue, theme.disabled, 1.0, 3.0);
    let hue_x = hue.x0 + hue.width() * (state.color_hue as f64 / 360.0).clamp(0.0, 1.0);
    p.draw_line(
        Point::new(hue_x, hue.y0 - 3.0),
        Point::new(hue_x, hue.y1 + 3.0),
        theme.on_surface,
        2.0,
    );

    let sv = PixelDesignApp::color_picker_sv_rect(modal);
    p.fill_rounded_rect(sv, state.color_picker_preview, 4.0);
    p.stroke_rounded_rect(sv, theme.disabled, 1.0, 4.0);
    p.draw_text(
        "Saturation / Value",
        sv.x0,
        sv.y0 - 8.0,
        theme.secondary,
        theme.font_size_small,
        FontWeight::NORMAL,
        false,
    );

    let cancel = PixelDesignApp::color_picker_cancel_rect(modal);
    p.fill_rounded_rect(cancel, self::theme::BG_BUTTON, 4.0);
    p.draw_text(
        "Cancel",
        cancel.x0 + 34.0,
        cancel.y0 + 22.0,
        theme.on_surface,
        theme.font_size_small,
        FontWeight::MEDIUM,
        false,
    );

    let apply = PixelDesignApp::color_picker_apply_rect(modal);
    p.fill_rounded_rect(apply, theme.primary, 4.0);
    p.draw_text(
        "Apply",
        apply.x0 + 42.0,
        apply.y0 + 22.0,
        theme.on_primary,
        theme.font_size_small,
        FontWeight::MEDIUM,
        false,
    );
}

fn hsv_color(h: f32, s: f32, v: f32) -> Color {
    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;
    let (r, g, b) = if h < 60.0 {
        (c, x, 0.0)
    } else if h < 120.0 {
        (x, c, 0.0)
    } else if h < 180.0 {
        (0.0, c, x)
    } else if h < 240.0 {
        (0.0, x, c)
    } else if h < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };
    Color::rgb8(
        ((r + m) * 255.0) as u8,
        ((g + m) * 255.0) as u8,
        ((b + m) * 255.0) as u8,
    )
}
