use tench_ui::parley::FontWeight;
use tench_ui::prelude::*;

use crate::ui::ai_panel;
use crate::ui::state::*;

/// Paint the AI assistant panel.
pub fn paint_ai_panel(
    p: &mut Painter<'_>,
    state: &mut PlayerState,
    theme: &tench_ui::prelude::Theme,
    video_rect: &Rect,
    size: Size,
    spacing: f64,
    spacing_large: f64,
) {
    if !state.ai_panel_open {
        return;
    }

    let ai_rect = ai_panel::panel_rect(video_rect.x1, size.width, size.height);
    p.fill_rect(ai_rect, theme.surface);

    let mut ay = spacing_large;
    p.draw_text(
        "AI Assistant",
        video_rect.x1 + spacing,
        ay,
        theme.on_surface,
        theme.font_size_large,
        FontWeight::BOLD,
        false,
    );
    ay += 28.0;

    p.draw_text(
        "Ask about this video, generate",
        video_rect.x1 + spacing,
        ay,
        theme.secondary,
        theme.font_size,
        FontWeight::NORMAL,
        false,
    );
    ay += 18.0;
    p.draw_text(
        "summaries, or find scenes.",
        video_rect.x1 + spacing,
        ay,
        theme.secondary,
        theme.font_size,
        FontWeight::NORMAL,
        false,
    );
    ay += 32.0;

    // Input field
    let input_rect = Rect::new(
        video_rect.x1 + spacing,
        ay,
        size.width - spacing,
        ay + theme.input_height,
    );
    p.fill_rounded_rect(input_rect, theme.background, theme.border_radius);
    p.stroke_rounded_rect(input_rect, theme.border, 1.0, theme.border_radius);
    if state.ai_input_text.is_empty() {
        p.draw_text(
            "Type a question...",
            video_rect.x1 + spacing + 8.0,
            ay + 18.0,
            theme.disabled,
            theme.font_size,
            FontWeight::NORMAL,
            false,
        );
    } else {
        p.draw_text(
            &state.ai_input_text,
            video_rect.x1 + spacing + 8.0,
            ay + 18.0,
            theme.on_surface,
            theme.font_size,
            FontWeight::NORMAL,
            false,
        );
    }
    state.register_click(input_rect, ClickAction::FocusAiInput);

    ay += 54.0;
    // Feature prompt buttons
    for feature in ai_panel::feature_prompts().iter() {
        let row = Rect::new(video_rect.x1 + spacing, ay, size.width - spacing, ay + 30.0);
        p.fill_rounded_rect(row, theme.background, theme.border_radius);
        p.draw_text(
            feature,
            row.x0 + 8.0,
            ay + 19.0,
            theme.on_surface,
            theme.font_size_small,
            FontWeight::MEDIUM,
            false,
        );
        state.register_click(row, ClickAction::SendAiPrompt(feature.to_string()));
        ay += 38.0;
    }

    // Cancel button for in-progress requests
    if state.ai_request_pending {
        let cancel_rect = Rect::new(video_rect.x1 + spacing, ay, size.width - spacing, ay + 28.0);
        p.fill_rounded_rect(
            cancel_rect,
            Color::rgba8(200, 50, 50, 200),
            theme.border_radius,
        );
        p.draw_text(
            "Cancel",
            cancel_rect.x0 + cancel_rect.width() / 2.0,
            ay + 18.0,
            Color::WHITE,
            theme.font_size_small,
            FontWeight::BOLD,
            true,
        );
        state.register_click(cancel_rect, ClickAction::CancelAiRequest);
        ay += 36.0;
    }

    // Chat log display area
    for msg in &state.ai_chat_log {
        let msg_color = match msg.role {
            AiMessageRole::User => theme.primary,
            AiMessageRole::Assistant => theme.on_surface,
            AiMessageRole::System => theme.secondary,
        };
        let prefix = match msg.role {
            AiMessageRole::User => "You: ",
            AiMessageRole::Assistant => "AI: ",
            AiMessageRole::System => "System: ",
        };
        let display_text = format!("{}{}", prefix, msg.text);
        let max_chars = 40;
        let chunks: Vec<&str> = display_text
            .as_bytes()
            .chunks(max_chars)
            .map(|c| std::str::from_utf8(c).unwrap_or(""))
            .collect();
        for chunk in &chunks {
            p.draw_text(
                chunk,
                video_rect.x1 + spacing,
                ay,
                msg_color,
                theme.font_size_small,
                FontWeight::NORMAL,
                false,
            );
            ay += 16.0;
        }
        ay += 8.0;
    }
}
