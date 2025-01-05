
use std::f64::consts::PI;
use super::Element;


impl Element {
    pub fn initialize_grid(&mut self) {
        
        if self.init_cond == 0 {
            for i in 0..self.config.NX {
                for j in 0..self.config.NV {
                    self.element_grid[i][j] = 0.;
                }
            }
        }

        if self.init_cond == 1 {
            let mut v:f64; 
            for ix in 0..self.config.NX {
                for iv in 0..self.config.NV {
                    v = -self.lv + (iv as f64) * self.dv;
                    self.element_grid[ix][iv] = (self.mu/2./PI).sqrt() * (-0.5*self.mu*v*v).exp() ;
                }
            }
        }
        if self.init_cond == 2 {
            let mut v:f64; 
            for ix in 0..self.config.NX {
                for iv in 0..self.config.NV {
                    v = -self.lv + (iv as f64) * self.dv;
                    if 2*ix < self.config.NX {
                    self.element_grid[ix][iv] = (self.mu/2./PI).sqrt() * (-0.5*self.mu*v*v).exp() ;
                    }
                    else {
                        self.element_grid[ix][iv] = 0.
                    }
                }
            }
        }

    }
}
