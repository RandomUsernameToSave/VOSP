pub mod element;
mod fields;
use element::Element;
use crate::parameters::{lambda, DT, NX, T,n_save};
use hdf5::File;
use crate::config;
pub struct Solver {
    elements:Vec<Element>,
    electric_field:fields::Field,
    potentiel_field:fields::Field,
    file_h5 : File,
}

impl Solver {
    pub fn new()-> Solver { 
        let elements    = vec![];

        let file = File::create("VOSP.h5").unwrap();
        let electric_field  = fields::Field::new(String::from("electric field"),file.clone());
        let potentiel_field  = fields::Field::new(String::from("potential field"),file.clone());

        Solver{elements:elements,electric_field:electric_field,potentiel_field,file_h5:file}
    }

    pub fn add_element (&mut self,element:Element) {
        self.elements.push(element);
    }

    pub fn init(&mut self) {
        for element in self.elements.iter_mut() {
            element.init(); // on initalize tous les champs de vecteurs des elements
        }
    }

    pub fn get_file_h5(&self) -> File {
        self.file_h5.clone()
    }

    fn calculate_charge_density(&self) -> Vec<f64> {
        let mut charge_density = vec![0.;NX]; 
        for element in self.elements.iter() {
            for i in 0..NX {
                charge_density[i] += (element.z_charge as f64) * element.density()[i] / (lambda*lambda);
            }
        }
        charge_density
    }

    pub fn next_step(&mut self) {
        for element in self.elements.iter_mut() {
            element.x_advection(DT/2.);
        }

        let charge_density = self.calculate_charge_density();
        self.potentiel_field.solve_potential(charge_density);
        self.potentiel_field.gradient(&mut self.electric_field);

        for element in self.elements.iter_mut() {
            element.v_advection(DT, &self.electric_field.field_values);
        }

        for element in self.elements.iter_mut() {
            element.x_advection(DT/2.);
        }
    }

    pub fn run(&mut self) {

        let mut t = DT;
        let mut k_iter: u32 = 1;
        // println!("before loop");
        while t < T {
            if k_iter % 100 == 0 {
                println!("Solver is at time : {t} \n Number of iteration {k_iter} ");
            }
            
            t += DT; 
            self.next_step();
            k_iter += 1;

            if k_iter% n_save == 0 {
                for element in self.elements.iter() {
                    element.save_h5(self.file_h5.clone(), t);
                }
                self.electric_field.save_h5(self.file_h5.clone(), t);
                self.potentiel_field.save_h5(self.file_h5.clone(), t);
            }

        }
    }
    
}
