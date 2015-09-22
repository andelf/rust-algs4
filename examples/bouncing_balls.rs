extern crate algs4;
extern crate sdl2;
extern crate sdl2_gfx;
extern crate rand;

use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;
use sdl2_gfx::primitives::DrawRenderer;

use rand::{thread_rng, Rng};
use algs4::priority_queues::event_driven_simulation::{Particle, CollisionSystem};


const BALLS: usize = 100;

fn run() -> Result<(), String> {
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();
    let mut timer = sdl_context.timer().unwrap();

    let mut rng = thread_rng();

    let win = video.window("bouncing balls", 800, 800)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut ren = win.renderer().build().unwrap();

    ren.set_draw_color(Color::RGB(255, 255, 255));
    ren.clear();
    ren.present();

    let mut running = true;

    let mut balls: Vec<Particle> = Vec::new();
    let mut colors: Vec<Color> = Vec::new();
    for _ in 0 .. BALLS {
        balls.push(rng.gen());
        colors.push(rng.gen());
    }
    let mut collision_sys = CollisionSystem::new(balls);

    collision_sys.init();
    let mut old_ps = collision_sys.particles.clone();

    collision_sys.tick();

    let mut last_world_time = (collision_sys.t * 1000.0) as u32;
    let mut last_update_ticks = 0;

    while running {
        for event in sdl_context.event_pump().unwrap().wait_timeout_iter(10) {
            match event {
                Event::Quit {..} => {
                    running = false
                },
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    match keycode {
                        Keycode::Escape | Keycode::Q => {
                            running = false;
                        },
                        _ => {}
                    }
                },
                _ => {
                }
            };
        }
        ren.set_draw_color(Color::RGBA(255, 255, 255, 255));
        let vp = ren.viewport();
        ren.fill_rect(vp);
        ren.clear();
        let width = vp.width();
        let height = vp.height();
        for (mut p, color) in old_ps.iter_mut().zip(colors.iter()) {
            p.do_move((timer.ticks() - last_update_ticks) as f64 / 1000.0);
            ren.filled_circle((p.rx * width as f64) as i16,
                              (p.ry * height as f64) as i16,
                              (p.radius * width as f64) as i16,
                              *color).unwrap();
            ren.aa_circle((p.rx * width as f64) as i16,
                          (p.ry * height as f64) as i16,
                          (p.radius * width as f64) as i16,
                          *color).unwrap();
            last_update_ticks = timer.ticks();
        }

        if timer.ticks() >= last_world_time {
            old_ps = collision_sys.particles.clone();
            let _ = collision_sys.tick();
            last_world_time = (collision_sys.t * 1000.0) as u32;
        }

        ren.present();
    }
    Ok(())
}


fn main() {
    run().unwrap();
}
