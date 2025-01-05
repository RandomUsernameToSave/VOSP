use super::Element;

impl Element {
    pub fn density(&self) -> Vec<f64> {
        let mut density_vec = vec![0.;self.config.NX];

        for ix in 0..self.config.NX {
            for iv in 0..self.config.NV {
                density_vec[ix] += self.element_grid[ix][iv]* self.dv;
            }
        }

        density_vec
    }
}