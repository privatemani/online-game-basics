extern crate ggez;
use ggez::event::{self, Keycode, Mod};
use ggez::*;
mod client;

// Holy fucking shit I just wrote this and I don't know what it does
//TODO Make variable names not awful
struct MainState {
    pos_x: f32,
    pos_y: f32,
    main_sprite_sheet: graphics::Image,
    current_frame: u32,
    speed: f32,
    up: bool,
    down: bool,
    left: bool,
    right: bool,
    starting_background: graphics::Image,
    starting_background_x: f32,
    starting_background_y: f32,
    timer: u32,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let image = graphics::Image::new(_ctx, "/idle_right.png").unwrap();
        let s = MainState {
            pos_x: 960.0,
            pos_y: 540.0,
            main_sprite_sheet: image,
            current_frame: 0,
            speed: 5.0,
            up: false,
            down: false,
            left: false,
            right: false,
            starting_background: graphics::Image::new(_ctx, "/starting_background.png").unwrap(),
            starting_background_x: 0.0,
            starting_background_y: 0.0,
            timer: 0,
        };
        Ok(s)
    }
    fn walking_animation(&mut self, _ctx: &mut Context, frame_speed: u32, end_frame: u32, state: u32) -> GameResult<()> {
        let frame_time = 60.0 / frame_speed as f32;
        if state == 1 {
            if self.current_frame == 0 && self.timer as f32 > frame_time {
                self.main_sprite_sheet = graphics::Image::new(_ctx, "/forward_1.png").unwrap();
                self.current_frame = self.current_frame + 1;
                self.timer = 0;
            } else if self.current_frame == 1 && self.timer as f32 > frame_time  {
                self.current_frame = self.current_frame + 1;
                self.main_sprite_sheet = graphics::Image::new(_ctx, "/forward_2.png").unwrap();
                self.timer = 0;
            } else if self.current_frame == 2 && self.timer as f32 > frame_time  {
                self.current_frame = self.current_frame + 1;
                self.main_sprite_sheet = graphics::Image::new(_ctx, "/forward_3.png").unwrap();
                self.timer = 0;
            } else if self.current_frame == 3 && self.timer as f32 > frame_time  {
                self.current_frame = self.current_frame + 1;
                self.main_sprite_sheet = graphics::Image::new(_ctx, "/forward_4.png").unwrap();
                self.timer = 0;
            } else if self.current_frame == 4 && self.timer as f32 > frame_time  {
                self.current_frame = self.current_frame + 1;
                self.main_sprite_sheet = graphics::Image::new(_ctx, "/forward_5.png").unwrap();
                self.timer = 0;
            } else if self.current_frame == 5 && self.timer as f32 > frame_time  {
                self.current_frame = self.current_frame + 1;
                self.main_sprite_sheet = graphics::Image::new(_ctx, "/forward_6.png").unwrap();
                self.timer = 0;
            } else if self.current_frame == 6 && self.timer as f32 > frame_time  {
                self.current_frame = self.current_frame + 1;
                println!("Heyo");
                self.main_sprite_sheet = graphics::Image::new(_ctx, "/forward_7.png").unwrap();
                self.timer = 0;
            } else if self.current_frame == 7 && self.timer as f32 > frame_time  {
                self.current_frame = 0;
                self.main_sprite_sheet = graphics::Image::new(_ctx, "/forward_8.png").unwrap();
                self.timer = 0;
            } else {
                self.timer = self.timer + 1;
            }
        } else if state == 2 {
            if self.current_frame == 0 && self.timer as f32 > frame_time {
                self.main_sprite_sheet = graphics::Image::new(_ctx, "/right_1.png").unwrap();
                self.current_frame = self.current_frame + 1;
                self.timer = 0;
            } else if self.current_frame == 1 && self.timer as f32 > frame_time  {
                self.current_frame = self.current_frame + 1;
                self.main_sprite_sheet = graphics::Image::new(_ctx, "/right_2.png").unwrap();
                self.timer = 0;
            } else if self.current_frame == 2 && self.timer as f32 > frame_time  {
                self.current_frame = self.current_frame + 1;
                self.main_sprite_sheet = graphics::Image::new(_ctx, "/right_3.png").unwrap();
                self.timer = 0;
            } else if self.current_frame == 3 && self.timer as f32 > frame_time  {
                self.current_frame = self.current_frame + 1;
                self.main_sprite_sheet = graphics::Image::new(_ctx, "/right_4.png").unwrap();
                self.timer = 0;
            } else if self.current_frame == 4 && self.timer as f32 > frame_time  {
                self.current_frame = self.current_frame + 1;
                self.main_sprite_sheet = graphics::Image::new(_ctx, "/right_5.png").unwrap();
                self.timer = 0;
            } else if self.current_frame == 5 && self.timer as f32 > frame_time  {
                self.current_frame = self.current_frame + 1;
                self.main_sprite_sheet = graphics::Image::new(_ctx, "/right_6.png").unwrap();
                self.timer = 0;
            } else if self.current_frame == 6 && self.timer as f32 > frame_time  {
                self.current_frame = self.current_frame + 1;
                println!("Heyo");
                self.main_sprite_sheet = graphics::Image::new(_ctx, "/right_7.png").unwrap();
                self.timer = 0;
            } else if self.current_frame == 7 && self.timer as f32 > frame_time  {
                self.current_frame = 0;
                self.main_sprite_sheet = graphics::Image::new(_ctx, "/right_8.png").unwrap();
                self.timer = 0;
            } else {
                self.timer = self.timer + 1;
            }
        } else if state == 3 {
            if self.current_frame == 0 && self.timer as f32 > frame_time {
                self.main_sprite_sheet = graphics::Image::new(_ctx, "/back_1.png").unwrap();
                self.current_frame = self.current_frame + 1;
                self.timer = 0;
            } else if self.current_frame == 1 && self.timer as f32 > frame_time  {
                self.current_frame = self.current_frame + 1;
                self.main_sprite_sheet = graphics::Image::new(_ctx, "/back_2.png").unwrap();
                self.timer = 0;
            } else if self.current_frame == 2 && self.timer as f32 > frame_time  {
                self.current_frame = self.current_frame + 1;
                self.main_sprite_sheet = graphics::Image::new(_ctx, "/back_3.png").unwrap();
                self.timer = 0;
            } else if self.current_frame == 3 && self.timer as f32 > frame_time  {
                self.current_frame = self.current_frame + 1;
                self.main_sprite_sheet = graphics::Image::new(_ctx, "/back_4.png").unwrap();
                self.timer = 0;
            } else if self.current_frame == 4 && self.timer as f32 > frame_time  {
                self.current_frame = self.current_frame + 1;
                self.main_sprite_sheet = graphics::Image::new(_ctx, "/back_5.png").unwrap();
                self.timer = 0;
            } else if self.current_frame == 5 && self.timer as f32 > frame_time  {
                self.current_frame = self.current_frame + 1;
                self.main_sprite_sheet = graphics::Image::new(_ctx, "/back_6.png").unwrap();
                self.timer = 0;
            } else if self.current_frame == 6 && self.timer as f32 > frame_time  {
                self.current_frame = self.current_frame + 1;
                println!("Heyo");
                self.main_sprite_sheet = graphics::Image::new(_ctx, "/back_7.png").unwrap();
                self.timer = 0;
            } else if self.current_frame == 7 && self.timer as f32 > frame_time  {
                self.current_frame = 0;
                self.main_sprite_sheet = graphics::Image::new(_ctx, "/back_8.png").unwrap();
                self.timer = 0;
            } else {
                self.timer = self.timer + 1;
            }
        } else if state == 4 {
            if self.current_frame == 0 && self.timer as f32 > frame_time {
                self.main_sprite_sheet = graphics::Image::new(_ctx, "/left_1.png").unwrap();
                self.current_frame = self.current_frame + 1;
                self.timer = 0;
            } else if self.current_frame == 1 && self.timer as f32 > frame_time  {
                self.current_frame = self.current_frame + 1;
                self.main_sprite_sheet = graphics::Image::new(_ctx, "/left_2.png").unwrap();
                self.timer = 0;
            } else if self.current_frame == 2 && self.timer as f32 > frame_time  {
                self.current_frame = self.current_frame + 1;
                self.main_sprite_sheet = graphics::Image::new(_ctx, "/left_3.png").unwrap();
                self.timer = 0;
            } else if self.current_frame == 3 && self.timer as f32 > frame_time  {
                self.current_frame = self.current_frame + 1;
                self.main_sprite_sheet = graphics::Image::new(_ctx, "/left_4.png").unwrap();
                self.timer = 0;
            } else if self.current_frame == 4 && self.timer as f32 > frame_time  {
                self.current_frame = self.current_frame + 1;
                self.main_sprite_sheet = graphics::Image::new(_ctx, "/left_5.png").unwrap();
                self.timer = 0;
            } else if self.current_frame == 5 && self.timer as f32 > frame_time  {
                self.current_frame = self.current_frame + 1;
                self.main_sprite_sheet = graphics::Image::new(_ctx, "/left_6.png").unwrap();
                self.timer = 0;
            } else if self.current_frame == 6 && self.timer as f32 > frame_time  {
                self.current_frame = self.current_frame + 1;
                println!("Heyo");
                self.main_sprite_sheet = graphics::Image::new(_ctx, "/left_7.png").unwrap();
                self.timer = 0;
            } else if self.current_frame == 7 && self.timer as f32 > frame_time  {
                self.current_frame = 0;
                self.main_sprite_sheet = graphics::Image::new(_ctx, "/left_8.png").unwrap();
                self.timer = 0;
            } else {
                self.timer = self.timer + 1;
            }
        }
            // self.image_source = "forward_{}".to_string, {}.to_string();
            Ok(())

    }
    fn movement(&mut self, _ctx: &mut Context) -> GameResult<()> {
        if self.right {
            if self.starting_background_x > -1920.0 && self.pos_x > 960.0 {
                self.starting_background_x = self.starting_background_x - self.speed;
                self.walking_animation(_ctx, 24, 8, 2).unwrap();
            } else if self.pos_x < 1880.0 {
                self.pos_x = self.pos_x + self.speed;
                self.walking_animation(_ctx, 24, 8, 2).unwrap();
            } else if self.pos_x < 1880.0 && self.pos_x < 960.0 {
                self.pos_x = self.pos_x + self.speed;
                self.walking_animation(_ctx, 24, 8, 2).unwrap();
            } else {
                self.walking_animation(_ctx, 24, 8, 2).unwrap();
            }
        } else if self.down {
            if self.starting_background_y > -1070.0 && self.pos_y > 540.0 {
                self.starting_background_y = self.starting_background_y - self.speed;
                self.walking_animation(_ctx, 24, 8, 3).unwrap();
            } else if self.pos_y < 1020.0 {
                self.pos_y = self.pos_y + self.speed;
                self.walking_animation(_ctx, 24, 8, 3).unwrap();
            } else if self.pos_y < 540.0 && self.pos_y < 1020.0{
                self.pos_y = self.pos_y + self.speed;
            } else {
                self.walking_animation(_ctx, 24, 8, 3).unwrap();
            }
        } else if self.up {
            if self.starting_background_y < -20.0  && self.pos_y < 540.0 {
                self.starting_background_y = self.starting_background_y + self.speed;
                self.walking_animation(_ctx, 24, 8, 1).unwrap();
            } else if self.pos_y > 0.0 {
                self.pos_y = self.pos_y - self.speed;
                self.walking_animation(_ctx, 24, 8, 1).unwrap();
            } else if self.pos_y < 540.0 && self.pos_y > 0.0 {
                self.pos_y = self.pos_y - self.speed;
                self.walking_animation(_ctx, 24, 8, 1).unwrap();
            } else {
                self.walking_animation(_ctx, 24, 8, 1).unwrap();
            } // State is direction, 1 is forward, 2 is right, 3 is down, etc
        } else if self.left {
            if self.starting_background_x < -10.0 && self.pos_x < 960.0 {
                self.starting_background_x = self.starting_background_x + self.speed;
                self.walking_animation(_ctx, 24, 8, 4).unwrap();
            } else if self.pos_x > 0.0 {
                self.pos_x = self.pos_x - self.speed;
                self.walking_animation(_ctx, 24, 8, 4).unwrap();
            } else if self.pos_x > 0.0 && self.pos_x > 960.0 {
                self.pos_x = self.pos_x + self.speed;
                self.walking_animation(_ctx, 24, 8, 2).unwrap();
            } else {
                self.walking_animation(_ctx, 24, 8, 4).unwrap();
            }
        } else {
            self.pos_x = self.pos_x;
            self.pos_y = self.pos_y;
        }
        Ok(())
    }
}

