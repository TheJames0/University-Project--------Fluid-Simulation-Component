const SIM_SIZE: usize = 100;
struct Fluid
{
    //Declare field values density
    density:[f32; SIM_SIZE * SIM_SIZE],
    velocity_x:[f32; SIM_SIZE * SIM_SIZE],
    velocity_y:[f32; SIM_SIZE * SIM_SIZE],
    
}
fn exportDensity()
{

}
fn main()
{
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