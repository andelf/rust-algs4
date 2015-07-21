extern crate algs4;
extern crate sdl2;
extern crate sdl2_gfx;
extern crate rand;


use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;
use sdl2_gfx::primitives::DrawRenderer;

use rand::{thread_rng, Rng};
use algs4::priority_queues::event_driven_simulation::Particle;

fn run() -> Result<(), String> {
    let mut ctx = sdl2::init().video().unwrap();
    let mut rng = thread_rng();

    let win = ctx.window("bouncing balls", 800, 800)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut ren = win.renderer().build().unwrap();

    ren.set_draw_color(Color::RGB(255, 255, 255));
    ren.clear();
    ren.present();

    let mut running = true;

    let mut balls = Vec::new();
    for _ in 0 .. 200 {
        let color: Color = rng.gen();
        balls.push((Particle::new(), color));
    }

    while running {
        for event in ctx.event_pump().wait_timeout_iter(10) {
            match event {
                Event::Quit {..} => {
                    running = false
                },
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    match keycode {
                        Keycode::Escape | Keycode::Q => {
                            running = false;
                        },
                        Keycode::Space => {
                        },
                        _ => {}
                    }
                },
                _ => {
                }
            };
        }
        ren.set_draw_color(Color::RGBA(255, 255, 255, 255));
        //ren.filled_box()
        let vp = ren.get_viewport();
        ren.fill_rect(vp);
        ren.clear();
        let width = vp.width();
        let height = vp.height();
        for &mut (ref mut ball, color) in balls.iter_mut() {
            try!(ren.filled_circle((ball.rx * width as f64) as i16,
                                   (ball.ry * height as f64) as i16,
                                   (ball.radius * width as f64) as i16,
                                   color));
            try!(ren.aa_circle((ball.rx * width as f64) as i16,
                               (ball.ry * height as f64) as i16,
                               (ball.radius * width as f64) as i16,
                               color));
            ball.do_move(0.1);

        }
        ren.set_draw_color(Color::RGBA(255, 255, 255, 125));
        ren.present();
    }
    Ok(())
}


fn main() {
    run().unwrap();
}
