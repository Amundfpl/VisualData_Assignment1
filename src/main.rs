// Uncomment these following global attributes to silence most warnings of "low" interest:
/*
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unreachable_code)]
#![allow(unused_mut)]
#![allow(unused_unsafe)]
#![allow(unused_variables)]
*/
extern crate nalgebra_glm as glm;
use core::f64;
use std::{ mem, ptr, os::raw::c_void };
use std::{f32, thread};
use std::sync::{Mutex, Arc, RwLock};

mod shader;
mod util;
mod mesh; 
mod scene_graph;
mod toolbox;

use scene_graph::SceneNode;
use gl::{BindVertexArray, BufferData, TIME_ELAPSED, UniformMatrix4fv};
use glm::{Mat4, dot, identity, length, vec3};
use glutin::event::{Event, WindowEvent, DeviceEvent, KeyboardInput, ElementState::{Pressed, Released}, VirtualKeyCode::{self, *}};
use glutin::event_loop::ControlFlow;

use crate::mesh::Helicopter;
use crate::toolbox::{Heading, simple_heading_animation};

// initial window size
const INITIAL_SCREEN_W: u32 = 800;
const INITIAL_SCREEN_H: u32 = 600;

// == // Helper functions to make interacting with OpenGL a little bit prettier. You *WILL* need these! // == //

// Get the size of an arbitrary array of numbers measured in bytes
// Example usage:  byte_size_of_array(my_array)
fn byte_size_of_array<T>(val: &[T]) -> isize {
    std::mem::size_of_val(&val[..]) as isize
}

// Get the OpenGL-compatible pointer to an arbitrary array of numbers
// Example usage:  pointer_to_array(my_array)
fn pointer_to_array<T>(val: &[T]) -> *const c_void {
    &val[0] as *const T as *const c_void
}

// Get the size of the given type in bytes
// Example usage:  size_of::<u64>()
fn size_of<T>() -> i32 {
    mem::size_of::<T>() as i32
}

// Get an offset in bytes for n units of type T, represented as a relative pointer
// Example usage:  offset::<u64>(4)
fn offset<T>(n: u32) -> *const c_void {
    (n * mem::size_of::<T>() as u32) as *const T as *const c_void
}

fn colorchange(program_id: u32) -> i32 {

    //terminated Cstring
    let time_name = std::ffi::CString::new("Time").unwrap();

    let time_location = unsafe {
        gl::GetUniformLocation(
            program_id,
            time_name.as_ptr()
        )
    };

    time_location
}

// Get a null pointer (equivalent to an offset of 0)
// ptr::null()

