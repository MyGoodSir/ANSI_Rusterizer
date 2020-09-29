use super::graphics::*;
use std::{ops::Mul, fmt};
use fmt::Formatter;


//make vectors generic\
#[derive(Copy, Clone)]
pub struct v3{
    pub x: f32,
    pub y: f32,
    pub z: f32
}
#[derive(Copy, Clone)]
pub struct v4{
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32
}

impl v3{
    pub fn new(xin:f32,yin:f32,zin:f32)->v3{
        v3{x:xin,y:yin,z:zin}
    }
}
impl v4{
    pub fn new(xin:f32,yin:f32,zin:f32,win:f32)->v4{
        v4{x:xin,y:yin,z:zin,w:win}
    }
}

#[derive(Copy, Clone)]
pub struct mat4{
    pub a1:v4,
    pub a2:v4,
    pub a3:v4,
    pub a4:v4
}

impl Mul for v4{
    type Output = f32;

    fn mul(self, rhs: Self) -> f32{
        self.x * rhs.x +
        self.y * rhs.y +
        self.z * rhs.z +
        self.w * rhs.w
    }
}
impl Mul<v4> for mat4{
    type Output = v4;

    fn mul(self, rhs: v4) -> v4{
        v4{x: self.a1 * rhs,
           y: self.a2 * rhs,
           z: self.a3 * rhs,
           w: self.a4 * rhs }
    }
}


//clockwise winding order
#[derive(Copy, Clone)]
pub struct tri{
    pub verts: [v3; 3],
    pub col: ansi_color,
}
impl tri{
    pub fn new(p1x:f32,p1y:f32,p1z:f32,
               p2x:f32,p2y:f32,p2z:f32,
               p3x:f32,p3y:f32,p3z:f32) -> tri{
                let mut p1 = v3::new(p1x,p1y,p1z);
                let mut p2 = v3::new(p2x,p2y,p2z);
                let mut p3 = v3::new(p3x,p3y,p3z);
                tri{verts:[p1,p2,p3], col:ansi_color{foreground: ColorValue::blue, background: ColorValue::blue}}
    }
    pub fn translate(&mut self, tvec:v3){
        for i in 0..=2{
            self.verts[i].x += tvec.x;
            self.verts[i].y += tvec.y;
            self.verts[i].z += tvec.z;
        }
    }
}
#[derive(Clone)]
pub struct mesh{
    pub tris: Vec<tri>
}
///////////////////

impl mesh{
    pub fn translate(&mut self, tvec:v3){
        for mut tri in self.tris.iter_mut(){
            tri.translate(tvec);
        }
    }
    pub fn print_tris(&mut self){
        for t in self.tris.iter(){
            for v in t.verts.iter(){
                print!("({}, {}, {}),  ",v.x,v.y,v.z);
            }
            print!("\n");
        }
    }

}


pub fn create_cube() -> mesh{
    let mut v = Vec::<tri>::new();
    //top
    v.push(tri::new(0.0, 1.0, 0.0,   0.0, 1.0, 1.0,   1.0, 1.0, 1.0));
    v.push(tri::new(0.0, 1.0, 0.0,   1.0, 1.0, 1.0,   1.0, 1.0, 0.0));
    //back
    v.push(tri::new(1.0, 0.0, 1.0,   1.0, 1.0, 1.0,   0.0, 1.0, 1.0));
    v.push(tri::new(1.0, 0.0, 1.0,   0.0, 1.0, 1.0,   0.0, 0.0, 1.0));
    //front
    v.push(tri::new(0.0, 0.0, 0.0,   0.0, 1.0, 0.0,   1.0, 1.0, 0.0));
    v.push(tri::new(0.0, 0.0, 0.0,   1.0, 1.0, 0.0,   1.0, 0.0, 0.0));
    //left
    v.push(tri::new(0.0, 0.0, 1.0,   0.0, 1.0, 1.0,   0.0, 1.0, 0.0));
    v.push(tri::new(0.0, 0.0, 1.0,   0.0, 1.0, 0.0,   0.0, 0.0, 0.0));
    //right
    v.push(tri::new(1.0, 0.0, 0.0,   1.0, 1.0, 0.0,   1.0, 1.0, 1.0));
    v.push(tri::new(1.0, 0.0, 0.0,   1.0, 1.0, 1.0,   1.0, 0.0, 1.0));
    //bottom
    v.push(tri::new(1.0, 0.0, 1.0,   0.0, 0.0, 1.0,   0.0, 0.0, 0.0));
    v.push(tri::new(1.0, 0.0, 1.0,   0.0, 0.0, 0.0,   1.0, 0.0, 0.0));

    mesh{tris: v}
}