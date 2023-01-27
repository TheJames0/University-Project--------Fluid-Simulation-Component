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
/* Adds fluid value to fluid field ie, density or velocity
 */
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
/*This fluid feature models the dampening motion of the fluids or the loss in momentum overtime aswell as dispersion of density*/
fn diffuse(b:u8,x_curr:&mut[f32],x_prev:&mut[f32],diff:f32,dt:f32,iter:u8)
{
    let size: u32 = SIM_SIZE as u32;
    let a:f32 = dt * diff * ((size-2) * (size-2))as f32;
    let c:f32 = (6.0 * a) + 1.0;
    linear_solve(b, x_curr, x_prev, a, c, iter);
}
/* This fluid feature ensures fluid incompressibility by running 'boxes' through each grid cell and ensuring the net outflow is the same as the net inflow within each 'box'*/
fn project(vel_x:&mut[f32],vel_y:&mut[f32],p:&mut[f32],div:&mut[f32],iter:u8)
{
    let size: u32 = SIM_SIZE as u32;
    for y in 1..size-1
    {
        for x in 1..size-1
        {
            div[getarrayindex(x, y)] = -0.5*(vel_x[getarrayindex(x+1,y)]-vel_x[getarrayindex(x-1,y)]+vel_y[getarrayindex(x,y+1)]-vel_y[getarrayindex(x,y-1)])/size as f32;
            p[getarrayindex(x, y)] = 0.0;
        }
    }
    set_bnd(0, div);
    set_bnd(0, p);
    linear_solve(0, p, div, 1.0, 6.0, iter);

    for y in 1..size-1
    {
        for x in 1..size-1
        {
            vel_x[getarrayindex(x, y)] -= 0.5 * (p[getarrayindex(x + 1, y)]-p[getarrayindex(x - 1, y)]) * size as f32;
            vel_y[getarrayindex(x, y)] -= 0.5 * (p[getarrayindex(x, y + 1)]-p[getarrayindex(x, y - 1)]) * size as f32;
        }
    }
    set_bnd(1, vel_x);
    set_bnd(2, vel_y);
}
/*This fluid feature models the force exerted on a 'particle' by surrounding 'particles' */
fn advect(b:u8,d:&mut[f32],d0:&mut[f32],vel_x:&[f32],vel_y:&[f32],dt:f32)
{
    let size: u32 = SIM_SIZE as u32;
    let float_size:f32 = size as f32;
    let mut i0:f32;
    let mut i1:f32;
    let mut j0:f32;
    let mut j1:f32;

    let dtx:f32 = dt* (float_size - 2.0);
    let dty:f32 = dt* (float_size - 2.0);

    let mut s0:f32;
    let mut s1:f32;
    let mut t0:f32;
    let mut t1:f32;

    let mut tmp1:f32;
    let mut tmp2:f32;

    let mut float_x:f32 = 0.0;
    let mut float_y:f32 = 0.0;
    let mut i:f32;
    let mut j:f32;


    for y in 1..size-1
    {
        for x in 1..size-1
        {
            tmp1 = dtx * vel_x[getarrayindex(x, y)];
            tmp2 = dty * vel_y[getarrayindex(x, y)];

            i = float_x - tmp1;
            j = float_y - tmp2;

            if i < 0.5 { i = 0.5;}
            if i > float_size + 0.5 {i = float_size + 0.5;}
            i0 = i.floor();
            i1 = i0 + 1.0;
            if j < 0.5 {j = 0.5;}
            if j > float_size + 0.5 {j = float_size + 0.5;}
            j0 = j.floor();
            j1 = j0 + 1.0;

            s1 = i - i0;
            s0 = 1.0 - s1;
            t1 = j - j0;
            t0 = 1.0 - t1;

            let int_i0:u32 = i0 as u32;
            let int_i1:u32 = i1 as u32;
            let int_j0:u32 = j0 as u32;
            let int_j1:u32 = j1 as u32;

            d[getarrayindex(x, y)] = s0 * (t0 * d0[getarrayindex(int_i0, int_j0)])
                                        + (t1 * d0[getarrayindex(int_i0, int_j1)])
                                    + s1 * (t0 * d0[getarrayindex(int_i1, int_j0)])
                                        +(t1 * d0[getarrayindex(int_i1, int_j1)]);

            float_x += 1.0;
        }
        float_y += 1.0;
    }
    set_bnd(b, d);
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
    //Set Fluid Constants
    let visc = 0.000001;
    let iter:u8 = 8;
    let dt = 0.00001;
    let diff:f32 = 0.0000001;
    //Main Iteration Loop
    while frame_count < 15 
    {
        //Add density
        add_value(50, 50, &mut _fluid.density,10000.0);
        //Add velocity
        add_value(50, 50, &mut _fluid.velocity_y,0.00000000001);

        //Fluid steps
        diffuse(1,&mut _fluid.prev_velocity_x,&mut _fluid.velocity_x,visc,dt,iter);
        diffuse(2,&mut _fluid.prev_velocity_y,&mut _fluid.velocity_y,visc,dt,iter);

        project(&mut _fluid.prev_velocity_x,&mut _fluid.prev_velocity_y,&mut _fluid.velocity_x,&mut _fluid.velocity_y,iter);

        //Temp Prev Vel Arrays
        let tempx:[f32; SIM_SIZE*SIM_SIZE] = _fluid.prev_velocity_x;
        let tempy:[f32; SIM_SIZE*SIM_SIZE] = _fluid.prev_velocity_y;
        
        advect(1,&mut _fluid.velocity_x,&mut _fluid.prev_velocity_x,&tempx,&tempy,dt);
        advect(2,&mut _fluid.velocity_y,&mut _fluid.prev_velocity_y,&tempx,&tempy,dt);

        project(&mut _fluid.velocity_x,&mut _fluid.velocity_y,&mut _fluid.prev_velocity_x,&mut _fluid.prev_velocity_y,iter);
        
        diffuse(0,&mut _fluid.prev_density, &mut _fluid.density, diff, dt, iter);
        advect(0, &mut _fluid.density, &mut _fluid.prev_density, &_fluid.velocity_x, &_fluid.velocity_y, dt);
        //Export density
        export_density(&mut _fluid.density,frame_count);
        //Increment frame count
        frame_count = frame_count + 1;
    }

}