use super::*;

impl PixelDesignState {
    pub fn cancel_modal_action(&mut self) {
        self.show_text_input = false;
        self.is_drawing = false;
        self.current_stroke = None;
        self.is_panning = false;
        self.pan_start = None;
        self.shape_drag_start = None;
        self.move_drag_start = None;
        self.move_layer_start_offset = None;
        self.show_color_picker = false;
        self.active_modal = ModalType::None;
        self.blend_mode_dropdown_open = false;
        self.layer_context_menu_idx = None;
        self.renaming_layer_idx = None;
        if let Some(job) = self
            .ai_jobs
            .iter_mut()
            .find(|job| matches!(job.status, JobStatus::Queued | JobStatus::Running))
        {
            job.status = JobStatus::Failed;
            self.ai_cancel_requested = true;
        }
        self.status_msg = "Cancelled".into();
    }

    pub fn run_ai_job(&mut self) {
        let id = format!("job-{}", self.ai_jobs.len() + 1);
        let label = if self.ai_prompt.trim().is_empty() {
            format!("{} request", self.expanded_ai.label())
        } else {
            self.ai_prompt.trim().chars().take(30).collect()
        };
        self.ai_jobs.insert(
            0,
            AiJob {
                id,
                tool: self.expanded_ai,
                label,
                status: JobStatus::Queued,
                progress: 0,
            },
        );
        self.ai_jobs[0].status = JobStatus::Running;
        self.ai_jobs[0].progress = 0;
        if self.ai_jobs.len() > 1 {
            self.ai_jobs[1].status = JobStatus::Done;
            self.ai_jobs[1].progress = 100;
        }
        self.ai_cancel_requested = false;
        self.status_msg = format!("Running {}", self.expanded_ai.label());
    }

    pub fn cancel_ai_job(&mut self) {
        if let Some(job) = self
            .ai_jobs
            .iter_mut()
            .find(|job| matches!(job.status, JobStatus::Queued | JobStatus::Running))
        {
            job.status = JobStatus::Failed;
            self.status_msg = "AI job cancelled".into();
        }
    }

    pub fn cycle_export_format(&mut self) {
        let formats = ["PNG", "JPEG", "WebP", "BMP"];
        let idx = formats
            .iter()
            .position(|format| *format == self.export_format)
            .unwrap_or(0);
        self.export_format = formats[(idx + 1) % formats.len()].into();
        self.status_msg = format!("Export format: {}", self.export_format);
    }
}
