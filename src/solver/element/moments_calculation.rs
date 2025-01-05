use super::Element;
use crate::parameters::{ NX,NV};

impl Element {
    pub fn density(&self) -> Vec<f64> {
        let mut density_vec = vec![0.;NX];

        for ix in 0..NX {
            for iv in 0..NV {
                density_vec[ix] += self.element_grid[ix][iv]* self.dv;
            }
        }

        density_vec
    }
}