use crate::parameters::{field_bc_left, field_bc_right,NX};

use super::Field;


impl Field {
    pub fn boundary_conditions(&mut self) {
        if field_bc_left==0 {
            self.field_values[0] = 0. ;
        }
        if field_bc_left==1 {
            self.field_values[0] = self.field_values[2] ;
        } 
        if field_bc_right == 0 {
            self.field_values[NX-1] = 0.  ;
        }
        if field_bc_right == 1 {
            self.field_values[NX-1] = self.field_values[NX-2]  ;
        }
    }
}
