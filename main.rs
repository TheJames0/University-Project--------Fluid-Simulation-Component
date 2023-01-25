const SIM_SIZE: usize = 100;
extern crate image;
use image::{RgbImage, Rgb};
/*struct holds fluid characteristics */
struct Fluid
{
    //Declare current field values
    density:[f32; SIM_SIZE * SIM_SIZE],
    velocity_x:[f32; SIM_SIZE * SIM_SIZE],
    velocity_y:[f32; SIM_SIZE * SIM_SIZE],
    //Declare previous field values
    prev_density:[f32; SIM_SIZE * SIM_SIZE],
    prev_velocity_x:[f32; SIM_SIZE * SIM_SIZE],
    prev_velocity_y:[f32; SIM_SIZE * SIM_SIZE],
    
}
/*Function flattens a 2D index to a 1D index to access 1D array*/
fn getarrayindex(x:u32,y:u32) -> usize
{
    let size: u32 = SIM_SIZE as u32;
    let index: usize = ((y*size) + x) as usize;
    return index;
}
/*function returns a value from a fluid array*/
fn getarrayvalue(x:u32,y:u32,array:&mut[f32]) -> f32 {
    let size: u32 = SIM_SIZE as u32;
    let index: usize = getarrayindex(x, y);
    return  array[index];
}
/*Function reads 1D fluid array and converts value and position to 2D image format */
fn export_density(density_array:&mut[f32],iteration: u8)
{
    let size: u32 = SIM_SIZE as u32;
    //Declare Image
    let mut image: RgbImage = RgbImage::new(100, 100);
    
    for x in 0..size 
    {
        for y in 0..size
        {
            //Iterate and set pixel value based on
            let val: u8 = (getarrayvalue(x, y, density_array)*255.0) as u8;
            *image.get_pixel_mut(x, y) = image::Rgb([val,0,0]);
            
        }
    }
    //Set output path
    let path = format!("output{}.png",iteration);
    //Save image
    image.save(path).unwrap();
}
fn add_value(x:u32,y:u32,array:&mut[f32],value:f32)
{
    let size: u32 = SIM_SIZE as u32;
    let index: usize = getarrayindex(x, y);
    array[index] += value;

}
/*
This function sets the Dirichlet boundary conditions using indexes 1 to reflect values at horizontal boundaries, 2 to reflect values at vertical boundaries */
fn set_bnd(b:u8, curr_x:&mut[f32])
{
    let size: u32 = SIM_SIZE as u32;
    for i in 1..size-1
        {   
            //If boundary condition match then reflect left boundary
            if b==1
            {
                curr_x[getarrayindex(0  ,i)] = -curr_x[getarrayindex(1,i)];
            }
            else
            {
                curr_x[getarrayindex(0  ,i)] = curr_x[getarrayindex(1,i)];
            }
            //If boundary condition match then reflect right boundary
            if b==1
            {
                curr_x[getarrayindex(size - 1,i)] = -curr_x[getarrayindex(size-2,i)];
            }
            else
            {
                curr_x[getarrayindex(size - 1,i)] = curr_x[getarrayindex(size-2,i)];
            }
            //If boundary condition match then reflect bottom boundary
            if b==2
            {
                curr_x[getarrayindex(i,0  )] = -curr_x[getarrayindex(i,1)];
            }
            else
            {
                curr_x[getarrayindex(i,0  )] = curr_x[getarrayindex(i,1)];
            }
            //If boundary condition match then reflect top boundary
            if b==2
            {
                curr_x[getarrayindex(i,size - 1)] = -curr_x[getarrayindex(i,size-2)];
            }
            else
            {
                curr_x[getarrayindex(i,size - 1)] = curr_x[getarrayindex(i,size-2)];
            }
        }
        //Reflect corner grid cells by averaged edge boundary
        //Top left
        curr_x[getarrayindex(0, size-1)] = 0.5 * curr_x[getarrayindex(1, size-1)] + curr_x[getarrayindex(0, size-2)];
        //Top right
        curr_x[getarrayindex(size - 1, size-1)] = 0.5 * curr_x[getarrayindex(size-2, size-1)] + curr_x[getarrayindex(size-1, size-2)];
        //Bottom left
        curr_x[getarrayindex(0, 0)] = 0.5 * curr_x[getarrayindex(1, 0)] + curr_x[getarrayindex(0, 1)];
        //Bottom right
        curr_x[getarrayindex(size-1, 0)] = 0.5 * curr_x[getarrayindex(size-2, 0)] + curr_x[getarrayindex(size-1, 1)];

}
/*Linear solver this function implements the Gauss Seidel method which solves linear system equations , param b is index for simulation edge bound,
x_curr is the current fluid array,
x_prev is the previous fluid array,
a and c are both differential constants,
iter param is the amount of iterations to be performed*/
fn linear_solve(b:u8 ,x_curr:&mut[f32] ,x_prev:&[f32] ,a:f32 ,c:f32 ,iter:u8 )
{
    let size: u32 = SIM_SIZE as u32;
    let crecip = 1.0 / c;
    for i in 0..iter
    {
        for x in 1..size-1
        {
            for y in 1..size-1
            {
                x_curr[getarrayindex(x, y)] = (x_prev[getarrayindex(x, y)]
                + a*(x_curr[getarrayindex(x + 1, y)]
                + x_curr[getarrayindex(x - 1, y)] 
                + x_curr[getarrayindex(x, y + 1)]
                + x_curr[getarrayindex(x, y - 1)])) * crecip;
            }
        }
        set_bnd(b, x_curr);
    }
}
/*This fluid feature models the dampening motion of the fluids or the loss in momentum overtime */
fn diffuse(b:u8,x_curr:&mut[f32])
{

}
/* This fluid feature ensures fluid incompressibility by running 'boxes' through each grid cell and ensuring the net outflow is the same as the net inflow within each 'box'*/
fn project()
{

}
/*This fluid feature models the force exerted on a 'particle' by surrounding 'particles' */
fn advect()
{

}
fn main()
{
    
    //Init mutable frame count int
    let mut frame_count = 0;
    //Init Fluid Array values
    let mut _fluid = Fluid
    {
        //Initialize current field values
        density:[0.0; SIM_SIZE*SIM_SIZE],
        velocity_x:[0.0; SIM_SIZE*SIM_SIZE],
        velocity_y:[0.0; SIM_SIZE * SIM_SIZE],
        //Initialize previous field values
        prev_density:[0.0; SIM_SIZE * SIM_SIZE],
        prev_velocity_x:[0.0; SIM_SIZE * SIM_SIZE],
        prev_velocity_y:[0.0; SIM_SIZE * SIM_SIZE],
    };
    
    //Main Iteration Loop
    while frame_count < 4 
    {
        //Add density
        add_value(50, 50, &mut _fluid.density,0.1);
        //Add velocity
        add_value(50, 50, &mut _fluid.velocity_y,0.1);

        //Fluid steps
        diffuse(1,_fluid.prev_velocity_x,_fluid.velocity_x,visc,dt,4);
        diffuse(2,_fluid.prev_velocity_y,_fluid.velocity_y,visc,dt,4);

        project(_fluid.prev_velocity_x,_fluid.prev_velocity_y,_fluid.velocity_x,_fluid.velocity_y,4);

        advect(1,_fluid.velocity_x,_fluid.prev_velocity_x,_fluid.prev_velocity_x,_fluid.prev_velocity_y,_fluid.prev_velocity_y,dt);
        advect(2,_fluid.velocity_y,_fluid.prev_velocity_y,_fluid.prev_velocity_x,_fluid.prev_velocity_x,_fluid.prev_velocity_y,dt);

        project(_fluid.velocity_x,_fluid.velocity_y,_fluid.prev_velocity_x,_fluid.prev_velocity,4);
        //Export density
        export_density(&mut _fluid.density,frame_count);
        //Increment frame count
        frame_count = frame_count + 1;
    }

}