// == // Generate your VAO here
unsafe fn create_vao(vertices: &Vec<f32>, indices: &Vec<u32>, rgba: &Vec<f32>, normalVectors: &Vec<f32>) -> u32 {
    let mut vao: u32 = 0;
    let mut vbo: u32 = 0;
    let mut c_vbo: u32 = 0;
    let mut  index_buffer: u32 = 0;
    let mut normalV: u32 = 0;


     // * Generate a VAO and bind it
    gl::GenVertexArrays(1 ,&mut vao);
    gl::BindVertexArray(vao);


    // * Generate a VBO and bind it
    gl::GenBuffers(1,  &mut vbo);
    gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

    // * Fill it with data
    gl::BufferData(gl::ARRAY_BUFFER, 
        byte_size_of_array(vertices),
        pointer_to_array(vertices),
         gl::STATIC_DRAW);
    
    // * Configure a VAP for the data and enable it
    //VBO vertices
    gl::VertexAttribPointer(2, 3, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
    gl::EnableVertexAttribArray(2);
    
      // color VBO
    gl::GenBuffers(1, &mut c_vbo);
    gl::BindBuffer(gl::ARRAY_BUFFER, c_vbo);

    gl::BufferData(gl::ARRAY_BUFFER, 
        byte_size_of_array(rgba), 
        pointer_to_array(rgba), 
        gl::STATIC_DRAW
    );

    //c_vbo pointer. 
    gl::VertexAttribPointer(3, 4, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
    gl::EnableVertexAttribArray(3);


    //Normal vector Vertex buffer object filled. 
    gl::GenBuffers(1, &mut normalV);
    gl::BindBuffer(gl::ARRAY_BUFFER, normalV);

    gl::BufferData(gl::ARRAY_BUFFER, byte_size_of_array(normalVectors), pointer_to_array(normalVectors), gl::STATIC_DRAW);

    gl::VertexAttribPointer(5, 3, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
    gl::EnableVertexAttribArray(5);



    // * Generate a IBO and bind it
    gl::GenBuffers(1,  &mut index_buffer);
    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, index_buffer);

    // * Fill it with data
    gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
         byte_size_of_array(indices), 
         pointer_to_array(indices),
          gl::STATIC_DRAW
        );

    // This should:
    // * Generate a VAO and bind it
    // * Generate a VBO and bind it
    // * Fill it with data
    // * Configure a VAP for the data and enable it
    // * Generate a IBO and bind it
    // * Fill it with data
    // * Return the ID of the VAO
    return vao;
}



unsafe fn draw_scene(node: &scene_graph::SceneNode, 
    view_projection_matrix: &glm::Mat4,
    transformation_so_far: &glm::Mat4,
    matrix_location: i32){
        let to_origin = glm::translation(&(-node.reference_point));
        let rotation_x = glm::rotation(node.rotation.x, &glm::vec3(1.0,0.0,0.0));
        let rotation_y = glm::rotation(node.rotation.y, &glm::vec3(0.0,1.0,0.0));
        let rotation_z = glm::rotation(node.rotation.z, &glm::vec3(0.0,0.0,1.0));
        let back_from_origin = glm::translation(&(node.reference_point));
        let position = glm::translation(&node.position);
        let relative_transformation = position * back_from_origin * rotation_z * rotation_y * rotation_x * to_origin;
        let current_transformation = transformation_so_far * relative_transformation;
        let mvp = view_projection_matrix * current_transformation;
        
        if node.vao_id != 0{
            gl::UniformMatrix4fv(matrix_location, 1, gl::FALSE, mvp.as_ptr());
            gl::UniformMatrix4fv(7, 1, gl::FALSE, current_transformation.as_ptr());
            gl::BindVertexArray(node.vao_id);
            gl::DrawElements(gl::TRIANGLES, node.index_count, gl::UNSIGNED_INT, std::ptr::null());
        } 
            

    for &child in &node.children{
        draw_scene(&*child, view_projection_matrix, &current_transformation, matrix_location);
    }
}

fn main() {
    // Set up the necessary objects to deal with windows and event handling
    let el = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_title("Gloom-rs")
        .with_resizable(true)
        .with_inner_size(glutin::dpi::LogicalSize::new(INITIAL_SCREEN_W, INITIAL_SCREEN_H));
    let cb = glutin::ContextBuilder::new()
        .with_vsync(true);
    let windowed_context = cb.build_windowed(wb, &el).unwrap();
    // Uncomment these if you want to use the mouse for controls, but want it to be confined to the screen and/or invisible.
    // windowed_context.window().set_cursor_grab(true).expect("failed to grab cursor");
    // windowed_context.window().set_cursor_visible(false);

    // Set up a shared vector for keeping track of currently pressed keys
    let arc_pressed_keys = Arc::new(Mutex::new(Vec::<VirtualKeyCode>::with_capacity(10)));
    // Make a reference of this vector to send to the render thread
    let pressed_keys = Arc::clone(&arc_pressed_keys);

    // Set up shared tuple for tracking mouse movement between frames
    let arc_mouse_delta = Arc::new(Mutex::new((0f32, 0f32)));
    // Make a reference of this tuple to send to the render thread
    let mouse_delta = Arc::clone(&arc_mouse_delta);

    // Set up shared tuple for tracking changes to the window size
    let arc_window_size = Arc::new(Mutex::new((INITIAL_SCREEN_W, INITIAL_SCREEN_H, false)));
    // Make a reference of this tuple to send to the render thread
    let window_size = Arc::clone(&arc_window_size);

    // Spawn a separate thread for rendering, so event handling doesn't block rendering
    let render_thread = thread::spawn(move || {
        // Acquire the OpenGL Context and load the function pointers.
        // This has to be done inside of the rendering thread, because
        // an active OpenGL context cannot safely traverse a thread boundary
        let context = unsafe {
            let c = windowed_context.make_current().unwrap();
            gl::load_with(|symbol| c.get_proc_address(symbol) as *const _);
            c
        };

        let mut window_aspect_ratio = INITIAL_SCREEN_W as f32 / INITIAL_SCREEN_H as f32;

        // Set up openGL
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LESS);
            gl::Enable(gl::CULL_FACE);
            gl::Disable(gl::MULTISAMPLE);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
            gl::DebugMessageCallback(Some(util::debug_callback), ptr::null());

            // Print some diagnostics
            println!("{}: {}", util::get_gl_string(gl::VENDOR), util::get_gl_string(gl::RENDERER));
            println!("OpenGL\t: {}", util::get_gl_string(gl::VERSION));
            println!("GLSL\t: {}", util::get_gl_string(gl::SHADING_LANGUAGE_VERSION));
        }
       
        let terrain = mesh::Terrain::load("./resources/lunarsurface.obj");
        let terrain_vao = unsafe{ create_vao(&terrain.vertices, &terrain.indices, &terrain.colors, &terrain.normals)};

        let helicpoter = mesh::Helicopter::load("resources/helicopter.obj");

        let helicopter_body = helicpoter.body;
        let helicopter_door = helicpoter.door;
        let helicopter_main_rotor = helicpoter.main_rotor;
        let helicopter_tail_rotor = helicpoter.tail_rotor;

        let helicopter_body_vao = unsafe {
            create_vao(&helicopter_body.vertices, &helicopter_body.indices, &helicopter_body.colors, &helicopter_body.normals)
        };
        let helicopter_door_vao = unsafe {
            create_vao(&helicopter_door.vertices, &helicopter_door.indices, &helicopter_door.colors, &helicopter_door.normals)
        };
        let helicopter_main_rotor_vao = unsafe {
            create_vao(&helicopter_main_rotor.vertices, &helicopter_main_rotor.indices, &helicopter_main_rotor.colors, &helicopter_main_rotor.normals)
        };
        let helicopter_tail_rotor_vao = unsafe {
            create_vao(&helicopter_tail_rotor.vertices, &helicopter_tail_rotor.indices, &helicopter_tail_rotor.colors, &helicopter_tail_rotor.normals)
        };


        // original
        let mut root_node = scene_graph::SceneNode::new();
        let mut terrain_scene_node = scene_graph::SceneNode::from_vao(terrain_vao, terrain.index_count);
        //let mut root_helicopter = scene_graph::SceneNode::new();
        let mut h_body_scene_node = scene_graph::SceneNode::from_vao(helicopter_body_vao, helicopter_body.index_count); 
        let mut h_door_scene_node = scene_graph::SceneNode::from_vao(helicopter_door_vao, helicopter_door.index_count);
        let mut h_main_rotor_scene_node = scene_graph::SceneNode::from_vao(helicopter_main_rotor_vao, helicopter_main_rotor.index_count);  
        let mut h_tail_rotor_scene_node = scene_graph::SceneNode::from_vao(helicopter_tail_rotor_vao, helicopter_tail_rotor.index_count); 
        
        //let mut testhelicopter = scene_graph::SceneNode::new();
        let mut helicopters = Vec::new();
        for _ in 0..5 {
            helicopters.push(scene_graph::SceneNode::new());
        }

        //original
        /* 
        root_helicopter.add_child(&h_body_scene_node);
        root_helicopter.add_child(&h_door_scene_node);
        root_helicopter.add_child(&h_main_rotor_scene_node);
        root_helicopter.add_child(&h_tail_rotor_scene_node);
        terrain_scene_node.add_child(&root_helicopter);
        testhelicopter.add_child(&h_body_scene_node);
        testhelicopter.add_child(&h_door_scene_node);
        testhelicopter.add_child(&h_main_rotor_scene_node);
        testhelicopter.add_child(&h_tail_rotor_scene_node);
        terrain_scene_node.add_child(&testhelicopter);
    */
        

        
        for i in 0..helicopters.len(){
            helicopters[i].add_child(&h_body_scene_node);
            helicopters[i].add_child(&h_door_scene_node);
            helicopters[i].add_child(&h_main_rotor_scene_node);
            helicopters[i].add_child(&h_tail_rotor_scene_node);
            terrain_scene_node.add_child(&helicopters[i]);
        }
        root_node.add_child(&terrain_scene_node);
        //root_node.print();
        //root_helicopter.print();
        //terrain_scene_node.print();

        //set rotation points:
        h_tail_rotor_scene_node.reference_point = glm::vec3(0.35, 2.3, 10.4);
        h_main_rotor_scene_node.reference_point = glm::vec3(0.0, 3.0, 0.0);
        h_door_scene_node.reference_point = glm::vec3(1.0, 0.0, 1.0);
        h_body_scene_node.reference_point = glm::vec3(0.0, 0.0, 0.0);


        let mut camera = glm::Vec3::new(1.0,0.0,1.0);
        let mut camera_angle = glm::vec2(0.0, 0.0);
        // == // Set up your shaders here

        // Basic usage of shader helper:
        // The example code below creates a 'shader' object.
        // It which contains the field `.program_id` and the method `.activate()`.
        // The `.` in the path is relative to `Cargo.toml`.
        // This snippet is not enough to do the exercise, and will need to be modified (outside
        // of just using the correct path), but it only needs to be called once

        
        let simple_shader = unsafe {
            shader::ShaderBuilder::new()
                .attach_file("./shaders/simple.frag").attach_file("./shaders/simple.vert")
                .link()
        };

        //optional a.)
        let time_location = colorchange(simple_shader.program_id);
        
        //SetUp matrix transformation as uniform value:
        let transform_name = std::ffi::CString::new("transform").unwrap();
        let matrix_location = unsafe{ gl::GetUniformLocation(simple_shader.program_id, transform_name.as_ptr())
        };


        // The main rendering loop
        let first_frame_time = std::time::Instant::now();
        let mut previous_frame_time = first_frame_time;
        let mut door_open = false;
        let mut e_was_pressed = false;
        loop {
            // Compute time passed since the previous frame and since the start of the program
            let now = std::time::Instant::now();
            let elapsed = now.duration_since(first_frame_time).as_secs_f32();
            let delta_time = now.duration_since(previous_frame_time).as_secs_f32();
            previous_frame_time = now;

            // Handle resize events
            if let Ok(mut new_size) = window_size.lock() {
                if new_size.2 {
                    context.resize(glutin::dpi::PhysicalSize::new(new_size.0, new_size.1));
                    window_aspect_ratio = new_size.0 as f32 / new_size.1 as f32;
                    (*new_size).2 = false;
                    println!("Window was resized to {}x{}", new_size.0, new_size.1);
                    unsafe { gl::Viewport(0, 0, new_size.0 as i32, new_size.1 as i32); }
                }
            }

            // Handle mouse movement. delta contains the x and y movement of the mouse since last frame in pixels
            if let Ok(mut delta) = mouse_delta.lock() {

                // == // Optionally access the accumulated mouse movement between
                // == // frames here with `delta.0` and `delta.1`

                *delta = (0.0, 0.0); // reset when done
            }

            // == // Please compute camera transforms here (exercise 2 & 3)
        let mut movement = glm::vec4(0.0, 0.0, 0.0, 0.0);

        let movement_speed: f32 = 50.0;

        if let Ok(keys) = pressed_keys.lock() {
            let e_pressed = keys.contains(&VirtualKeyCode::E);
            for key in keys.iter() {
                match key {

                    VirtualKeyCode::W => {
                        movement.z -= 1.0*movement_speed;
                    }

                    VirtualKeyCode::S => {
                        movement.z += 1.0*movement_speed;
                    }

                    VirtualKeyCode::A => {
                        movement.x -= 1.0*movement_speed;
                    }

                    VirtualKeyCode::D => {
                        movement.x += 1.0*movement_speed;
                    }

                    VirtualKeyCode::Space => {
                        movement.y += 1.0*movement_speed;
                    }

                    VirtualKeyCode::LShift => {
                        movement.y -= 1.0*movement_speed;
                    }

                    VirtualKeyCode::Left => {
                        camera_angle.x += delta_time*2.0;
                    }

                    VirtualKeyCode::Right => {
                        camera_angle.x -= delta_time*2.0;
                    }

                    VirtualKeyCode::Up => {
                        camera_angle.y += delta_time*2.0;
                    }

                    VirtualKeyCode::Down => {
                        camera_angle.y -= delta_time*2.0;
                    }

                    _ => {}
                }
            }
            // Toggle door only once when E changes from released -> pressed
            if e_pressed && !e_was_pressed {
                door_open = !door_open;

                if door_open {
                    h_door_scene_node.position.z = 2.0;
                } else {
                    h_door_scene_node.position.z = 0.0;
                }
            }

            e_was_pressed = e_pressed;
        }

            let camera_horizontal_rotation: glm::Mat4 = glm::rotation(
                camera_angle.x,&glm::vec3(0.0, 1.0, 0.0));

            let camera_vertical_rotation: glm::Mat4 =
                glm::rotation(
                    camera_angle.y,&glm::vec3(1.0, 0.0, 0.0));

            let camera_rotation: glm::Mat4 = camera_horizontal_rotation * camera_vertical_rotation;

            let world_movement = camera_rotation * movement;

            camera.x += world_movement.x * delta_time;
            camera.y += world_movement.y * delta_time;
            camera.z += world_movement.z * delta_time;
            
            let identity_matrix : glm::Mat4 = glm::identity(); 

            let mut transform : glm::Mat4 = glm::identity();
            let trans: glm::Mat4 = glm::translation(&glm::vec3(-camera.x,-camera.y,-camera.z));
            let horizontal_rotation: glm::Mat4 = glm::rotation(-camera_angle.x, &glm::vec3(0.0, 1.0, 0.0));
            let vertical_rotation: glm::Mat4 = glm::rotation(-camera_angle.y, &glm::vec3(1.0, 0.0, 0.0));
            let project: glm::Mat4 = glm::perspective(window_aspect_ratio,45.0_f32.to_radians(),1.0,1000.0);
            transform = project*vertical_rotation*horizontal_rotation*trans*transform;

            //let billboard_position =  glm::translation(&glm::vec3(1.0, 0.0, 0.0));
            //let billboard_rotation: glm::Mat4 = camera_rotation;
            //let billboard_model = billboard_position * billboard_rotation;
            //let billboard_transform = project * vertical_rotation * horizontal_rotation * trans * billboard_model;

            unsafe {
                // Clear the color and depth buffers
                gl::ClearColor(0.035, 0.046, 0.078, 1.0); // night sky
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);


                // == // Issue the necessary gl:: commands to draw your scene here
                simple_shader.activate();

                // Task d: animated colour
                gl::Uniform1f(time_location, elapsed); //task d.)

                //let animation = toolbox::simple_heading_animation(elapsed);

                //original
                /*  
                root_helicopter.position.x = animation.x;
                root_helicopter.position.z = animation.z;
                root_helicopter.rotation.y = animation.yaw;
                root_helicopter.rotation.z = animation.roll;
                root_helicopter.rotation.x = animation.pitch;
                */
                //testhelicopter.rotation.y = 90.0;

                for i in 0..helicopters.len() {
                    let offset = i as f32 * 3.1;
                    let animation = toolbox::simple_heading_animation(elapsed + offset);
                    helicopters[i].position.x = animation.x;
                    helicopters[i].position.z = animation.z;

                    helicopters[i].rotation.y = animation.yaw;
                    helicopters[i].rotation.z = animation.roll;
                    helicopters[i].rotation.x = animation.pitch;
        }


                //apply for all helicopters since we use the same nodes:
                h_main_rotor_scene_node.rotation.y = elapsed * 10.0;
                h_tail_rotor_scene_node.rotation.x = elapsed * 10.0;
                
                
                //gl::UniformMatrix4fv(matrix_location, 1, gl::FALSE, transform.as_ptr());
                //println!("Camera value x = {}", camera.x);
                //println!("Camera value y = {}", camera.y);
                //println!("Camera value z = {}", camera.z);


                //gl::BindVertexArray(my_vao);
                //gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());


                //billboard drawing
                //gl::UniformMatrix4fv(matrix_location,1,gl::FALSE,billboard_transform.as_ptr());
                //gl::BindVertexArray(billboard_vao);
                //gl::DrawElements(gl::TRIANGLES,6,gl::UNSIGNED_INT,std::ptr::null());

                //now using scene graph to draw:
                /*
                

                gl::BindVertexArray(terrain_vao);
                gl::DrawElements(gl::TRIANGLES, terrain.index_count, gl::UNSIGNED_INT, std::ptr::null());
                //helicopter
                gl::BindVertexArray(helicopter_body_vao);
                gl::DrawElements(gl::TRIANGLES, helicopter_body.index_count, gl::UNSIGNED_INT, std::ptr::null());

                gl::BindVertexArray(helicopter_door_vao);
                gl::DrawElements(gl::TRIANGLES, helicopter_door.index_count, gl::UNSIGNED_INT, std::ptr::null());

                gl::BindVertexArray(helicopter_main_rotor_vao);
                gl::DrawElements(gl::TRIANGLES, helicopter_main_rotor.index_count, gl::UNSIGNED_INT, std::ptr::null());

                gl::BindVertexArray(helicopter_tail_rotor_vao);
                gl::DrawElements(gl::TRIANGLES, helicopter_tail_rotor.index_count, gl::UNSIGNED_INT, std::ptr::null());
                */
                draw_scene(&root_node, &transform,  &identity_matrix, matrix_location);
            }

            // Display the new color buffer on the display
            context.swap_buffers().unwrap(); // we use "double buffering" to avoid artifacts
        }
    });


    // == //
    // == // From here on down there are only internals.
    // == //


    // Keep track of the health of the rendering thread
    let render_thread_healthy = Arc::new(RwLock::new(true));
    let render_thread_watchdog = Arc::clone(&render_thread_healthy);
    thread::spawn(move || {
        if !render_thread.join().is_ok() {
            if let Ok(mut health) = render_thread_watchdog.write() {
                println!("Render thread panicked!");
                *health = false;
            }
        }
    });

    // Start the event loop -- This is where window events are initially handled
    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        // Terminate program if render thread panics
        if let Ok(health) = render_thread_healthy.read() {
            if *health == false {
                *control_flow = ControlFlow::Exit;
            }
        }

        match event {
            Event::WindowEvent { event: WindowEvent::Resized(physical_size), .. } => {
                println!("New window size received: {}x{}", physical_size.width, physical_size.height);
                if let Ok(mut new_size) = arc_window_size.lock() {
                    *new_size = (physical_size.width, physical_size.height, true);
                }
            }
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                *control_flow = ControlFlow::Exit;
            }
            // Keep track of currently pressed keys to send to the rendering thread
            Event::WindowEvent { event: WindowEvent::KeyboardInput {
                    input: KeyboardInput { state: key_state, virtual_keycode: Some(keycode), .. }, .. }, .. } => {

                if let Ok(mut keys) = arc_pressed_keys.lock() {
                    match key_state {
                        Released => {
                            if keys.contains(&keycode) {
                                let i = keys.iter().position(|&k| k == keycode).unwrap();
                                keys.remove(i);
                            }
                        },
                        Pressed => {
                            if !keys.contains(&keycode) {
                                keys.push(keycode);
                            }
                        }
                    }
                }

                // Handle Escape and Q keys separately
                match keycode {
                    Escape => { *control_flow = ControlFlow::Exit; }
                    Q      => { *control_flow = ControlFlow::Exit; }
                    _      => { }
                }
            }
            Event::DeviceEvent { event: DeviceEvent::MouseMotion { delta }, .. } => {
                // Accumulate mouse movement
                if let Ok(mut position) = arc_mouse_delta.lock() {
                    *position = (position.0 + delta.0 as f32, position.1 + delta.1 as f32);
                }
            }
            _ => { }
        }
    });
}
