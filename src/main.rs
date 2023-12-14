use piston_window::{*, types::Vec2d};

const WINDOW_WIDTH : u32 = 800;
const WINDOW_HEIGHT : u32 = 600;
const WINDOW_FPS : u64 = 60;

const BALL_RADIUS : f64 = 10.0;
const BALL_COLOR : [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const BALL_VELOCITY_INC : f64 = -1.0;

const PADDLE_WIDTH : f64 = 10.0;
const PADDLE_HEIGHT : f64 = 80.0;
const PADDLE_COLOR : [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const PADDLE_VELOCITY_INC : f64 = 2.0;

const BACKGROUND_COLOR : [f32; 4] = [0.1, 0.1, 0.1, 1.0];

enum PlayerType {
    HUMAN,
    BOT
}

struct Vec2f {
    x: f64,
    y: f64,
}

struct BBox {
    ll: Vec2f,
    ur: Vec2f
}

trait GameElement {
    fn get_bbox(&self) -> BBox;
    fn draw(&self, context: &Context, g2d: &mut G2d);
}

struct Paddle {
    pos: Vec2f,
    size: Vec2f,
    velocity: Vec2f,
    player_type: PlayerType
}

impl Paddle {
    fn is_human(&self) -> bool {
        matches!(self.player_type, PlayerType::HUMAN)
    }
}

impl GameElement for Paddle {
    fn get_bbox(&self) -> BBox {
        BBox { ll: Vec2f { x: self.pos.x, y: self.pos.y }, ur: Vec2f { x: self.pos.x + self.size.x, y: self.pos.y + self.size.y } }
    }
    fn draw(&self, context: &Context, g2d: &mut G2d) {
        rectangle(PADDLE_COLOR,
            [self.pos.x, self.pos.y, self.size.x, self.size.y],
            context.transform, g2d);
    }
}

struct Ball {
    pos: Vec2f,
    radius: f64,
    velocity: Vec2f
}

impl GameElement for Ball {
    fn get_bbox(&self) -> BBox {
        BBox { ll: Vec2f { x: self.pos.x-self.radius, y: self.pos.y-self.radius }, ur: Vec2f { x: self.pos.x+self.radius, y: self.pos.y+self.radius } }
    }
    fn draw(&self, context: &Context, g2d: &mut G2d) {
        ellipse(BALL_COLOR,
            [self.pos.x - self.radius, self.pos.y - self.radius, self.radius * 2.0, self.radius * 2.0],
            context.transform, g2d);
    }
}

struct GameState {
    ball: Ball,
    paddle_left: Paddle,
    paddle_right: Paddle
}

impl GameState {
    fn get_ball(&mut self) -> &mut Ball {
        &mut self.ball
    }
    fn get_paddle_left(&mut self) -> &mut Paddle {
        &mut self.paddle_left
    }
    fn get_paddle_right(&mut self) -> &mut Paddle {
        &mut self.paddle_right
    }
    fn set_human_paddle_velocity(&mut self, velocity_y: f64) {
        if self.paddle_left.is_human() {
            self.paddle_left.velocity.y = velocity_y;
        }
        if self.paddle_right.is_human() {
            self.paddle_right.velocity.y = velocity_y;
        }
    }
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Pong", [WINDOW_WIDTH, WINDOW_HEIGHT])
        .exit_on_esc(true)
        .build()
        .unwrap();

    window.set_max_fps(WINDOW_FPS);
    window.set_ups(WINDOW_FPS);

    let mut game_state = GameState { 
        ball: Ball { 
            pos: Vec2f { x: 400.0, y: 300.0 }, 
            radius: BALL_RADIUS, 
            velocity: Vec2f { x: BALL_VELOCITY_INC, y: BALL_VELOCITY_INC } 
        },
        paddle_left: Paddle { 
            pos: Vec2f { x: 100.0, y: 250.0 }, 
            size: Vec2f { x: PADDLE_WIDTH, y: PADDLE_HEIGHT }, 
            velocity: Vec2f {  x: 0.0, y: 0.0 }, 
            player_type: PlayerType::HUMAN 
        },
        paddle_right: Paddle { 
            pos: Vec2f { x: 700.0, y: 250.0 }, 
            size: Vec2f { x: PADDLE_WIDTH, y: PADDLE_HEIGHT }, 
            velocity: Vec2f {  x: 0.0, y: 0.0 }, 
            player_type: PlayerType::BOT 
        }
    };

    while let Some( event) = window.next() {

        handle_input(&event, &mut game_state);
        handle_bot(&mut game_state);
        update_game(&mut game_state);

        window.draw_2d(&event, |context, g2d, _| {

            clear(BACKGROUND_COLOR, g2d);

            game_state.get_paddle_left().draw(&context, g2d);
            game_state.get_paddle_right().draw(&context, g2d);
            game_state.get_ball().draw(&context, g2d);

            draw_bbox(game_state.get_paddle_left(), &context, g2d);
            draw_bbox(game_state.get_paddle_right(), &context, g2d);
            draw_bbox(game_state.get_ball(), &context, g2d);
        });

    }
}

fn draw_bbox(element: &dyn GameElement, context: &Context, g2d: &mut G2d) {
    let bbox = element.get_bbox();
    rectangle([1.0, 0.0, 0.0, 0.2],
        [bbox.ll.x, bbox.ll.y, bbox.ur.x - bbox.ll.x, bbox.ur.y - bbox.ll.y],
        context.transform, g2d);
}

fn is_out_of_bounds(bbox: &BBox) -> bool {
    is_out_of_bounds_on_width(bbox) || is_out_of_bounds_on_height(bbox)
}

fn is_out_of_bounds_on_width(bbox: &BBox) -> bool {
    let max_w = WINDOW_WIDTH as f64;
    bbox.ll.x < 0.0 || bbox.ur.x > max_w
}

fn is_out_of_bounds_on_height(bbox: &BBox) -> bool {
    let max_h = WINDOW_HEIGHT as f64;
    bbox.ll.y < 0.0 || bbox.ur.y > max_h
}

fn has_collision(bbox_a: &BBox, bbox_b: &BBox) -> bool {
    bbox_a.ll.x <= bbox_b.ur.x && bbox_a.ur.x >= bbox_b.ll.x && bbox_a.ll.y <= bbox_b.ur.y && bbox_a.ur.y >= bbox_b.ll.y
}

fn get_bounce_direction(ball_velocity: &Vec2f, ball_bbox: &BBox, bbox: &BBox) -> Vec2f {

    let ball_center : Vec2f = get_bbox_center(ball_bbox);
    let bbox_center : Vec2f = get_bbox_center(bbox);
    let mut bounce_direction : Vec2f = Vec2f { x: 1.0, y: 1.0 };

    if ball_velocity.x > 0.0 && ball_center.x < bbox_center.x  {
        bounce_direction.x = -1.0;
    } 
    if ball_velocity.y > 0.0 && ball_center.y < bbox_center.y {
        bounce_direction.y = -1.0;
    }

    bounce_direction
}

fn get_bbox_center(bbox: &BBox) -> Vec2f {
    Vec2f { x: ((bbox.ur.x - bbox.ll.x) / 2.0) + bbox.ll.x, y: ((bbox.ur.y - bbox.ll.y) / 2.0) + bbox.ll.y }
}

fn clamp(value: f64, min: f64, max: f64) -> f64 {
    if value < min {
        return min;
    } else if value > max {
        return max;
    }
    value
}

fn handle_input(event: &Event, game_state: &mut GameState) {

    // on key press
    if let Some(Button::Keyboard(key)) = event.press_args() {
        match key {
            Key::Up => game_state.set_human_paddle_velocity(-PADDLE_VELOCITY_INC),
            Key::Down => game_state.set_human_paddle_velocity(PADDLE_VELOCITY_INC),
            _ => {}
        }
    }

    // on key release
    if let Some(Button::Keyboard(key)) = event.release_args() {
        match key {
            Key::Up => game_state.set_human_paddle_velocity(0.0),
            Key::Down => game_state.set_human_paddle_velocity(0.0),
            _ => {}
        }
    }

}

fn handle_bot(game_state: &mut GameState) {
    // TODO
}

fn update_game(game_state: &mut GameState) {

    let min = 0.0;
    let max_w = WINDOW_WIDTH as f64;
    let max_h = WINDOW_HEIGHT as f64;

    // move paddle
    game_state.get_paddle_left().pos.y = clamp( game_state.get_paddle_left().pos.y + game_state.get_paddle_left().velocity.y, min, max_h - game_state.get_paddle_left().size.y );
    game_state.get_paddle_right().pos.y = clamp( game_state.get_paddle_right().pos.y + game_state.get_paddle_right().velocity.y, min, max_h - game_state.get_paddle_right().size.y );

    // move ball
    game_state.get_ball().pos.x += game_state.get_ball().velocity.x;
    game_state.get_ball().pos.y += game_state.get_ball().velocity.y;

    // handle collisions
    let mut ball_velocity = Vec2f { x: game_state.get_ball().velocity.x, y: game_state.get_ball().velocity.y };

    if is_out_of_bounds_on_width(&game_state.get_ball().get_bbox()) {
        ball_velocity.x *= -1.0;
    }
    if is_out_of_bounds_on_height(&game_state.get_ball().get_bbox()) {
        ball_velocity.y *= -1.0;
    }

    if has_collision(&game_state.get_ball().get_bbox(), &game_state.get_paddle_left().get_bbox()) {
        // how to determine the direction?
        let bounce_direction : Vec2f = get_bounce_direction(&ball_velocity, &game_state.get_ball().get_bbox(), &game_state.get_paddle_left().get_bbox());
        ball_velocity.y = bounce_direction.x * BALL_VELOCITY_INC;
        ball_velocity.x = bounce_direction.y * BALL_VELOCITY_INC;
    }

    game_state.get_ball().velocity = ball_velocity;

}