#![allow(non_upper_case_globals)]

extern crate piston_window;
use piston_window::*;

const size:Size = Size{width:800, height:600};

struct Vec2 {
	x: f64,
	y: f64,
}

struct Ball {
	pos: Vec2,
	speed: Vec2,
}

fn min(a: f64, b: f64) -> f64 {
	if b < a {
		return b;
	} else {
		return a;
	}
}

fn main() {
	let mut window:PistonWindow = WindowSettings::new("Test Rust Game", size)
		.exit_on_esc(true)
		.build()
		.unwrap();

	let mut ball = Ball{
		pos: Vec2{x:100.0, y:size.height as f64-100.0},
		speed: Vec2{x:1.0, y:1.0},
	};
	let ball_size = 20.0;
	let mut box_states = [true; 40];

	while let Some(e) = window.next() {
		window.draw_2d(&e, |c, g| {
			clear([0.0, 0.0, 0.0, 1.0], g);

			let color = [1.0, 1.0, 1.0, 1.0];
			let num_boxes = 10.0;
			let box_size:f64 = (size.width as f64 - ((num_boxes+1.0)*10.0)) / num_boxes;
			for i in 0..10 {
				for j in 0..4 {
					if box_states[j*10+i] {
						let f = i as f64;
						let f2 = j as f64;
						let x = 10.0 + f*(box_size+10.0);
						let y = 10.0 + f2*(box_size/2.0+10.0);
						let w = box_size;
						let h = box_size/2.0;
						rectangle(color, [x, y, w, h], c.transform, g);

						if ball.pos.x < x+w &&
						   ball.pos.x > x &&
						   ball.pos.y < y+h &&
						   ball.pos.y > y {
							box_states[j*10+i] = false;

							let xl = x+w - ball.pos.x;
							let xr = ball.pos.x - x;
							let yl = y+h - ball.pos.y;
							let yr = ball.pos.y - y;
							let xm = min(xl, xr);
							let ym = min(yl, yr);
							if xm < ym {
								if xm == xl {
									ball.speed.x = 1.0;
								} else {
									ball.speed.x = -1.0;
								}
							} else {
								if ym == yl {
									ball.speed.y = 1.0;
								} else {
									ball.speed.y = -1.0;
								}
							}
						}
					}
				}
			}

			ball.pos.x += ball.speed.x * 5.0;
			ball.pos.y += ball.speed.y * 5.0;
			if ball.pos.x < 0.0 { ball.speed.x = 1.0; }
			if ball.pos.x + ball_size > size.width as f64 { ball.speed.x = -1.0; }
			if ball.pos.y < 0.0 { ball.speed.y = 1.0; }
			if ball.pos.y + ball_size > size.height as f64 { ball.speed.y = -1.0; }
			rectangle(color, [ball.pos.x, ball.pos.y, 20.0, 20.0], c.transform, g);
		});
	}
}