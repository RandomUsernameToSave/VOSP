pub mod element;
mod fields;
use element::Element;

use hdf5::File;
use crate::config::Config;
pub struct Solver {
    elements:Vec<Element>,
    electric_field:fields::Field,
    potentiel_field:fields::Field,
    file_h5 : File,
    config: Config,
}

impl Solver {
    pub fn new(config:Config)-> Solver { 
        let elements    = vec![];

        let file = File::create("VOSP.h5").unwrap();
        let electric_field  = fields::Field::new(String::from("electric field"),file.clone(),config.clone());
        let potentiel_field  = fields::Field::new(String::from("potential field"),file.clone(),config.clone());

        Solver{elements:elements,electric_field:electric_field,potentiel_field,file_h5:file,config:config}
    }

    pub fn add_element (&mut self,element:Element) {
        self.elements.push(element);
    }

    pub fn init(&mut self) {
        for element in self.elements.iter_mut() {
            element.init(); // on initalize tous les champs de vecteurs des elements
            element.config = self.config.clone();
        }
    }

    pub fn get_file_h5(&self) -> File {
        self.file_h5.clone()
    }

    fn calculate_charge_density(&self) -> Vec<f64> {
        let mut charge_density = vec![0.;self.config.NX]; 
        for element in self.elements.iter() {
            for i in 0..self.config.NX {
                charge_density[i] += (element.z_charge as f64) * element.density()[i] / (self.config.lambda*self.config.lambda);
            }
        }
        charge_density
    }

    pub fn next_step(&mut self) {
        for element in self.elements.iter_mut() {
            element.x_advection(self.config.DT/2.);
        }

        let charge_density = self.calculate_charge_density();
        self.potentiel_field.solve_potential(charge_density);
        self.potentiel_field.gradient(&mut self.electric_field);

        for element in self.elements.iter_mut() {
            element.v_advection(self.config.DT, &self.electric_field.field_values);
        }

        for element in self.elements.iter_mut() {
            element.x_advection(self.config.DT/2.);
        }
    }

    pub fn run(&mut self) {

        let mut t = self.config.DT;
        let mut k_iter: u32 = 1;
        // println!("before loop");
        while t < self.config.end_time {
            if k_iter % 100 == 0 {
                println!("Solver is at time : {t} \n Number of iteration {k_iter} ");
            }
            
            t += self.config.DT; 
            self.next_step();
            k_iter += 1;

            if k_iter% self.config.n_save == 0 {
                for element in self.elements.iter() {
                    element.save_h5(self.file_h5.clone(), t);
                }
                self.electric_field.save_h5(self.file_h5.clone(), t);
                self.potentiel_field.save_h5(self.file_h5.clone(), t);
            }

        }
    }
    
}
