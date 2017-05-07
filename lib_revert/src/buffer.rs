use util::Color;
use util::None;

pub trait Buffer {
    type T: Sized+Clone;

    fn get_width(&self) -> u32;
    fn get_height(&self) -> u32;
    fn get_pixel(&self, x:u32,y:u32) -> Self::T;
    fn get_pixel_by_coords(&self, x:f32,y:f32) -> Self::T {
        let x_wrap = (x.abs() % 1.0) * (self.get_width()-1) as f32;
        let y_wrap = (y.abs() % 1.0) * (self.get_height()-1) as f32;
        self.get_pixel(x_wrap as u32, y_wrap as u32)
    }
}

pub trait WritableBuffer : Buffer {
    fn set_pixel(&mut self, x:u32,y:u32,value:Self::T);
    
    fn clear(&mut self, clear_val: Self::T) {
        for x in 0..self.get_width() {
            for y in 0..self.get_height() {
                self.set_pixel(x,y, clear_val.clone());
            }
        }
    }
}


//------------------------------Color buffer----------------------------------
#[derive(Clone)]
pub struct ColorBuffer {
    w:u32,
    h:u32,
    bmp: Box<[u8]>
    //bmp: &mut [u8]
}

impl ColorBuffer {
    pub fn new(width:u32,height:u32) -> ColorBuffer {
        let len = (width*height*4) as usize;
        use std::iter::repeat;

        //TODO @oliver: optimise allocation of buffer
        ColorBuffer {
            w:width,
            h:height,
            bmp:repeat(0).take(len).collect::<Vec<u8>>().into_boxed_slice()
            //bmp:repeat(0).take(len).collect::<Vec<u8>>().as_mut_slice()
        }
    }

    pub fn get_width(&self) -> u32 {
        self.w
    }

    pub fn get_height(&self) -> u32 {
        self.h
    }

    pub fn get_bmp(&self) -> Box<[u8]> {
        self.bmp.clone()
    }

    pub fn get_ptr(&self) -> *const u8 {
        self.bmp.as_ref().as_ptr()
    }
}

impl Buffer for ColorBuffer {
    type T = Color;
    fn get_width(&self) -> u32 {
        return self.w;
    }
    fn get_height(&self) -> u32 {
        return self.h;
    }
    fn get_pixel(&self, x:u32,y:u32) -> Color {
        if (x < 0 || x > self.get_width()) {
            return Color {r:255,g:0,b:0}
        }
        if (y < 0 || y > self.get_height()) {
            return Color {r:255,g:0,b:0}
        }
        return Color {
            r: self.bmp[(((x as usize)+(y*self.w) as usize)*4)+2],
            g: self.bmp[(((x as usize)+(y*self.w) as usize)*4)+1],
            b: self.bmp[(((x as usize)+(y*self.w) as usize)*4)+0]
        }
    }
}

impl WritableBuffer for ColorBuffer {
    fn set_pixel(&mut self, x:u32,y:u32,value:Color) {
        self.bmp[(((x as usize)+(y*self.w) as usize)*4)]=value.b; //B
        self.bmp[(((x as usize)+(y*self.w) as usize)*4)+1]=value.g; //G
        self.bmp[(((x as usize)+(y*self.w) as usize)*4)+2]=value.r; //R
        self.bmp[(((x as usize)+(y*self.w) as usize)*4)+3]=0;
    }
}

//------------------------------Depth buffer----------------------------------
#[derive(Clone)]
pub struct DepthBufferF32 {
    w:u32,
    h:u32,
    depth: Box<[f32]>
}

impl DepthBufferF32 {
    pub fn new(width:u32,height:u32) -> DepthBufferF32 {
        let len = (width*height) as usize;
        use std::iter::repeat;

        //TODO @oliver: optimise allocation of buffer
        DepthBufferF32 {
            w:width,
            h:height,
            depth:repeat(10.0).take(len).collect::<Vec<f32>>().into_boxed_slice()
        }
    }
}

impl Buffer for DepthBufferF32 {
    type T = f32;

    fn get_width(&self) -> u32 {
        return self.w;
    }
    fn get_height(&self) -> u32 {
        return self.h;
    }
    fn get_pixel(&self, x:u32,y:u32) -> f32 {
        return self.depth[(((x as usize)+(y*self.w) as usize))];
    }

}

impl WritableBuffer for DepthBufferF32 {
    fn set_pixel(&mut self, x:u32,y:u32,value:f32) {
        self.depth[(((x as usize)+(y*self.w) as usize))]=value;
    }
}

//------------------------------Depth buffer----------------------------------
pub struct NoneBuffer {
    w:u32,
    h:u32,
    //depth: Box<[f32]>
}

impl NoneBuffer {
    pub fn new(width:u32,height:u32) -> NoneBuffer {
        //let len = (width*height) as usize;
        //use std::iter::repeat;

        //TODO @oliver: optimise allocation of buffer
        NoneBuffer {
            w:width,
            h:height//,
            //depth:repeat(10.0).take(len).collect::<Vec<f32>>().into_boxed_slice()
        }
    }
}

impl Buffer for NoneBuffer {
    type T = None;

    fn get_width(&self) -> u32 {
        return self.w;
    }
    fn get_height(&self) -> u32 {
        return self.h;
    }
    fn get_pixel(&self, x:u32,y:u32) -> None {
        //return self.depth[(((x as usize)+(y*self.w) as usize))];
        None{}
    }

}

impl WritableBuffer for NoneBuffer {
    fn set_pixel(&mut self, x:u32,y:u32,value:None) {
        //self.depth[(((x as usize)+(y*self.w) as usize))]=value;
    }
}
