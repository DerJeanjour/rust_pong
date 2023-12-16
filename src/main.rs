use piston_window::*;

const WINDOW_WIDTH : u32 = 800;
const WINDOW_HEIGHT : u32 = 600;
const WINDOW_FPS : u64 = 60;

const RED : [f32; 4] = [1.0,0.0,0.0,1.0];
const GREEN : [f32; 4] = [0.0,1.0,0.0,1.0];
const BLUE : [f32; 4] = [0.0,0.0,1.0,1.0];

const BALL_RADIUS : f64 = 10.0;
const BALL_COLOR : [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const BALL_VELOCITY_INC : f64 = 1.0;

const PADDLE_WIDTH : f64 = 10.0;
const PADDLE_HEIGHT : f64 = 160.0;
const PADDLE_COLOR : [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const PADDLE_VELOCITY_INC : f64 = 2.0;

const BACKGROUND_COLOR : [f32; 4] = [0.1, 0.1, 0.1, 1.0];

const ZERO_VEC : Vec2f = Vec2f { x: 0.0, y: 0.0 };

#[derive(Clone)]
struct Vec2f {
    x: f64,
    y: f64,
}

impl Vec2f {
    fn length(&self) -> f64 {
        ( self.x.powi(2) + self.y.powi(2) ).sqrt()
    }
    fn normalize(&self) -> Vec2f {
        let len = self.length();
        if len <= 0.0 {
            return ZERO_VEC.clone();
        }
        Vec2f { x: self.x / len, y: self.y / len }
    }
    fn add(&self, vec : &Vec2f) -> Vec2f {
        Vec2f { x: self.x + vec.x, y: self.y + vec.y }
    }
    fn dot(&self, vec : &Vec2f) -> f64 {
        self.x * vec.x + self.y * vec.y
    }
    fn to_array(&self) -> [f64; 2] {
        [self.x, self.y]
    }
}

impl ToString for Vec2f {
    fn to_string(&self) -> String {
        format!("({:.2}, {:.2})", self.x, self.y)
    }
}

struct BBoxSide {
    a: Vec2f,
    b: Vec2f,
    normal: Vec2f
}

impl BBoxSide {
    fn get_reflect_vec(&self, dir: &Vec2f) -> Vec2f {
        let dot_product = dir.dot(&self.normal);
        Vec2f {
            x: dir.x - 2.0 * dot_product * self.normal.x,
            y: dir.y - 2.0 * dot_product * self.normal.y,
        }.normalize()
    }
}

struct BBox {
    min: Vec2f,
    max: Vec2f
}

impl BBox {
    fn center(&self) -> Vec2f {
        Vec2f { x: ((self.max.x - self.min.x) / 2.0) + self.min.x, y: ((self.max.y - self.min.y) / 2.0) + self.min.y }
    }
    fn get_side(&self, dir: Direction) -> BBoxSide {

        let tl = Vec2f { x: self.min.x, y: self.min.y };
        let tr = Vec2f { x: self.max.x, y: self.min.y };
        let ll = Vec2f { x: self.min.x, y: self.max.y };
        let lr = Vec2f { x: self.max.x, y: self.max.y };

        let mut a = ZERO_VEC.clone();
        let mut b = ZERO_VEC.clone();

        match dir {
            Direction::UP => {
                a = tl;
                b = tr;
            }
            Direction::DOWN => {
                a = ll;
                b = lr;
            }
            Direction::RIGHT => {
                a = tr;
                b = lr;
            }
            Direction::LEFT => {
                a = tl;
                b = ll;
            }
        }
        BBoxSide { a: a, b: b, normal: dir.vector() }
    }
}

enum Direction {
    LEFT,
    RIGHT,
    UP,
    DOWN
}

impl Direction {
    fn vector(&self) -> Vec2f {
        match self {
            Direction::UP => Vec2f { x: 0.0, y: -1.0 },
            Direction::DOWN => Vec2f { x: 0.0, y: 1.0 },
            Direction::LEFT => Vec2f { x: -1.0, y: 0.0 },
            Direction::RIGHT => Vec2f { x: 1.0, y: 0.0 },
        }
    }
}

enum PlayerType {
    HUMAN,
    BOT
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
        BBox { min: Vec2f { x: self.pos.x, y: self.pos.y }, max: Vec2f { x: self.pos.x + self.size.x, y: self.pos.y + self.size.y } }
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
        BBox { min: Vec2f { x: self.pos.x-self.radius, y: self.pos.y-self.radius }, max: Vec2f { x: self.pos.x+self.radius, y: self.pos.y+self.radius } }
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
            velocity: Vec2f { x: -BALL_VELOCITY_INC, y: -BALL_VELOCITY_INC } 
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
            player_type: PlayerType::HUMAN 
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
        [bbox.min.x, bbox.min.y, bbox.max.x - bbox.min.x, bbox.max.y - bbox.min.y],
        context.transform, g2d);
}

fn is_out_of_bounds(bbox: &BBox) -> bool {
    is_out_of_bounds_on_width(bbox) || is_out_of_bounds_on_height(bbox)
}

fn is_out_of_bounds_on_width(bbox: &BBox) -> bool {
    let max_w = WINDOW_WIDTH as f64;
    bbox.min.x < 0.0 || bbox.max.x > max_w
}

fn is_out_of_bounds_on_height(bbox: &BBox) -> bool {
    let max_h = WINDOW_HEIGHT as f64;
    bbox.min.y < 0.0 || bbox.max.y > max_h
}

fn has_collision(bbox_a: &BBox, bbox_b: &BBox) -> bool {
    bbox_a.min.x <= bbox_b.max.x && bbox_a.max.x >= bbox_b.min.x && bbox_a.min.y <= bbox_b.max.y && bbox_a.max.y >= bbox_b.min.y
}

fn get_playground_bounce_direction(ball_velocity: &Vec2f, ball_bbox: &BBox) -> Vec2f {
    let mut bounce_direction = ball_velocity.normalize();
    if is_out_of_bounds_on_width(&ball_bbox) {
        bounce_direction.x *= -1.0;
    }
    if is_out_of_bounds_on_height(&ball_bbox) {
        bounce_direction.y *= -1.0;
    }
    bounce_direction
}

fn get_paddle_bounce_direction(ball_velocity: &Vec2f, ball_bbox: &BBox, bbox: &BBox) -> Vec2f {

    let right_bbox : BBoxSide = bbox.get_side(Direction::RIGHT);
    let left_ball_bbox : BBoxSide = ball_bbox.get_side(Direction::LEFT);
    if right_bbox.a.x > left_ball_bbox.a.x {
        return right_bbox.get_reflect_vec(&ball_velocity);
    }

    let left_bbox : BBoxSide = bbox.get_side(Direction::LEFT);
    let right_ball_bbox : BBoxSide = ball_bbox.get_side(Direction::RIGHT);
    if left_bbox.a.x < right_ball_bbox.a.x {
        return left_bbox.get_reflect_vec(&ball_velocity);
    }

    let top_bbox : BBoxSide = bbox.get_side(Direction::UP);
    let bottom_ball_bbox : BBoxSide = ball_bbox.get_side(Direction::DOWN);
    if top_bbox.a.y < bottom_ball_bbox.a.y {
        return top_bbox.get_reflect_vec(&ball_velocity);
    }

    let bottom_bbox : BBoxSide = bbox.get_side(Direction::DOWN);
    let top_ball_bbox : BBoxSide = ball_bbox.get_side(Direction::UP);
    if bottom_bbox.a.y > top_ball_bbox.a.y {
        return bottom_bbox.get_reflect_vec(&ball_velocity);
    }

    ZERO_VEC

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

    // check bounces
    let mut ball_velocity = game_state.get_ball().velocity.clone();
    let mut bounce_direction = ball_velocity.normalize();

    if is_out_of_bounds( &game_state.get_ball().get_bbox() ) {
        bounce_direction = get_playground_bounce_direction(&ball_velocity, &game_state.get_ball().get_bbox());
    }

    if has_collision(&game_state.get_ball().get_bbox(), &game_state.get_paddle_left().get_bbox()) {
        bounce_direction = get_paddle_bounce_direction(&ball_velocity, &game_state.get_ball().get_bbox(), &game_state.get_paddle_left().get_bbox());
    }

    if has_collision(&game_state.get_ball().get_bbox(), &game_state.get_paddle_right().get_bbox()) {
        bounce_direction = get_paddle_bounce_direction(&ball_velocity, &game_state.get_ball().get_bbox(), &game_state.get_paddle_right().get_bbox());
    }

    // update ball velocity
    ball_velocity.x = bounce_direction.x * BALL_VELOCITY_INC;
    ball_velocity.y = bounce_direction.y * BALL_VELOCITY_INC;
    game_state.get_ball().velocity = ball_velocity;
}