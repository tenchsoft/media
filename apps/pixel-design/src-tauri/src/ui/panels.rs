use super::state::{AiTool, JobStatus, PixelDesignState};
use super::theme;
use tench_ui::parley::FontWeight;
use tench_ui::prelude::*;

pub fn paint_ai_panel(state: &PixelDesignState, p: &mut Painter<'_>, theme: &Theme, panel: Rect) {
    paint_panel_shell(p, panel);
    let x = panel.x0 + 18.0;
    let mut y = panel.y0 + 34.0;
    p.draw_text(
        "AI",
        x,
        y,
        theme.on_surface,
        theme.font_size,
        FontWeight::BOLD,
        false,
    );
    y += 28.0;

    // Model selection (Phase 6)
    p.draw_text(
        "Model:",
        x,
        y,
        theme.secondary,
        theme.font_size_small,
        FontWeight::NORMAL,
        false,
    );
    let model_rect = Rect::new(x + 50.0, y - 10.0, panel.x1 - 18.0, y + 6.0);
    p.fill_rounded_rect(model_rect, theme::BG_ROW_INACTIVE, 3.0);
    p.draw_text(
        &state.ai_model,
        x + 56.0,
        y,
        theme.on_surface,
        theme.font_size_small,
        FontWeight::NORMAL,
        false,
    );
    y += 16.0;

    p.draw_text(
        state.expanded_ai.label(),
        x,
        y,
        theme::ACCENT_GREEN,
        theme.font_size_small,
        FontWeight::BOLD,
        false,
    );
    y += 16.0;
    let prompt = Rect::new(x, y, panel.x1 - 18.0, y + 54.0);
    p.fill_rounded_rect(prompt, theme::BG_ROW_INACTIVE, 5.0);
    p.draw_text(
        &state.ai_prompt,
        prompt.x0 + 10.0,
        prompt.y0 + 25.0,
        theme.secondary,
        theme.font_size_small,
        FontWeight::NORMAL,
        false,
    );
    let run = Rect::new(x, y + 66.0, panel.x1 - 18.0, y + 102.0);
    p.fill_rounded_rect(run, theme::ACCENT, 5.0);
    p.draw_text(
        "Run AI Job",
        run.x0 + 82.0,
        run.y0 + 23.0,
        theme::TEXT_ON_ACCENT,
        theme.font_size_small,
        FontWeight::BOLD,
        false,
    );

    // Phase 6: Cancel button
    let cancel = Rect::new(x, y + 108.0, panel.x1 - 18.0, y + 136.0);
    p.fill_rounded_rect(cancel, theme::BG_ROW_INACTIVE, 4.0);
    p.draw_text(
        "Cancel",
        cancel.x0 + 100.0,
        cancel.y0 + 16.0,
        theme.on_surface,
        theme.font_size_small,
        FontWeight::NORMAL,
        false,
    );

    y += 150.0;
    p.draw_text(
        "Tools",
        x,
        y,
        theme.on_surface,
        theme.font_size,
        FontWeight::BOLD,
        false,
    );
    y += 18.0;
    for tool in AiTool::ALL {
        let active = state.expanded_ai == tool;
        p.fill_rounded_rect(
            Rect::new(x, y, panel.x1 - 18.0, y + 28.0),
            if active {
                theme::BG_ROW_ACTIVE
            } else {
                theme::BG_ROW_INACTIVE
            },
            4.0,
        );
        p.draw_text(
            tool.glyph(),
            x + 10.0,
            y + 19.0,
            theme.secondary,
            theme.font_size_small,
            FontWeight::BOLD,
            false,
        );
        p.draw_text(
            tool.label(),
            x + 42.0,
            y + 19.0,
            theme.on_surface,
            theme.font_size_small,
            FontWeight::MEDIUM,
            false,
        );
        y += 34.0;
    }

    y += 12.0;
    p.draw_text(
        "Queue",
        x,
        y,
        theme.on_surface,
        theme.font_size,
        FontWeight::BOLD,
        false,
    );
    y += 20.0;
    for job in state.ai_jobs.iter().take(5) {
        let status = job.status.label(job.progress);
        p.fill_rounded_rect(
            Rect::new(x, y, panel.x1 - 18.0, y + 34.0),
            theme::BG_ROW_INACTIVE,
            4.0,
        );
        p.draw_text(
            &job.id,
            x + 8.0,
            y + 14.0,
            theme.disabled,
            9.0,
            FontWeight::NORMAL,
            false,
        );
        p.draw_text(
            job.tool.glyph(),
            x + 10.0,
            y + 25.0,
            theme.secondary,
            theme.font_size_small,
            FontWeight::BOLD,
            false,
        );
        p.draw_text(
            &job.label,
            x + 42.0,
            y + 21.0,
            theme.on_surface,
            theme.font_size_small,
            FontWeight::NORMAL,
            false,
        );
        p.draw_text(
            &status,
            panel.x1 - 74.0,
            y + 21.0,
            job_status_color(job.status),
            theme.font_size_small,
            FontWeight::MEDIUM,
            false,
        );
        y += 40.0;
    }
}

