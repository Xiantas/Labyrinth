#![allow(non_snake_case)]

mod labyrinth;
mod generation;
mod utils;
mod navigation;
mod path_finding;
mod input_manager;

use labyrinth::Labyrinth;
use generation::from_point_generate;
use utils::Point;
use navigation::Ways;
use input_manager::InputSystem;

use sdl2::keyboard::Scancode;
use sdl2::event::Event;


fn main() -> Result<(), String> {
	let sdl_context = sdl2::init()?;

	let video_subsystem = sdl_context.video()?;

	let window = video_subsystem.window("Laby !!!", 1500, 1000)
		.position_centered()
		.opengl()
		.build()
		.map_err(|e| e.to_string())?;

	let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
	let mut event_pump = sdl_context.event_pump()?;

	let mut lastInstant = std::time::Instant::now();
	let mut frame_duration = lastInstant.elapsed();
	const FRAME_DURATION_TARGET: std::time::Duration = std::time::Duration::from_millis(25);

	let mut inputSystem = InputSystem::new(1, &vec![(Scancode::Return, 0)]);

	'run: loop {
		
		if lastInstant.elapsed() > frame_duration {
			frame_duration = lastInstant.elapsed() - frame_duration;
		} else {
			frame_duration = std::time::Duration::from_millis(0);
		}
		lastInstant = std::time::Instant::now();
		if frame_duration < FRAME_DURATION_TARGET {
			frame_duration = FRAME_DURATION_TARGET - frame_duration;
			std::thread::sleep(frame_duration);
		}

		for event in event_pump.poll_iter() {
			match event {
				Event::Quit {..} | Event::KeyDown { scancode: Some(Scancode::Escape), .. } => {
					break 'run
				},

				Event::KeyDown {scancode : Some(key), repeat : false, ..} => {
					inputSystem.updateDownKey(key);
				}

				Event::KeyUp {scancode : Some(key), repeat : false, ..} => {
					inputSystem.updateUpKey(key)
				},

				_ => {}
			}
		}
		inputSystem.refreshFields();

		if !inputSystem.fields[0].pressed {
			continue
		}

		canvas.set_draw_color((0,0,0));
		canvas.clear();

		let mut laby = Labyrinth::new(74, 49);

		from_point_generate(&mut laby, None);

		let ways = Ways::generate(&laby, None);
		let path = path_finding::path_from_connected(&ways, Point(0,48), Point(73,0));

		laby.render(&mut canvas);
		path.render(&mut canvas);
		let _ = canvas.present();
	}

	Ok(())
}
