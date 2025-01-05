
use super::Field;


impl Field {
    pub fn boundary_conditions(&mut self) {
        if self.config.field_bc_left==0 {
            self.field_values[0] = 0. ;
        }
        if self.config.field_bc_left==1 {
            self.field_values[0] = self.field_values[2] ;
        } 
        if self.config.field_bc_right == 0 {
            self.field_values[self.config.NX-1] = 0.  ;
        }
        if self.config.field_bc_right == 1 {
            self.field_values[self.config.NX-1] = self.field_values[self.config.NX-2]  ;
        }
    }
}
