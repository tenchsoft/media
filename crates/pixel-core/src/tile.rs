//! Tile-based rendering for large images.

/// Tile size in pixels (64x64 tiles).
pub const TILE_SIZE: u32 = 64;

/// A single tile in the tile grid.
#[derive(Clone, Debug)]
pub struct Tile {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
    pub dirty: bool,
}

impl Tile {
    /// Creates a new transparent tile.
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
            data: vec![0u8; (width * height * 4) as usize],
            dirty: true,
        }
    }

    /// Returns the pixel at (lx, ly) relative to the tile origin.
    pub fn pixel(&self, lx: u32, ly: u32) -> (u8, u8, u8, u8) {
        if lx >= self.width || ly >= self.height {
            return (0, 0, 0, 0);
        }
        let offset = (ly * self.width + lx) as usize * 4;
        (
            self.data[offset],
            self.data[offset + 1],
            self.data[offset + 2],
            self.data[offset + 3],
        )
    }

    /// Sets the pixel at (lx, ly) relative to the tile origin.
    pub fn set_pixel(&mut self, lx: u32, ly: u32, r: u8, g: u8, b: u8, a: u8) {
        if lx >= self.width || ly >= self.height {
            return;
        }
        let offset = (ly * self.width + lx) as usize * 4;
        self.data[offset] = r;
        self.data[offset + 1] = g;
        self.data[offset + 2] = b;
        self.data[offset + 3] = a;
        self.dirty = true;
    }

    /// Marks the tile as clean (rendered).
    pub fn mark_clean(&mut self) {
        self.dirty = false;
    }
}

/// A grid of tiles covering an image.
#[derive(Clone, Debug)]
pub struct TileGrid {
    pub image_width: u32,
    pub image_height: u32,
    pub cols: u32,
    pub rows: u32,
    pub tiles: Vec<Tile>,
}

impl TileGrid {
    /// Creates a tile grid for the given image dimensions.
    pub fn new(image_width: u32, image_height: u32) -> Self {
        let cols = image_width.div_ceil(TILE_SIZE);
        let rows = image_height.div_ceil(TILE_SIZE);
        let mut tiles = Vec::new();

        for row in 0..rows {
            for col in 0..cols {
                let x = col * TILE_SIZE;
                let y = row * TILE_SIZE;
                let w = TILE_SIZE.min(image_width - x);
                let h = TILE_SIZE.min(image_height - y);
                tiles.push(Tile::new(x, y, w, h));
            }
        }

        Self {
            image_width,
            image_height,
            cols,
            rows,
            tiles,
        }
    }

    /// Returns the tile index for the given pixel coordinates.
    pub fn tile_index_for_pixel(&self, x: u32, y: u32) -> Option<usize> {
        if x >= self.image_width || y >= self.image_height {
            return None;
        }
        let col = x / TILE_SIZE;
        let row = y / TILE_SIZE;
        Some((row * self.cols + col) as usize)
    }

    /// Returns the tile at the given grid position.
    pub fn tile_at(&self, col: u32, row: u32) -> Option<&Tile> {
        if col >= self.cols || row >= self.rows {
            return None;
        }
        self.tiles.get((row * self.cols + col) as usize)
    }

    /// Returns a mutable reference to the tile at the given grid position.
    pub fn tile_at_mut(&mut self, col: u32, row: u32) -> Option<&mut Tile> {
        if col >= self.cols || row >= self.rows {
            return None;
        }
        self.tiles.get_mut((row * self.cols + col) as usize)
    }

    /// Sets a pixel at absolute image coordinates.
    pub fn set_pixel(&mut self, x: u32, y: u32, r: u8, g: u8, b: u8, a: u8) {
        if let Some(idx) = self.tile_index_for_pixel(x, y) {
            let tile = &mut self.tiles[idx];
            let lx = x - tile.x;
            let ly = y - tile.y;
            tile.set_pixel(lx, ly, r, g, b, a);
        }
    }

    /// Returns dirty tiles that need re-rendering.
    pub fn dirty_tiles(&self) -> Vec<&Tile> {
        self.tiles.iter().filter(|t| t.dirty).collect()
    }

    /// Marks all tiles as dirty.
    pub fn mark_all_dirty(&mut self) {
        for tile in &mut self.tiles {
            tile.dirty = true;
        }
    }

    /// Marks all tiles as clean.
    pub fn mark_all_clean(&mut self) {
        for tile in &mut self.tiles {
            tile.dirty = false;
        }
    }

    /// Returns the number of dirty tiles.
    pub fn dirty_count(&self) -> usize {
        self.tiles.iter().filter(|t| t.dirty).count()
    }
}

/// A rectangular region that needs to be re-rendered.
#[derive(Clone, Debug, Default)]
pub struct DirtyRegion {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl DirtyRegion {
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// Expands the region to include the given point.
    pub fn include_point(&mut self, px: u32, py: u32) {
        let x2 = self.x + self.width;
        let y2 = self.y + self.height;
        self.x = self.x.min(px);
        self.y = self.y.min(py);
        self.width = (x2.max(px + 1) - self.x).max(1);
        self.height = (y2.max(py + 1) - self.y).max(1);
    }

    /// Returns true if the region is empty (zero area).
    pub fn is_empty(&self) -> bool {
        self.width == 0 || self.height == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tile_grid_dimensions() {
        let grid = TileGrid::new(200, 150);
        assert_eq!(grid.cols, 4); // ceil(200/64) = 4
        assert_eq!(grid.rows, 3); // ceil(150/64) = 3
        assert_eq!(grid.tiles.len(), 12);
    }

    #[test]
    fn set_and_get_pixel() {
        let mut grid = TileGrid::new(128, 128);
        grid.set_pixel(10, 20, 255, 128, 0, 255);
        let idx = grid.tile_index_for_pixel(10, 20).unwrap();
        let tile = &grid.tiles[idx];
        let (r, g, _b, _a) = tile.pixel(10, 20);
        assert_eq!(r, 255);
        assert_eq!(g, 128);
    }

    #[test]
    fn dirty_tracking() {
        let mut grid = TileGrid::new(128, 128);
        assert_eq!(grid.dirty_count(), 4); // 2x2 tiles, all start dirty
        grid.mark_all_clean();
        assert_eq!(grid.dirty_count(), 0);
        grid.set_pixel(10, 10, 255, 0, 0, 255);
        assert_eq!(grid.dirty_count(), 1);
    }

    #[test]
    fn dirty_region_include_point() {
        let mut region = DirtyRegion::new(10, 10, 5, 5);
        region.include_point(20, 30);
        assert_eq!(region.x, 10);
        assert_eq!(region.width, 11); // 21 - 10
        assert_eq!(region.height, 21); // 31 - 10
    }
}
