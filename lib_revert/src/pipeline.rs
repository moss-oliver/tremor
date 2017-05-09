//use Vec2u32;
//use Vec3f32;
use cgmath::Vector4;
use buffer::WritableBuffer;
use std::cmp;
use util::Rasterizable;
use util::Tri;
use vertex_shader::VertexShader;
use pixel_shader::PixelShader;
use pixel_masker::PixelMasker;
use pixel_masker::NonePixelMasker;
use std::mem;
//Templates: Vertex, Vertex Shader, Rasterize Vertex, Pixel Shader, Back Buffer pixel type, Masker pixel
pub struct Pipeline<V, VS, RV, PS, BP, MS, MP> where V: Sized, VS: VertexShader<V=V,R=RV>, RV:Rasterizable, PS: PixelShader<R=RV, P=BP>, BP: Sized+Clone, MS: PixelMasker<V=RV,P=MP>, MP: Sized+Clone {
    pub vs:Option<VS>,
    pub ps:Option<PS>,
    pub masker:Option<MS>
}

impl<V, VS, RV, PS, BP, MS, MP> Pipeline<V, VS, RV, PS, BP, MS, MP> where V: Sized, VS: VertexShader<V=V,R=RV>, RV:Rasterizable+Clone, PS: PixelShader<R=RV, P=BP>, BP: Sized+Clone, MS: PixelMasker<V=RV,P=MP>, MP: Sized+Clone {
    pub fn new(vs: VS, ps: PS, ms: MS) -> Pipeline<V, VS, RV, PS, BP, MS, MP> {
        Pipeline {
            vs: Option::Some(vs),
            ps: Option::Some(ps),
            masker: Option::Some(ms)
            //masker: NonePixelMasker::<RV>::new()
        }
    }

    //BB: Backbuffer - What is drawn to. (eg. colorbuffer)
    //MB: Maskbuffer - What is tested against (eg. depthbuffer)
    pub fn create_session<'a, BB, MB>(&'a self, buffer: &'a mut BB, test_buffer: &'a mut MB) -> Result<PipelineSession<'a, V, VS, RV, PS, BB, BP, MS, MB, MP>, &'static str> where BB: WritableBuffer<T=BP> + 'a, MB: WritableBuffer<T=MP> + 'a {
        
        if buffer.get_width() != test_buffer.get_width() {
            return Result::Err("create_session buffer width is not consistent.");
        }
        if buffer.get_height() != test_buffer.get_height() {
            return Result::Err("create_session buffer height is not consistent.");
        }
        
        return Result::Ok(PipelineSession {
            //pipe: Box::new(self),
            vs: self.vs.clone().unwrap(),
            ps: self.ps.clone().unwrap(),
            buff: buffer,
            masker: self.masker.clone().unwrap(),
            test_buff: test_buffer
        })
    }
}

pub struct PipelineSession<'a, V, VS, RV, PS, BB:'a, BP, MS, MB:'a, MP> where V: Sized, VS: VertexShader<V=V,R=RV>, RV:Rasterizable, PS: PixelShader<R=RV, P=BP>, BB: WritableBuffer<T=BP>, BP: Sized+Clone, MS: PixelMasker<V=RV,P=MP>, MB: WritableBuffer<T=MP> + 'a, MP: Sized+Clone{
    vs: VS,
    ps: PS,
    buff: &'a mut BB,
    masker: MS,
    test_buff: &'a mut MB
}

fn ipart(val: f32) -> i32 {
    val as i32
}
fn round(val: f32) -> i32 {
    self::ipart(val + 0.5)
}
fn fpart(val:f32) -> f32 {
    if val < 0.0 {
        return 1.0 - (val-val.floor());
    }
    return val-val.floor();
}
fn rfpart(val:f32) -> f32 {
    return 1.0 - self::fpart(val);
}

fn min<T:PartialOrd>(a:T,b:T)->T { if a<b{a}else{b}}

fn max<T:PartialOrd>(a:T,b:T)->T { if a>b{a}else{b}}

