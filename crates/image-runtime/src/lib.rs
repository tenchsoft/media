use serde::{Deserialize, Serialize};

pub mod pixel;
pub mod view;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ImageRuntimeCapabilities {
    pub product_id: &'static str,
    pub can_open_archives: bool,
    pub can_edit_pixels: bool,
    pub can_use_ai_tools: bool,
}

pub const VIEW_RUNTIME: ImageRuntimeCapabilities = ImageRuntimeCapabilities {
    product_id: "tench-view",
    can_open_archives: true,
    can_edit_pixels: true,
    can_use_ai_tools: true,
};

pub const PIXEL_DESIGN_RUNTIME: ImageRuntimeCapabilities = ImageRuntimeCapabilities {
    product_id: "tench-pixel-design",
    can_open_archives: false,
    can_edit_pixels: true,
    can_use_ai_tools: true,
};

pub fn image_runtime_products() -> [ImageRuntimeCapabilities; 2] {
    [VIEW_RUNTIME, PIXEL_DESIGN_RUNTIME]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn separates_view_and_pixel_design_capabilities() {
        const { assert!(VIEW_RUNTIME.can_open_archives) };
        const { assert!(!PIXEL_DESIGN_RUNTIME.can_open_archives) };
        const { assert!(PIXEL_DESIGN_RUNTIME.can_edit_pixels) };
    }
}
