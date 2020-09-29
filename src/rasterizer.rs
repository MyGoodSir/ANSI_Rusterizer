use super::graphics::*;
use super::geometry::*;
use super::terminal::*;
use std::fmt;
use fmt::Formatter;
use std::f32::consts::PI;

//projection
//[x,y,z] -> [(height/width) (1/tan(theta/2)) x / z, 
//            (1/tan(theta/2)) y / z, 
//            z * (zf/(zf-zn)) - (zf*zn)/(zf-zn)]

/*
 *aspect ratio: a = (width/height)
 *field of view: fov = (1/tan(theta/2))
 *viewplane normalization: q = (zfar/(zfar-znear))
 *
 * new normalization equation:
 *
 * [(f*fov*x)/z, (fov*y)/z, z*q - znear * q]
 *
 *
 *    a*fov      0      0      0
 *        0    fov      0      0
 *        0      0      q  -zn*q 
 *        0      0      1      0    
 */


 
pub struct Scene{
    pub width: u32,
    pub height: u32,
    pub fov:f32,
    pub asp_rt: f32,
    pub zfar:f32,
    pub znear:f32,
    pub proj_mat:mat4,
    pub rend:Renderer,
    pub console:Terminal,
    objects: Vec<mesh>
}
impl Scene{

    pub fn new(width: u32, height: u32) -> Self{
        let zero = v4{x: 0.0, y: 0.0, z: 0.0, w: 0.0};
        let mut o = Self{width:  width, height: height, 
                   fov: 1.0 / (90.0 * 0.5 / 180.0 * PI).tan(),
                   asp_rt: ((height as f32)/(width as f32)),
                   zfar: 1000.0,
                   znear: 0.1,
                   proj_mat:mat4{ a1: zero, a2: zero, a3: zero, a4: zero},
                   rend: Renderer::new(width as i32, height as i32),
                   console: Terminal::new(),
                   objects: Vec::<mesh>::new()
                };
        o.init_console();
        o.init_rend();
        o.make_proj_matrix();
        o.width = o.console.width;
        o.height = o.console.height;
        o
    }
    pub fn init_console(&mut self){
        self.console.get_handles();
        self.console.setup_font(4);
        self.console.resize_window(self.width as i32, self.height as i32);
        self.console.get_terminal_size();
        self.console.resize_buffer(self.console.width as i16, self.console.height as i16);
        self.console.clear();
        self.console.hide_cursor();
    }
    pub fn init_rend(&mut self){
        self.rend = Renderer::new(self.console.width as i32, self.console.height as i32);
        self.rend.init_pbuff();
    }
    pub fn reset_frame(&mut self){
        self.console.clear();
        self.rend.reset_pbuff();
        self.rend.reset_bbuff();
    }
    pub fn draw_objects_wireframe(&mut self){
        let obj_itr: Vec<mesh> = self.objects.clone();
        for obj in obj_itr{
            let tris = self.get_proj_tris(obj);
            for t in tris{
                let ps = t.verts;
                self.rend.draw_triangle(ps[0].x as i32, ps[0].y as i32, ps[1].x as i32 , ps[1].y as i32, ps[2].x as i32, ps[2].y as i32, t.col.foreground);
            }
        }
    }
    pub fn display(&mut self){
        self.rend.bits_to_px(); 
        print!("{}",self.rend.px_to_string());
    }

    pub fn add_object(&mut self, obj: mesh){
        self.objects.push(obj);
    }
    pub fn rotate(pos: v3) -> v3{
        let mut pos4 = v4{x:pos.x, y:pos.y, z:pos.z, w: 1.0};
        let rotx: mat4 = mat4{
            a1: v4::new(1.0, 0.0,0.0,0.0),
            a2: v4::new(0.0, (0.005f32).cos(), -1.0*(0.005f32).sin(),0.0),
            a3: v4::new(0.0, (0.005f32).sin(), (0.005f32).cos(), 0.0),
            a4: v4::new(0.0, 0.0,0.0,1.0)
        };
        let rotz: mat4 = mat4{
            a1: v4::new((0.01f32).cos(), -1.0*(0.01f32).sin(),0.0,0.0),
            a2: v4::new((0.01f32).sin(), (0.01f32).cos(),0.0,0.0),
            a3: v4::new(0.0, 0.0, 1.0, 0.0),
            a4: v4::new(0.0, 0.0,0.0,1.0)
        };

        pos4 = rotx*pos4;
        pos4 = rotz*pos4;

        v3::new(pos4.x,pos4.y,pos4.z)


    }

    pub fn make_proj_matrix(&mut self){
        let q = self.zfar/(self.zfar-self.znear);
        self.proj_mat = mat4{
            a1: v4::new(self.asp_rt*self.fov, 0.0,0.0,0.0),
            a2: v4::new(0.0, self.fov,0.0,0.0),
            a3: v4::new(0.0, 0.0, q, -1.0*self.znear*q),
            a4: v4::new(0.0, 0.0,1.0,0.0)
        };
    }
    pub fn to_ndc(&mut self, pos:v3) -> v3{
        let pos4 = v4{x:pos.x, y:pos.y, z:pos.z, w: 1.0};
        let o = self.proj_mat*pos4;
        if o.w != 0f32{
            v3{ x:o.x/o.w, y:o.y/o.w, z:o.z }
        }else{
            v3{ x:o.x, y:o.y, z:o.z }
        }
    }
    pub fn scale_to_view(&mut self, point:v3) -> v3{
        let mut output = point;
        output.x += 1.0; output.y += 1.0;
        output.x *= 0.5 * self.width as f32;
        output.y *= 0.5 * self.height as f32;
        output
    }
    pub fn update(&mut self){
        for t in self.objects[0].tris.iter_mut(){
            t.verts[0] = Self::rotate(t.verts[0]);
            t.verts[1] = Self::rotate(t.verts[1]);
            t.verts[2] = Self::rotate(t.verts[2]);
        }
    }
    pub fn get_proj_tris(&mut self, obj:mesh) -> Vec<tri>{
        let mut temp_obj = obj.tris.clone();

        for t in temp_obj.iter_mut(){
            t.translate(v3{x:0.0,y:0.0,z:1.25});

            t.verts[0] = self.to_ndc(t.verts[0]);
            t.verts[1] = self.to_ndc(t.verts[1]);
            t.verts[2] = self.to_ndc(t.verts[2]);

            t.verts[0] = self.scale_to_view(t.verts[0]);
            t.verts[1] = self.scale_to_view(t.verts[1]);
            t.verts[2] = self.scale_to_view(t.verts[2]);
        }
        temp_obj

    }
}