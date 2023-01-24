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
fn exportdensity()
{
    let size: u32 = SIM_SIZE as u32;
    let mut image: RgbImage = RgbImage::new(100, 100);
    
    for x in 0..size 
    {
        for y in 0..size
        {
            
            *image.get_pixel_mut(x, y) = image::Rgb([255,0,0]);
            
        }
    }
    // Save the buffer as "image.png"
    image.save("output.png").unwrap();
}
fn main()
{
    exportdensity();
    //Init mutable frame count int
    let mut frame_count = 0;
    //Init Fluid Array values
    let mut _fluid = Fluid
    {
        density:[0.0; SIM_SIZE*SIM_SIZE],
        velocity_x:[0.0; SIM_SIZE*SIM_SIZE],
        velocity_y:[0.0; SIM_SIZE * SIM_SIZE],
    };
    //Main Iteration Loop
    while frame_count < 100 
    {
        //Increment frame count
        frame_count = frame_count + 1;
    }

}