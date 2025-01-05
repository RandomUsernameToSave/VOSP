use crate::parameters::{dxi, NX,DX};
use std::f64::consts::PI;
mod field_boundary_conditions;
use hdf5::{File, Group};

pub struct Field {
    name:String,
    pub field_values:Vec<f64>,
    group_h5:Group,
}

impl Field {
    
    pub fn new(name:String,file:File) -> Field {
        let field_values = vec![0.;NX];
        let group: Group = file.create_group(&name).unwrap();

        Field {name:name, field_values:field_values,group_h5:group}
    }

    pub fn save_h5(&self,file:File,t:f64) {
        let dataset = self.group_h5.new_dataset::<f64>().shape([NX]).create(t.to_string().as_str()).unwrap();
    
    
        dataset.write(&self.field_values);
    }

    pub fn gradient(&self, gradient_field:&mut Field ){
        for ix in 1..NX-1 {
            gradient_field.field_values[ix] = -(self.field_values[ix+1] - self.field_values[ix-1] )*dxi*0.5;
        }
    }

    fn tolerance_limit(forcing_vector:&Vec<f64>) -> f64 {
        let mut tol = 0. ;
        let mut R;
        for i in 1..NX-1 {
            R = 1./12. * (forcing_vector[i+1] - 2.*forcing_vector[i] + forcing_vector[i-1]).abs();
            if R>tol {
                tol = R;
            }
        }
        tol
    }

    fn has_converged(&self,forcing_vector:&Vec<f64>) -> bool {
        // let converged = true;

        let tol = Field::tolerance_limit(&forcing_vector);
        let mut R :f64;
        for i in 1..NX-1 {
            R  = (self.field_values[i+1] - 2.* self.field_values[i] + self.field_values[i-1])*dxi*dxi - forcing_vector[i];
            R = R.abs();
            if R>tol {
                return false;
            }
        }
        true


  
    }


    pub fn solve_potential(&mut self,forcing_vector:Vec<f64>) {
        // NOT WORKING YET

        //let rho = ( (PI/NX as f64).cos() +DX*DX/DY/DY) * (PI/NY as f64).cos() / (1. + DX*DX/DY/DY);
        //let w = 2./ (1. + (1.-rho.powi(2)).sqrt());
        let w = 2./ (1. + (1.-(PI*DX).sin()));
        let a = 2./(DX*DX) ;
        let mut converged:bool = self.has_converged(&forcing_vector);

        while !converged {
            self.boundary_conditions();
            for i in 1..(NX-1) {
                self.field_values[i] = (1.-w)*self.field_values[i] + w/a * (-forcing_vector[i] + (self.field_values[i-1] + self.field_values[i+1])*dxi*dxi) ;
                
            }
            converged = self.has_converged(&forcing_vector);
        }
    }
}
