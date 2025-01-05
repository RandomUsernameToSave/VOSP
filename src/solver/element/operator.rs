use super::Element;
mod boundary_conditions_element;


impl Element {
    // BOTH FAILS FOR NOW
    pub fn x_advection(&mut self,DT:f64) {
        
        let cached_grid = self.element_grid.clone();
        

        let mut v1;
        let mut calc_ind:isize;
        let mut start_ind:usize ;
        let mut th ;

        let mut betal ;
        let mut betar;
        let mut betac;

        let mut al ;
        let mut ar ;
        let mut ac:f64;
        let mut r0:f64;
        let mut r1:f64;
        let mut r2:f64;
        let mut r0p:f64;
        let mut r1p:f64;
        let mut r2p:f64;
        let mut t0:f64;
        let mut t1:f64;
        let mut t2:f64;
        let mut q:f64;


        for ix in 0..self.config.NX {
            for iv  in 0..self.config.NV {

                v1 = -self.lv + ( iv as f64)*self.dv;
                
                calc_ind = ix as isize- (v1*DT*self.config.dxi ).ceil() as isize;
                //calc_ind -= 1;
                th = ((v1*DT*self.config.dxi).ceil() as isize) as f64 - v1*DT*self.config.dxi;

                if calc_ind <= 0 {
                    self.element_grid[ix][iv] = self.boundary_conditions("lx",ix,iv);
                }
                else if calc_ind >= ((self.config.NX-2) as isize) {
                    self.element_grid[ix][iv] = self.boundary_conditions("ux",ix,iv);
                }
                else {
                    
                start_ind = calc_ind as usize;


                betal =  (cached_grid[start_ind][iv] - cached_grid[start_ind-1][iv]).powi(2);
                betar= (cached_grid[start_ind+1][iv] - cached_grid[start_ind][iv]).powi(2);
                betac = ((cached_grid[start_ind+1][iv] - 2. * cached_grid[start_ind][iv] + cached_grid[start_ind-1][iv]).powi(2)) * 13./3. 
                        + 0.25 * (( cached_grid[start_ind+1][iv] - cached_grid[start_ind-1][iv]).powi(2));

                al = 0.25 / ((self.config.epsi + betal).powi(2));
                ar = 0.25 / ((self.config.epsi + betar).powi(2));
                ac = 0.5 / ((self.config.epsi + betac).powi(2));

                betar = ar/ (ar+ac+al);
                betal = al/ (ar+ac+al);
                betac = ac/ (ar+ac+al);

                r0 = cached_grid[start_ind][iv] - 1./12. * betac * (cached_grid[start_ind+1][iv] - 2.* cached_grid[start_ind][iv] + cached_grid[start_ind-1][iv]);
                r1 =    betal * self.config.dxi * (cached_grid[start_ind][iv] - cached_grid[start_ind-1][iv])  + 
                        betar * self.config.dxi * (cached_grid[start_ind+1][iv] - cached_grid[start_ind][iv]) + 
                        betac * 0.5*self.config.dxi * (cached_grid[start_ind+1][iv] - cached_grid[start_ind-1][iv]);
                r2 = 2.* betac * self.config.dxi*self.config.dxi*((cached_grid[start_ind+1][iv] - 2.* cached_grid[start_ind][iv] + cached_grid[start_ind-1][iv]));
                

                // SECOND PASS
                // 
                // SECOND PASS 
                
                start_ind += 1;

                betal =  (cached_grid[start_ind][iv] - cached_grid[start_ind-1][iv]).powi(2);
                betar= (cached_grid[start_ind+1][iv] - cached_grid[start_ind][iv]).powi(2);
                betac = 13./3. * ((cached_grid[start_ind+1][iv] - 2. * cached_grid[start_ind][iv] + cached_grid[start_ind-1][iv]).powi(2)) + 0.25 * (( cached_grid[start_ind+1][iv] - cached_grid[start_ind-1][iv]).powi(2));

                al = 0.25 / ((self.config.epsi + betal).powi(2));
                ar = 0.25 / ((self.config.epsi + betar).powi(2));
                ac = 0.5 / ((self.config.epsi + betac).powi(2));

                betar = ar/ (ar+ac+al);
                betal = al/ (ar+ac+al);
                betac = ac/ (ar+ac+al);

                r0p = cached_grid[start_ind][iv] - 1./12. * betac * (cached_grid[start_ind+1][iv] - 2.* cached_grid[start_ind][iv] + cached_grid[start_ind-1][iv]);
                r1p =    betal * self.config.dxi * (cached_grid[start_ind][iv] - cached_grid[start_ind-1][iv])  + 
                        betar * self.config.dxi * (cached_grid[start_ind+1][iv] - cached_grid[start_ind][iv]) + 
                        betac * 0.5*self.config.dxi * (cached_grid[start_ind+1][iv] - cached_grid[start_ind-1][iv]);
                r2p = 2.* betac * self.config.dxi*self.config.dxi*((cached_grid[start_ind+1][iv] - 2.* cached_grid[start_ind][iv] + cached_grid[start_ind-1][iv]));

                // LAST RECONSTRUCTION 
                //
                // LAST RECONSTRUCTION 

                t0 =(1.-th)*r0 + th*r0p;
                t1 = self.config.DX* th*(1.-th) * (0.5*r1 - 0.5 * r1p);
                q = 3.*th - 6.*th*th + 4. * th*th*th;
                t2 = self.config.DX*self.config.DX /24.* ( (1.-q)*r2 +q*r2p);
                self.element_grid[ix][iv] = t0+t1+t2;
                }

            }
        }
    }

