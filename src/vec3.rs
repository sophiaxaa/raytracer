
use std::ops;
use std::io::Write;

pub struct Vec3{
    pub e: [f64; 3], 
}


impl Vec3{

    pub fn new(e0: f64, e1: f64, e2: f64) -> Self{
        Vec3 { e: [e0, e1, e2] }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

      //all return a vector 


    pub fn unit_vector(&self) -> Vec3{
        let length = self.length();
        Vec3{
            e:[
                self.e[0] / length,
                self.e[1] / length, 
                self.e[2] / length,
            ]
        }
    }


    //dot product
    pub fn dot(&self, other: &Vec3) -> f64{
        self.e[0] * other.e[0] + self.e[1] * other.e[1] + self.e[2] * other.e[2]
    }

    //cross product
    pub fn cross(&self, other: &Vec3) -> Vec3{
        Vec3 {
            e: [
                self.e[1] * other.e[2] - self.e[2] * other.e[1],
                self.e[2] * other.e[0] - self.e[0] * other.e[2],
                self.e[0] * other.e[1] -  self.e[1] * other.e[0],
            ]

    }
    }
}
    //exist in std::ops 
    //add
    impl ops::Add for Vec3{
        type Output = Vec3;

        fn add(self, other: Vec3) -> Vec3{
            //vec3 is a vector, so add components 
            //new vec3 instance 
            Vec3{
                //  redefine e
                e:[
                    
                    self.e[0] + other.e[0],
                    self.e[1] + other.e[1],
                    self.e[2] + other.e[2],
                ]
            }
        }
    }

    //subtract
    impl ops::Sub for Vec3{
        type Output = Vec3;

        fn sub(self, other: Vec3) -> Vec3{
            Vec3{
                e:[
                    self.e[0] - other.e[0],
                    self.e[1] - other.e[1],
                    self.e[2] - other.e[2],
                ]
            }
        }
    }


    //multiply by a SCALAR
    impl ops::Mul<f64> for Vec3{
        type Output = Vec3; 

        fn mul(self, num: f64) -> Vec3{
           Vec3{
            e:[
                self.e[0] * num,
                self.e[1] * num,
                self.e[2] * num, 
            ]
           } 
        }

    }

    //divide by a SCALAR
    impl ops::Div<f64> for Vec3{
        type Output = Vec3; 

        fn div(self, num: f64) -> Vec3{
            Vec3{
                e:[
                    self.e[0] / num,
                    self.e[1] / num,
                    self.e[2] / num, 
                ]
            }
            
        }

    }

    //negate 
    impl ops::Neg for Vec3{
        type Output = Vec3;

        fn neg(self) -> Vec3{
         Vec3{
             e:[
                -self.e[0],
                -self.e[1],
                -self.e[2],
             ]
          }
        }

    }


    //implement indexing operators

    impl std::ops::Index<usize> for Vec3{
        type Output = f64;

        fn index(&self, i: usize) -> &f64{
            &self.e[i]

        }
    }

    impl std::ops::IndexMut<usize> for Vec3{

    fn index_mut(&mut self, i: usize) -> &mut f64{
        &mut self.e[i]
    }

    }

    //compound assignment operators 
    //modifying current instance instead of returning a vec3

    impl ops::AddAssign for Vec3{
        fn add_assign(&mut self, other: Vec3){
            
            self.e[0] += other.e[0];
            self.e[1] += other.e[1];
            self.e[2] += other.e[2];
                
            
        }
    }

    impl ops:: MulAssign<f64> for Vec3{
        fn mul_assign(&mut self, num: f64 ){
                    self.e[0] *= num;
                    self.e[1] *= num;
                    self.e[2] *= num; 

        }
    }
    
    impl ops:: DivAssign<f64> for Vec3{
        fn div_assign(&mut self, num: f64){
                    self.e[0] /= num;
                    self.e[1] /= num;
                    self.e[2] /= num;
        }
    }

    pub type Point3 = Vec3;
    pub type Color = Vec3;

    //color is an alias for vec3, &color is referencing vec3
    pub fn write_color<W: Write>(out: &mut W, pixel_color: &Color) {
        let r = pixel_color.x();
        let g = pixel_color.y();
        let b = pixel_color.z();

        //translate to the valid byte range (rather than 0-1)

        let r_byte = (255.999 * r) as u8;
        let g_byte = (255.999 * g) as u8;
        let b_byte = (255.999 * b) as u8;

        writeln!(out, "{} {} {}", r_byte, g_byte, b_byte).unwrap()
    }