impl<'a, V, VS, RV, PS, BB:'a, BP, MS, MB:'a, MP> PipelineSession<'a, V, VS, RV, PS, BB, BP, MS, MB, MP> where V: Sized, VS: VertexShader<V=V,R=RV>, RV:Rasterizable, PS: PixelShader<R=RV, P=BP>, BB: WritableBuffer<T=BP>, BP: Sized+Clone, MS: PixelMasker<V=RV,P=MP>, MB: WritableBuffer<T=MP> + 'a, MP: Sized+Clone {
    
    //pub fn clear(&mut self, clear_val: BP) {
        //for x in 0..self.buff.get_width() {
        //    for y in 0..self.buff.get_height() {
        //        self.buff.set_pixel(x,y, clear_val.clone());
        //    }
        //}
    //    self.buff.clear(clear_val);
    //}

    pub fn draw_point(&mut self, vertex: V) -> u32 {
        let vert1 = self.vs.transform(vertex);
        let p1 = vert1.get_position().truncate() * (1.0 /vert1.get_position().w.abs());
        if p1.z < 0.0 {
            return 0;
        }
        
        let size_x = self.buff.get_width() as f32;
        let size_y = self.buff.get_height() as f32;


        let x1 = (((p1.x+1.0) * size_x /2.0) );
        let y1 = (((p1.y+1.0) * size_y /2.0) );
        if (x1 as u32) < 0 || (x1 as u32) > self.buff.get_width()-1 {
            return 0;
        }
        if (y1 as u32) < 0 || (y1 as u32) > self.buff.get_height()-1 {
            return 0;
        }
        return PipelineSession::<'a, V, VS, RV, PS, BB, BP, MS, MB, MP>::
            render_point(vert1, x1 as u32, y1 as u32, &mut self.buff, &self.ps, &mut self.test_buff, &self.masker);
    }

    pub fn draw_line(&mut self, v1: V, v2: V) -> u32 {

        //Run vertex shader
        let vert1 = self.vs.transform(v1);
        let vert2 = self.vs.transform(v2);
        let p1 = vert1.get_position().truncate() * (1.0 /vert1.get_position().w.abs());
        let p2 = vert2.get_position().truncate() * (1.0 /vert2.get_position().w.abs());

        let size_x = self.buff.get_width() as f32;
        let size_y = self.buff.get_height() as f32;

        let mut count = 0;
        
        if p1.z < 0.0 && p2.z < 0.0 {
            return 0;
        }
        /*
        let size_x = self.buff.get_width() as f32;
        let size_y = self.buff.get_height() as f32;


        let x1 = (((p1.x+1.0) * size_x /2.0) ) as i32;
        let y1 = (((p1.y+1.0) * size_y /2.0) ) as i32;
        let x2 = (((p2.x+1.0) * size_x /2.0) ) as i32;
        let y2 = (((p2.y+1.0) * size_y /2.0) ) as i32;

        if x1 < 0 && x2 < 0 {
            return 0;
        }
        if y1 < 0 && y2 < 0 {
            return 0;
        }
        if x1 as u32 > self.buff.get_width() && x2 as u32 > self.buff.get_width() {
            return 0;
        }
        if y1 as u32 > self.buff.get_width() && y2 as u32 > self.buff.get_width() {
            return 0;
        }
        // Deltas
        let dx = (x2 - x1) as i32;
        let dy = (y2 - y1) as i32;
        
        let mut count = 0;
        //if dx > dy {
        /*for x in cmp::min(x1,x2)..cmp::max(x1,x2) {
            if x > 0 && x < self.buff.get_width() as i32-1 {
                //println!("x: {:?}", x);
                let y = y1 + dy * (x- cmp::min(x1,x2)) / dx;
                if y > 0 && y < self.buff.get_height() as i32-1 {
                    let diff = x as f32 / ((cmp::max(x1.abs(),x2.abs())-cmp::min(x1.abs(),x2.abs())) as f32) ;
                    let vert = vert1.scale(diff).add(&vert2.scale(1.0-diff));
                    count += PipelineSession::<'a, V, VS, RV, PS, BB, BP, MS, MB, MP>::render_point(vert, x as u32, y as u32, &mut self.buff, &self.ps, &mut self.test_buff, &self.masker);
                }
            }
        }*/
        let px1 = cmp::min(x1,x2);
        let px2 = cmp::max(x1,x2);
        let py1 = cmp::min(y1,y2);
        let py2 = cmp::max(y1,y2);

        let D = 2*dy - dx;
        let y = py1
        for x in px1..px2 {
            let vert = vert1.scale(diff).add(&vert2.scale(1.0-diff));
            count += PipelineSession::<'a, V, VS, RV, PS, BB, BP, MS, MB, MP>::render_point(vert, x as u32, y as u32, &mut self.buff, &self.ps, &mut self.test_buff, &self.masker);

        }

        D = 2*dy - dx
        y = y0

        for x from x0 to x1
            plot(x,y)
            if D > 0
                y = y + 1
                D = D - dx
            end if
            D = D + dy
        //}
        //for x from x1 to x2 {
        //y = y1 + dy * (x - x1) / dx
        //plot(x, y)
        //}

        //let vert = vert1.scale(bary1).add(&vert2.scale(bary2).add(&vert3.scale(bary3)));

        //count += PipelineSession::<'a, V, VS, RV, PS, BB, BP, MS, MB, MP>::render_point(vert, x as u32, y as u32, &mut self.buff, &self.ps, &mut self.test_buff, &self.masker);
*/

        let steep = (p2.y - p1.y).abs() > (p2.x - p1.x).abs();

        let px1 = (((p1.x+1.0) * size_x /2.0) );
        let py1 = (((p1.y+1.0) * size_y /2.0) );
        let px2 = (((p2.x+1.0) * size_x /2.0) );
        let py2 = (((p2.y+1.0) * size_y /2.0) );

        let x1 = px1; //min(px1, px2);
        let y1 = py1; //min(py1, py2);
        let x2 = px2; //max(px1, px2);
        let y2 = py2; //max(py1, py2);

        let x_min = min(px1, px2);
        let x_max = max(px1, px2);
        let y_min = min(py1, py2);
        let y_max = max(py1, py2);
        // Deltas
        let dx = (x2 - x1);
        let dy = (y2 - y1);
        //let m = dy / dx;
        //let b = y1 - m * x1;

        if steep {
            //y-centric
            let m = dx / dy;
            let b = x1 - m * y1;
            for y_val in min(y_min,size_x) as u32..min(y_max,size_x) as u32 {
                let y = y_val as f32;
                if y > 0.0 && y < size_y - 1.0 {
                    let x = m*y+b;
                    if x > 0.0 && x < (size_x - 1.0) {
                        let diff = y / ((max(y1.abs(),y2.abs())-min(y1.abs(),y2.abs())) as f32) ;
                        let vert = vert1.scale(diff).add(&vert2.scale(1.0-diff));
                        count += PipelineSession::<'a, V, VS, RV, PS, BB, BP, MS, MB, MP>::render_point(vert, x as u32, y_val, &mut self.buff, &self.ps, &mut self.test_buff, &self.masker);
                    }
                }
            }
        } else {
            //x-centric
            let m = dy / dx;
            let b = y1 - m * x1;
            for x_val in min(x_min,size_x) as u32..min(x_max,size_x) as u32 {
                let x = x_val as f32;
                if x > 0.0 && x < size_x - 1.0 {
                    let y = m*x+b;
                    if y > 0.0 && y < (size_y - 1.0) {
                        let diff = x / ((max(x1.abs(),x2.abs())-min(x1.abs(),x2.abs())) as f32) ;
                        let vert = vert1.scale(diff).add(&vert2.scale(1.0-diff));
                        count += PipelineSession::<'a, V, VS, RV, PS, BB, BP, MS, MB, MP>::render_point(vert, x_val, y as u32, &mut self.buff, &self.ps, &mut self.test_buff, &self.masker);
                    }
                }
            }
        }

        return count;
    }

    pub fn draw_tri(&mut self, tri: Tri<V>) -> u32 {

        //Run vertex shader
        let vert1 = self.vs.transform(tri.p1);
        let vert2 = self.vs.transform(tri.p2);
        let vert3 = self.vs.transform(tri.p3);
        
        //let p1_sign = 

        let mut p1 = vert1.get_position().truncate() * (1.0 /vert1.get_position().w.abs());
        let mut p2 = vert2.get_position().truncate() * (1.0 /vert2.get_position().w.abs());
        let mut p3 = vert3.get_position().truncate() * (1.0 /vert3.get_position().w.abs());

        //Behind camera culling
        if p1.z < 0.0 && p2.z < 0.0 && p3.z < 0.0 {
            return 0;
        }

        //Off screen culling
        if p1.x < -1.0 && p2.x < -1.0 && p3.x < -1.0 {
            return 0;
        }
        if p1.y < -1.0 && p2.y < -1.0 && p3.y < -1.0 {
            return 0;
        }
        if p1.x > 1.0 && p2.x > 1.0 && p3.x > 1.0 {
            return 0;
        }
        if p1.y > 1.0 && p2.y > 1.0 && p3.y > 1.0 {
            return 0;
        }

        let size_x = self.buff.get_width() as f32;
        let size_y = self.buff.get_height() as f32;


        let x1 = (((p1.x+1.0) * size_x /2.0) );
        let y1 = (((p1.y+1.0) * size_y /2.0) );
        let x2 = (((p2.x+1.0) * size_x /2.0) );
        let y2 = (((p2.y+1.0) * size_y /2.0) );
        let x3 = (((p3.x+1.0) * size_x /2.0) );
        let y3 = (((p3.y+1.0) * size_y /2.0) );

        // Deltas
        let dx12 = x1 - x2;
        let dx23 = x2 - x3;
        let dx31 = x3 - x1;

        let dy12 = y1 - y2;
        let dy23 = y2 - y3;
        let dy31 = y3 - y1;

        // Bounding rectangle
        let minx = cmp::max(cmp::min(cmp::min(x1 as i32, cmp::min(x2 as i32, x3 as i32)),self.buff.get_width() as i32),0) as i32; 
        let maxx = cmp::max(cmp::min(cmp::max(x1 as i32, cmp::max(x2 as i32, x3 as i32))+1,self.buff.get_width() as i32),0) as i32;
        let miny = cmp::max(cmp::min(cmp::min(y1 as i32, cmp::min(y2 as i32, y3 as i32)),self.buff.get_height() as i32),0) as i32;
        let maxy = cmp::max(cmp::min(cmp::max(y1 as i32, cmp::max(y2 as i32, y3 as i32))+1,self.buff.get_height() as i32),0) as i32;

        
        let c1 = dy12 * x1 - dx12 * y1;
        let c2 = dy23 * x2 - dx23 * y2;
        let c3 = dy31 * x3 - dx31 * y3;

        let mut cy1 = c1 + dx12 * miny as f32 - dy12 * minx as f32;
        let mut cy2 = c2 + dx23 * miny as f32 - dy23 * minx as f32;
        let mut cy3 = c3 + dx31 * miny as f32 - dy31 * minx as f32;

        let area_sq = x1*(y2-y3)+x2*(y3-y1)+x3*(y1-y2);

        //match backface_culling {
        //    BackfaceCulling::Back => {if area_sq <= 0 {return;}},
        //    BackfaceCulling::Front => {if area_sq >= 0 {return;}},
        //    BackfaceCulling::Both => {}
        //}
        let rotation = area_sq > 0.0;

        // Scan through bounding rectangle
        let mut count = 0;
        for y_int in miny..maxy {
            let y = y_int as f32;
            let mut cx1 = cy1;
            let mut cx2 = cy2;
            let mut cx3 = cy3;
            for x_int in minx..maxx {
                let x = x_int as f32;
                if rotation == false {
                    if cx1 >= 0.0 && cx2 >= 0.0 && cx3 >= 0.0 {
                        //is visible
                        //count +=1;
                        /*
                        let area_1_sq = x*(y2-y3)+x2*(y3-y)+x3*(y-y2);
                        let area_2_sq = x1*(y-y3)+x*(y3-y1)+x3*(y1-y);
                        let bary1 = area_1_sq as f32 / area_sq as f32;
                        let bary2 = area_2_sq as f32 / area_sq as f32;
                        let bary3 = 1.0 - bary1 - bary2;
                        //get lerped vert...
                        let vert = vert1.scale(bary1).add(&vert2.scale(bary2).add(&vert3.scale(bary3)));

                        count += PipelineSession::<'a, V, VS, RV, PS, BB, BP, MS, MB, MP>::render_point(vert, x as u32, y as u32, &mut self.buff, &self.ps, &mut self.test_buff, &self.masker);
                        */
                    }
                } else {
                    if cx1 <= 0.0 && cx2 <= 0.0 && cx3 <= 0.0 {
                        //is visible
                        //count +=1;
                        
                        let area_1_sq = x*(y2-y3)+x2*(y3-y)+x3*(y-y2);
                        let area_2_sq = x1*(y-y3)+x*(y3-y1)+x3*(y1-y);
                        let bary1 = area_1_sq as f32 / area_sq as f32;
                        let bary2 = area_2_sq as f32 / area_sq as f32;
                        let bary3 = 1.0 - bary1 - bary2;
                        //get lerped vert...
                        let vert = vert1.scale(bary1).add(&vert2.scale(bary2).add(&vert3.scale(bary3)));

                        count += PipelineSession::<'a, V, VS, RV, PS, BB, BP, MS, MB, MP>::render_point(vert, x as u32, y as u32, &mut self.buff, &self.ps, &mut self.test_buff, &self.masker);
                        
                    }
                }
                cx1 -= dy12;
                cx2 -= dy23;
                cx3 -= dy31;
            }
            cy1 += dx12;
            cy2 += dx23;
            cy3 += dx31;
        }
        return count;
    }
    
    fn render_point(vert: RV, x:u32, y:u32, buff: &mut BB, shader: &PS, test_buff: &mut MB, masker: &MS) -> u32 {
        {
            let vert_pos = vert.get_position() ;//.truncate() * (1.0 /vert.get_position().w.abs());
            if vert_pos.z < 0.0 {
                return 0;
            }
        }
        let test = masker.test(&vert, test_buff.get_pixel(x,y));
        match test {
            Some(val) => {
                test_buff.set_pixel(x,y,val);
                let p = shader.shade(vert);

                buff.set_pixel(x, y, p);
                return 1;
            },
            None => {},
        }
        return 0;
    }
}