    pub fn v_advection(&mut self,DT:f64, elec_field: &Vec<f64>) {
        
        let cached_grid = self.element_grid.clone();
        let dvi:f64 = 2.* self.lv/(self.config.NV as f64);

        let mut v1;
        let mut start_ind ;
        let mut calc_ind:isize;
        let mut th ;

        let mut betal ;
        let mut betar;
        let mut betac ;

        let mut al ;
        let mut ar ;
        let mut ac:f64;
        let mut r0:f64;
        let mut r1:f64;
        let mut r2:f64;
        let mut r0p:f64;
        let mut r1p:f64;
        let mut r2p:f64;
        let mut t0:f64;
        let mut t1:f64;
        let mut t2:f64;
        let mut q:f64;


        for ix in 0..self.config.NX {
            for iv  in 0..self.config.NV {

                v1 = -( self.z_charge as f64) *elec_field[ix]/self.mu;
                calc_ind = iv as isize- (v1*DT*self.dv ).ceil() as isize;
                //calc_ind -= 1;
                th = ((v1*DT*self.dv).ceil() as isize) as f64 - v1*DT*self.dv;

                if calc_ind <= 0 {
                    self.element_grid[ix][iv] = self.boundary_conditions("lv",ix,iv);
                }
                else if calc_ind >= ((self.config.NV-2) as isize) {
                    self.element_grid[ix][iv] = self.boundary_conditions("uv",ix,iv);
                }
                else {
                    
                start_ind = calc_ind as usize;


                betal =  (cached_grid[ix][start_ind] - cached_grid[ix][start_ind-1]).powi(2);
                betar= (cached_grid[ix][start_ind+1] - cached_grid[ix][start_ind]).powi(2);
                betac = 13./3. * ((cached_grid[ix][start_ind+1] - 2. * cached_grid[ix][start_ind] + cached_grid[ix][start_ind-1]).powi(2)) + 0.25 * (( cached_grid[ix][start_ind+1] - cached_grid[ix][start_ind-1]).powi(2));

                al = 0.25 / ((self.config.epsi + betal).powi(2));
                ar = 0.25 / ((self.config.epsi + betar).powi(2));
                ac = 0.5 / ((self.config.epsi + betac).powi(2));

                betar = ar/ (ar+ac+al);
                betal = al/ (ar+ac+al);
                betac = ac/ (ar+ac+al);

                r0 = cached_grid[ix][start_ind] - 1./12. * betac * (cached_grid[ix][start_ind+1] - 2.* cached_grid[ix][start_ind] + cached_grid[ix][start_ind-1]);
                r1 =    betal * (cached_grid[ix][start_ind] - cached_grid[ix][start_ind-1]) /self.dv + 
                        betar/self.dv * (cached_grid[ix][start_ind+1] - cached_grid[ix][start_ind]) + 
                        0.5*betac/self.dv * (cached_grid[ix][start_ind+1] - cached_grid[ix][start_ind-1]);
                r2 = 2.* betac /self.dv/self.dv*((cached_grid[ix][start_ind+1] - 2.* cached_grid[ix][start_ind] + cached_grid[ix][start_ind-1]));
                

                // SECOND PASS
                // 
                // SECOND PASS 
                
                start_ind += 1;

                betal =  (cached_grid[ix][start_ind] - cached_grid[ix][start_ind-1]).powi(2);
                betar= (cached_grid[ix][start_ind+1] - cached_grid[ix][start_ind]).powi(2);
                betac = 13./3. * ((cached_grid[ix][start_ind+1] - 2. * cached_grid[ix][start_ind] + cached_grid[ix][start_ind-1]).powi(2)) + 0.25 * (( cached_grid[ix][start_ind+1] - cached_grid[ix][start_ind-1]).powi(2));

                al = 0.25 / ((self.config.epsi + betal).powi(2));
                ar = 0.25 / ((self.config.epsi + betar).powi(2));
                ac = 0.5 / ((self.config.epsi + betac).powi(2));

                betar = ar/ (ar+ac+al);
                betal = al/ (ar+ac+al);
                betac = ac/ (ar+ac+al);

                r0p = cached_grid[ix][start_ind] - 1./12. * betac * (cached_grid[ix][start_ind+1] - 2.* cached_grid[ix][start_ind] + cached_grid[ix][start_ind-1]);
                r1p =    betal * (cached_grid[ix][start_ind] - cached_grid[ix][start_ind-1]) /self.dv + 
                        betar/self.dv * (cached_grid[ix][start_ind+1] - cached_grid[ix][start_ind]) + 
                        0.5*betac/self.dv * (cached_grid[ix][start_ind+1] - cached_grid[ix][start_ind-1]);
                r2p = 2.* betac /self.dv/self.dv*((cached_grid[ix][start_ind+1] - 2.* cached_grid[ix][start_ind] + cached_grid[ix][start_ind-1]));

                // LAST RECONSTRUCTION 
                //
                // LAST RECONSTRUCTION 

                t0 =(1.-th)*r0 + th*r0p;
                t1 = self.config.DX* th*(1.-th) * (0.5*r1 - 0.5 * r1p);
                q = 3.*th - 6.*th*th + 4. * th*th*th;
                t2 = self.config.DX*self.config.DX /24.* ( (1.-q)*r2 +q*r2p);
                self.element_grid[ix][iv] = t0+t1+t2;
                }

            }
        }
    }
}