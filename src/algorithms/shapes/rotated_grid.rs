use glam::{Mat2, Vec2};

/// An iterator over grid points which, after rotation by a given angle,
/// lie within the final (world–space) bounds. The grid is hexagonal:
/// every other row is offset by half a spacing, and the vertical spacing
/// is adjusted so that the nearest neighbor distance is equal to `spacing`.
pub struct RotatedGrid {
    // Final (world–space) bounds.
    final_x_min: f32,
    final_x_max: f32,
    final_y_min: f32,
    final_y_max: f32,
    // The desired nearest neighbor distance.
    spacing: f32,
    // In a hexagonal grid the vertical spacing is reduced.
    vertical_spacing: f32,
    // Rotation matrix (local -> final).
    rotation: Mat2,
    // The covering local bounding box computed by inverse rotating the final bounds.
    u_min: f32,
    _u_max: f32,
    v_min: f32,
    _v_max: f32,
    // Number of rows (in local space) required to cover the vertical span.
    num_rows: usize,
    // For even rows (not offset) the number of candidate columns.
    num_even: usize,
    // For odd rows (offset by half spacing) the number of candidate columns.
    num_odd: usize,
    // Current row and column indices for iteration.
    cur_row: usize,
    cur_col: usize,
}

impl RotatedGrid {
    /// Create a new rotated hexagonal grid.
    ///
    /// * `final_x_bounds` - (min_x, max_x) for the final (rotated) coordinates.
    /// * `final_y_bounds` - (min_y, max_y) for the final (rotated) coordinates.
    /// * `spacing` - the distance between nearest neighbors.
    /// * `angle` - the rotation angle (in radians) to apply.
    ///
    /// **Note:** In a hexagonal grid, every other row is offset by half a spacing,
    /// and the vertical spacing is `spacing * sqrt(3)/2` so that each point’s closest
    /// neighbors lie exactly at distance `spacing`.
    pub fn new(
        final_x_bounds: (f32, f32),
        final_y_bounds: (f32, f32),
        spacing: f32,
        angle: f32,
    ) -> Self {
        let (final_x_min, final_x_max) = final_x_bounds;
        let (final_y_min, final_y_max) = final_y_bounds;

        // Create the rotation matrix (from local to final space) and its inverse.
        let rotation = Mat2::from_angle(angle);
        let inv_rotation = Mat2::from_angle(-angle);

        // The four corners of the final bounds.
        let corners = [
            Vec2::new(final_x_min, final_y_min),
            Vec2::new(final_x_min, final_y_max),
            Vec2::new(final_x_max, final_y_min),
            Vec2::new(final_x_max, final_y_max),
        ];

        // Map each final-space corner back into local space.
        let local_corners: Vec<Vec2> = corners.iter().map(|&p| inv_rotation * p).collect();

        // Determine the axis–aligned bounding box in local space.
        let u_min = local_corners.iter().map(|p| p.x).fold(f32::INFINITY, f32::min);
        let u_max = local_corners.iter().map(|p| p.x).fold(f32::NEG_INFINITY, f32::max);
        let v_min = local_corners.iter().map(|p| p.y).fold(f32::INFINITY, f32::min);
        let v_max = local_corners.iter().map(|p| p.y).fold(f32::NEG_INFINITY, f32::max);

        // For a hexagonal grid, the vertical spacing is reduced.
        let vertical_spacing = spacing * (3.0f32).sqrt() / 2.0;

        // Compute how many rows we need to cover the vertical span.
        let num_rows = ((v_max - v_min) / vertical_spacing).floor() as usize + 1;
        // For even rows, candidate u values are u = u_min + i * spacing.
        let num_even = ((u_max - u_min) / spacing).floor() as usize + 1;
        // For odd rows, candidate u values are u = u_min + spacing/2 + i * spacing.
        let num_odd = if u_max > (u_min + spacing / 2.0) {
            (((u_max - (u_min + spacing / 2.0)) / spacing).floor() as usize) + 1
        } else {
            0
        };

        RotatedGrid {
            final_x_min,
            final_x_max,
            final_y_min,
            final_y_max,
            spacing,
            vertical_spacing,
            rotation,
            u_min,
            _u_max: u_max,
            v_min,
            _v_max: v_max,
            num_rows,
            num_even,
            num_odd,
            cur_row: 0,
            cur_col: 0,
        }
    }

    /// Check if a final-space point is within the final bounds.
    fn in_final_bounds(&self, p: Vec2) -> bool {
        p.x >= self.final_x_min
            && p.x <= self.final_x_max
            && p.y >= self.final_y_min
            && p.y <= self.final_y_max
    }
}

impl Iterator for RotatedGrid {
    type Item = Vec2;

    fn next(&mut self) -> Option<Self::Item> {
        // Iterate row by row in the local coordinate system.
        while self.cur_row < self.num_rows {
            // Choose the number of candidate columns based on whether the row is even or odd.
            let max_cols = if self.cur_row % 2 == 0 {
                self.num_even
            } else {
                self.num_odd
            };

            if self.cur_col < max_cols {
                // For even rows, no offset; for odd rows, add half a spacing.
                let u = if self.cur_row % 2 == 0 {
                    self.u_min + self.cur_col as f32 * self.spacing
                } else {
                    self.u_min + self.spacing / 2.0 + self.cur_col as f32 * self.spacing
                };
                // The vertical coordinate uses the hexagonal vertical spacing.
                let v = self.v_min + self.cur_row as f32 * self.vertical_spacing;
                self.cur_col += 1;

                // Rotate the local candidate point into final space.
                let candidate = Vec2::new(u, v);
                let final_point = self.rotation * candidate;
                // Only yield the point if it lies within the final bounds.
                if self.in_final_bounds(final_point) {
                    return Some(final_point);
                }
                // Otherwise, continue to the next candidate.
                continue;
            } else {
                // Finished the current row; move on to the next.
                self.cur_row += 1;
                self.cur_col = 0;
            }
        }
        None
    }
}

