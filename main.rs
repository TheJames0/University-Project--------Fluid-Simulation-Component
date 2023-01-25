const SIM_SIZE: usize = 100;
extern crate image;
use image::{RgbImage, Rgb};
struct Fluid
{
    //Declare field values density
    density:[f32; SIM_SIZE * SIM_SIZE],
    velocity_x:[f32; SIM_SIZE * SIM_SIZE],
    velocity_y:[f32; SIM_SIZE * SIM_SIZE],
    
}
fn getArrayValue(x:u32,y:u32,array:&mut[f32]) -> f32 {
    let size: u32 = SIM_SIZE as u32;
    let index: usize = ((y*size) + x) as usize;
    return  array[index];
}
fn exportdensity(density_array:&mut[f32])
{
    let size: u32 = SIM_SIZE as u32;
    let mut image: RgbImage = RgbImage::new(100, 100);
    
    for x in 0..size 
    {
        for y in 0..size
        {
            let val: u8 = (getArrayValue(x, y, density_array)*255.0) as u8;
            *image.get_pixel_mut(x, y) = image::Rgb([val,0,0]);
            
        }
    }
    // Save the buffer as "image.png"
    image.save("output.png").unwrap();
}
fn main()
{
    
    //Init mutable frame count int
    let mut frame_count = 0;
    //Init Fluid Array values
    let mut _fluid = Fluid
    {
        density:[125.0; SIM_SIZE*SIM_SIZE],
        velocity_x:[0.0; SIM_SIZE*SIM_SIZE],
        velocity_y:[0.0; SIM_SIZE * SIM_SIZE],
    };
    exportdensity(&mut _fluid.density);
    //Main Iteration Loop
    while frame_count < 100 
    {
        //Export density
        
        //Increment frame count
        frame_count = frame_count + 1;
    }

}