pub fn paint_adjust_panel(
    state: &PixelDesignState,
    p: &mut Painter<'_>,
    theme: &Theme,
    panel: Rect,
) {
    paint_panel_shell(p, panel);
    let x = panel.x0 + 18.0;
    let mut y = panel.y0 + 34.0;
    p.draw_text(
        "Adjustments",
        x,
        y,
        theme.on_surface,
        theme.font_size,
        FontWeight::BOLD,
        false,
    );
    y += 26.0;

    // Phase 4: Preset buttons mapped to real filters
    for (idx, preset) in PixelDesignState::adjust_presets().iter().enumerate() {
        let col = idx % 2;
        let row = idx / 2;
        let rx = x + col as f64 * 124.0;
        let ry = y + row as f64 * 38.0;
        let active = state.active_adjust.as_deref() == Some(*preset);
        p.fill_rounded_rect(
            Rect::new(rx, ry, rx + 114.0, ry + 30.0),
            if active {
                theme::ACCENT
            } else {
                theme::BG_ROW_INACTIVE
            },
            4.0,
        );
        p.draw_text(
            preset,
            rx + 12.0,
            ry + 20.0,
            if active {
                theme::TEXT_ON_ACCENT
            } else {
                theme.on_surface
            },
            theme.font_size_small,
            FontWeight::MEDIUM,
            false,
        );
    }

    y += 176.0;
    p.draw_text(
        "Properties",
        x,
        y,
        theme.on_surface,
        theme.font_size,
        FontWeight::BOLD,
        false,
    );
    y += 24.0;
    // Phase 4: 8 adjustment rows (Brightness, Contrast, Saturation, Temperature, Hue, Sharpness, Blur, Levels)
    for (label, value) in state.adjust_values.rows() {
        paint_signed_bar(
            p,
            theme,
            Rect::new(x, y, panel.x1 - 20.0, y + 30.0),
            label,
            value,
        );
        y += 40.0;
    }
}

pub fn paint_export_panel(
    state: &PixelDesignState,
    p: &mut Painter<'_>,
    theme: &Theme,
    panel: Rect,
) {
    paint_panel_shell(p, panel);
    let x = panel.x0 + 18.0;
    let mut y = panel.y0 + 34.0;
    p.draw_text(
        "Export",
        x,
        y,
        theme.on_surface,
        theme.font_size,
        FontWeight::BOLD,
        false,
    );
    y += 24.0;
    paint_option(
        p,
        theme,
        Rect::new(x, y, panel.x1 - 18.0, y + 34.0),
        "Format",
        &state.export_format,
    );
    y += 58.0;
    paint_option(
        p,
        theme,
        Rect::new(x, y, panel.x1 - 18.0, y + 34.0),
        "Quality",
        &format!("{}%", state.export_quality),
    );
    y += 58.0;
    paint_option(
        p,
        theme,
        Rect::new(x, y, panel.x1 - 18.0, y + 34.0),
        "Scale",
        &format!("{}%", state.export_scale),
    );
    y += 62.0;
    let out_w = state.document.width * state.export_scale / 100;
    let out_h = state.document.height * state.export_scale / 100;
    paint_option(
        p,
        theme,
        Rect::new(x, y, panel.x1 - 18.0, y + 34.0),
        "Output",
        &format!("{} x {}", out_w, out_h),
    );
    y += 74.0;
    p.fill_rounded_rect(
        Rect::new(x, y, panel.x1 - 18.0, y + 38.0),
        theme::ACCENT_GREEN,
        5.0,
    );
    p.draw_text(
        &format!("Export {}", state.export_format),
        x + 80.0,
        y + 24.0,
        theme::TEXT_ON_GREEN,
        theme.font_size_small,
        FontWeight::BOLD,
        false,
    );
}

fn paint_panel_shell(p: &mut Painter<'_>, panel: Rect) {
    p.fill_rect(panel, theme::BG_SURFACE);
    p.draw_line(
        Point::new(panel.x0, panel.y0),
        Point::new(panel.x0, panel.y1),
        theme::BORDER,
        1.0,
    );
}

fn paint_signed_bar(p: &mut Painter<'_>, theme: &Theme, rect: Rect, label: &str, value: i32) {
    p.draw_text(
        label,
        rect.x0,
        rect.y0 + 20.0,
        theme.secondary,
        theme.font_size_small,
        FontWeight::NORMAL,
        false,
    );
    let bar = Rect::new(
        rect.x0 + 92.0,
        rect.y0 + 10.0,
        rect.x1 - 48.0,
        rect.y0 + 18.0,
    );
    p.fill_rounded_rect(bar, theme::BG_BAR, 4.0);
    let mid = (bar.x0 + bar.x1) * 0.5;
    let fill = (value.unsigned_abs() as f64 / 100.0) * (bar.width() * 0.5);
    let fill_rect = if value >= 0 {
        Rect::new(mid, bar.y0, mid + fill, bar.y1)
    } else {
        Rect::new(mid - fill, bar.y0, mid, bar.y1)
    };
    p.fill_rounded_rect(fill_rect, theme::ACCENT_YELLOW, 4.0);
    p.draw_text(
        &format!("{value:+}"),
        rect.x1 - 40.0,
        rect.y0 + 20.0,
        theme.on_surface,
        theme.font_size_small,
        FontWeight::MEDIUM,
        false,
    );
}

fn paint_option(p: &mut Painter<'_>, theme: &Theme, rect: Rect, label: &str, value: &str) {
    p.draw_text(
        label,
        rect.x0,
        rect.y0 - 6.0,
        theme.secondary,
        theme.font_size_small,
        FontWeight::NORMAL,
        false,
    );
    p.fill_rounded_rect(rect, theme::BG_ROW_INACTIVE, 4.0);
    p.draw_text(
        value,
        rect.x0 + 12.0,
        rect.y0 + 22.0,
        theme.on_surface,
        theme.font_size_small,
        FontWeight::MEDIUM,
        false,
    );
}

fn job_status_color(status: JobStatus) -> Color {
    match status {
        JobStatus::Queued => theme::TEXT_DISABLED,
        JobStatus::Running => theme::ACCENT_YELLOW,
        JobStatus::Done => theme::ACCENT_GREEN,
        JobStatus::Failed => theme::ACCENT_RED,
    }
}