impl event::EventHandler for MainState {
    fn resize_event(&mut self, ctx: &mut Context, _width: u32, _height: u32) {
        let rect = ggez::graphics::Rect::new(0.0, 0.0, 1920.0, 1080.0);
        ggez::graphics::set_screen_coordinates(ctx, rect).unwrap();
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: Keycode, keymod: Mod, repeat: bool) {
        match keycode {
            Keycode::Up => {
                self.up = true;
            }
            Keycode::Left => {
                self.left = true;
            }
            Keycode::Right => {
                self.right = true;
            }
            Keycode::Down => {
                self.down = true;
            }
            Keycode::Escape => _ctx.quit().unwrap(),
            _ => (), // Do nothing
        }
    }
    fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, keymod: Mod, repeat: bool) {
        match keycode {
            Keycode::Up => {
                self.up = false;
            }
            Keycode::Left => {
                self.left = false;
            }
            Keycode::Right => {
                self.right = false;
            }
            Keycode::Down => {
                self.down = false;
            }
            Keycode::Escape => _ctx.quit().unwrap(),
            _ => (), // Do nothing
        }
    }
    // Guys turn back, don't even try
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.movement(_ctx).unwrap();
        println!("Player x {}", self.pos_x);
        println!("Player y {}", self.pos_y);
        client::client(self.pos_x as i32, self.pos_y as i32);
        //println!("pos_x {}, pos_y {}", self.pos_x, self.pos_y);
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.resize_event(ctx, 1280, 720);
        graphics::set_background_color(ctx, graphics::WHITE);
        let backgroundpoint = graphics::Point2::new(self.starting_background_x, self.starting_background_y);
        let point = graphics::Point2::new(self.pos_x, self.pos_y);

        graphics::clear(ctx);
        graphics::draw(ctx, &self.starting_background, backgroundpoint, 0.0).unwrap();
        graphics::draw(ctx, &self.main_sprite_sheet, point, 0.0).unwrap();
        graphics::present(ctx);
        Ok(())
    }
}

pub fn main() {
    let mut c = conf::Conf::new();
    c.window_mode.dimensions(1920, 1080);
    c.window_mode.max_dimensions(1920, 1080);
    c.window_setup.resizable = true;

    let ctx = &mut Context::load_from_conf("simple", "me", c).unwrap();
    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}
