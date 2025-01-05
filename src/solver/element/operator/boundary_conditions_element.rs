use super::Element;

impl Element {
    pub fn boundary_conditions(&self, l:&str,ix:usize,iv:usize) -> f64 {

        if l == "lx" {
            if self.lx_boundary == 0 {
                return 0.
            }

            if self.lx_boundary == 1 {
                return self.element_grid[ix+1][iv]
            }
        }
        else if l == "ux" {
            if self.ux_boundary == 0 {
                return 0.
            }
            if self.ux_boundary == 1 {
                return 0.
            }
        }
        0.
    }
}