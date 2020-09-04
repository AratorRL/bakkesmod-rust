use std::rc::Rc;
use std::cell::RefCell;

use bakkesmod;
use bakkesmod::prelude::*;

#[plugin_init]
pub fn on_load() {
    bakkesmod::register_notifier("rust_demolish", Box::new(move |_: Vec<String>| {
        match bakkesmod::get_local_car() {
            Some(car) => car.demolish(),
            None => log_console!("Car is NULL")
        };
    }));

    bakkesmod::register_notifier("rust_notifier", Box::new(normal_function_callback));

    bakkesmod::register_notifier("rust_set_loc", Box::new(move |_: Vec<String>| {
        match bakkesmod::get_local_car() {
            Some(car) => {
                let origin = vec3!(0.0, 0.0, 0.0);
                let new_loc = origin + vec3!(200.0, 1000.0, 50.0);
                car.set_location(new_loc);
            }
            None => log_console!("Car is NULL")
        };
    }));

    let cvar = bakkesmod::register_cvar("rust_cvar");
    log_console!("cvar name = {}", cvar.get_name());

    bakkesmod::register_notifier("rust_set_gravity", Box::new(move |params: Vec<String>| {
        if params.len() < 2 {
            log_console!("not enough parameters!");
            return;
        }

        let new_value_str = &params[1];
        let new_value: f32 = match new_value_str.parse::<f32>() {
            Ok(val) => val,
            Err(_) => { log_console!("invalid input!"); return; }
        };

        match bakkesmod::get_cvar("sv_soccar_gravity") {
            Some(cvar) => {
                log_console!("current value: {}", cvar.get_float_value());
                log_console!("setting to {}", new_value);
                cvar.set_float_value(new_value);
            }
            None => log_console!("cvar 'sv_soccar_gravity' not found")
        };
    }));

    bakkesmod::add_on_value_changed(&cvar, Box::new(move |_: String, cvar: CVar| {
        log_console!("cvar {} has a new value: {}", cvar.get_name(), cvar.get_string_value());
    }));

    bakkesmod::register_cvar("rust_ticker");

    let counter_base = Rc::new(RefCell::new(0));
    let counter_ref = Rc::clone(&counter_base);

    bakkesmod::hook_event("Function Engine.GameViewportClient.Tick", Box::new(move || {
        let ticker = bakkesmod::get_cvar("rust_ticker").unwrap();
        if !ticker.get_bool_value() {
            return;
        }

        let mut counter = counter_ref.borrow_mut();
        *counter += 1;
        if (*counter % 240) == 0 {
            // log_console!("viewport client tick");

            match bakkesmod::get_local_car() {
                Some(car) => {
                    let location = car.get_location();
                    log_console!("{}", location);

                    let vehicle_sim = car.get_vehicle_sim().unwrap();
                    let wheels = vehicle_sim.get_wheels();
                    let wheel0 = wheels.get(0);
                    log_console!("wheel 0 spin speed: {}", wheel0.get_spin_speed());
                    let wheel3 = wheels.get(3);
                    log_console!("wheel 3 spin speed: {}", wheel3.get_spin_speed());
                }
                None => log_console!("Car is NULL")
            };
        }
    }));

    bakkesmod::register_drawable(Box::new(move |canvas: Canvas| {
        let top_left = vec2!(100, 100);
        let width = vec2!(250, 0);
        let height = vec2!(0, 150);
        canvas.draw_line(top_left, top_left + width);
        canvas.draw_line(top_left + width, top_left + width + height);
        canvas.draw_line(top_left, top_left + height);
        canvas.draw_line(top_left + height, top_left + width + height);
    }));

    bakkesmod::hook_event_with_caller_post(
        "Function TAGame.Car_TA.ApplyBallImpactForces",
        Box::new(move |car: Box<CarWrapper>| {
            log_console!("Ball hit!");
            log_console!("car location: {}", car.get_location());
        })
    );

    bakkesmod::register_notifier("rust_set_timer", Box::new(move |params: Vec<String>| {
        if params.len() < 2 {
            log_console!("not enough parameters!");
        } else {
            let time_param = &params[1];
            let time: f32 = match time_param.parse::<f32>() {
                Ok(secs) => secs,
                Err(_) => { log_console!("invalid input!"); return; }
            };
            bakkesmod::set_timeout(Box::new(move || {
                log_console!("{} secs have passed!", time);
            }), time);
        }
    }));

    bakkesmod::register_notifier("rust_get_ball_info", Box::new(move |_: Vec<String>| {
        let game = match bakkesmod::get_game_event_as_server() {
            Some(g) => g,
            None => {
                log_console!("game is null!");
                return;
            }
        };
        
        match game.get_ball() {
            Some(ball) => log_console!("{}", ball.get_location()),
            None => log_console!("ball is NULL")
        };

    }));

    bakkesmod::register_notifier("rust_spawn_car", Box::new(spawn_car_callback));
}

fn normal_function_callback(params: Vec<String>) {
    log_console!("this is the callback for rust_notifier!");
    log_console!("params = {:x?}", params);
}

fn spawn_car_callback(_: Vec<String>) {
    let game = match bakkesmod::get_game_event_as_server() {
        Some(g) => g,
        None => {
            log_console!("game is null!");
            return;
        }
    };
        
    game.spawn_bot(22, "Bors");
}