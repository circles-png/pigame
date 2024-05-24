#![allow(clippy::cast_precision_loss)]

use anyhow::Result;
use pigame::graphics::text::{draw_text_ex, load_ttf_font, Properties};
use pigame::graphics::{
    clear_background, draw_rectangle, get_frame_time, get_time, next_frame, screen_height,
    screen_width,
};
use pigame::input::is_active;
use pigame::{
    graphics::{
        colour::{BLACK, BLUE, GREEN, ORANGE, RED, WHITE, YELLOW},
        text::FontSettings,
    },
    input::Input,
    maths::{
        glam::{vec2, Vec2},
        Rect,
    },
    rand::{random, thread_rng, Rng},
};

const PLAYER_SIZE: Vec2 = Vec2::from_array([53., 12.]);
const PLAYER_SPEED: f32 = 700.;
const BLOCK_SIZE: Vec2 = Vec2::from_array([53., 12.]);
const BALL_SIZE: f32 = 7.;

pub enum GameState {
    Menu,
    Game,
    LevelCompleted,
    Dead,
}

struct Player {
    target: Vec2,
    rect: Rect,
    dead: bool,
}

impl Player {
    pub fn new() -> Self {
        Self {
            target: Vec2 {
                x: screen_width() as f32 / 2. - PLAYER_SIZE.x / 2.,
                y: screen_height() as f32 - 40.,
            },
            rect: Rect::new(
                screen_width() as f32 / 2. - PLAYER_SIZE.x / 2.,
                screen_height() as f32 - 40.,
                PLAYER_SIZE.x,
                PLAYER_SIZE.y,
            ),
            dead: false,
        }
    }

    pub fn update(&mut self) {
        let x_move = match (is_active(Input::Left), is_active(Input::Right)) {
            (true, false) => -1.,
            (false, true) => 1.,
            _ => 0.,
        };
        self.target.x += x_move * PLAYER_SPEED * get_frame_time().as_secs_f32();
        self.target.x = self.target.x.clamp(0., screen_width() as f32 - self.rect.w);
        self.rect.x = (self.rect.x + self.target.x) / 2.;
    }

    pub fn draw(&self) {
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        draw_rectangle(
            self.rect.x as u32,
            self.rect.y as u32,
            self.rect.w as u32,
            self.rect.h as u32,
            BLUE,
        );
    }
}

struct Block {
    rect: Rect,
    lives: i32,
    row: i32,
}

impl Block {
    pub const fn new(pos: Vec2, row: i32) -> Self {
        Self {
            rect: Rect::new(pos.x, pos.y, BLOCK_SIZE.x, BLOCK_SIZE.y),
            lives: 1,
            row,
        }
    }

    pub fn draw(&self) {
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        draw_rectangle(
            self.rect.x as u32,
            self.rect.y as u32,
            self.rect.w as u32,
            self.rect.h as u32,
            match self.row {
                0 | 1 => RED,
                2 | 3 => ORANGE,
                4 | 5 => YELLOW,
                6 | 7 => GREEN,
                _ => unreachable!(),
            },
        );
    }
}

struct Ball {
    rect: Rect,
    vel: Vec2,
    hit_upper_wall: bool,
    hit_lower_wall: bool,
}

impl Ball {
    pub fn new(pos: Vec2) -> Self {
        Self {
            rect: Rect::new(pos.x, pos.y, BALL_SIZE, BALL_SIZE),
            vel: vec2(
                if random() {
                    thread_rng().gen_range(-2. ..=-0.5)
                } else {
                    thread_rng().gen_range(0.5..=2.)
                },
                1.,
            )
            .normalize(),
            hit_upper_wall: false,
            hit_lower_wall: false,
        }
    }

    pub fn update(&mut self, ball_speed: i32, dead: bool) {
        self.rect.x += self.vel.x * ball_speed as f32 * get_frame_time().as_secs_f32();
        self.rect.y += self.vel.y * ball_speed as f32 * get_frame_time().as_secs_f32();
        if self.rect.x < 5. {
            self.vel.x *= -1.;
        }
        if self.rect.x > screen_width() as f32 - self.rect.w - 5. {
            self.vel.x *= -1.;
        }
        if self.rect.y < 0. {
            self.vel.y = 1.;
            self.hit_upper_wall = true;
        }
        if self.rect.y > screen_height() as f32 && !dead {
            self.hit_lower_wall = true;
        }
        if self.rect.y > screen_height() as f32 && dead {
            self.vel.y = -1.;
        }
    }

    pub fn draw(&self) {
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        draw_rectangle(
            self.rect.x as u32,
            self.rect.y as u32,
            self.rect.w as u32,
            self.rect.h as u32,
            WHITE,
        );
    }
}

fn resolve_collision(a: &mut Rect, vel: &mut Vec2, b: &Rect) -> bool {
    let Some(intersection) = a.intersect(*b) else {
        return false;
    };

    let a_center = a.centre();
    let b_center = b.centre();
    let to = b_center - a_center;
    let to_signum = to.signum();
    if intersection.w > intersection.h {
        a.y -= to_signum.y * intersection.h;
        // if to_signum.y > 5. {
        //     vel.y = -vel.y.abs();
        // } else {
        //     vel.y = vel.y.abs();
        // }
        vel.y = -to_signum.y * vel.y.abs();
    } else {
        a.x -= to_signum.x * intersection.w;
        // if to_signum.x < 5. {
        //     vel.x = vel.x.abs();
        // } else {
        //     vel.x = -vel.x.abs();
        // }
        vel.x = -to_signum.x * vel.x.abs();
    }
    true
}

fn init_blocks(blocks: &mut Vec<Block>) {
    let (width, height) = (14, 8);
    let padding = 5.;
    let total_block_size = BLOCK_SIZE + vec2(padding, padding);
    let board_start_pos = vec2(
        total_block_size
            .x
            .mul_add(-(width as f32), screen_width() as f32)
            * 0.5,
        60.,
    );

    for i in 0..width * height {
        let block_x = (i % width) as f32 * total_block_size.x;
        let block_y = (i / width) as f32 * total_block_size.y;
        let row = i / width;
        blocks.push(Block::new(board_start_pos + vec2(block_x, block_y), row));
    }
}

#[allow(clippy::too_many_lines)]
fn main() -> Result<()> {
    let font = load_ttf_font(
        "res/Quinque Five Font.ttf",
        FontSettings {
            scale: 30.,
            ..Default::default()
        },
    )?;
    let mut score = 0;
    let mut hits = 0;
    let mut player = Player::new();
    let mut player_lives = 1;
    let mut already_hit_upper_wall = false;
    let mut already_hit_lower_wall = false;
    let mut blocks = Vec::new();
    let mut ball_spawned = false;
    let mut ball = Ball::new(vec2(player.rect.x, screen_height() as f32 / 2.));
    let mut ball_speed: i32 = 300;

    init_blocks(&mut blocks);

    loop {
        if !player.dead {
            player.update();
        }

        if player_lives == 4 {
            player.dead = true;
        }

        if ball.hit_upper_wall && !already_hit_upper_wall {
            player.rect.w /= 2.;
            already_hit_upper_wall = true;
        }
        if ball.hit_lower_wall && !already_hit_lower_wall {
            player_lives += 1;
            already_hit_lower_wall = true;
            ball_spawned = false;
        }

        if resolve_collision(&mut ball.rect, &mut ball.vel, &player.rect) {
            hits += 1;
            match hits {
                4 => {
                    if ball_speed < 375 {
                        ball_speed = 375;
                    }
                }
                12 => {
                    if ball_speed < 450 {
                        ball_speed = 450;
                    }
                }
                _ => {}
            }
            ball.vel = vec2(
                if random() {
                    thread_rng().gen_range(-2. ..=-0.5)
                } else {
                    thread_rng().gen_range(0.5..=2.)
                },
                -1.,
            )
            .normalize();
        }
        for block in &mut blocks {
            if resolve_collision(&mut ball.rect, &mut ball.vel, &block.rect) {
                block.lives -= 1;
                if !player.dead {
                    match block.row {
                        0 | 1 => {
                            score += 7;
                            if ball_speed != 600 {
                                ball_speed = 600;
                            }
                        }
                        2 | 3 => {
                            score += 6;
                            if ball_speed < 525 {
                                ball_speed = 525;
                            }
                        }
                        4 | 5 => score += 3,
                        6 | 7 => score += 1,
                        _ => unreachable!(),
                    }
                }
            }
        }

        clear_background(BLACK);

        if !player.dead {
            blocks.retain(|block| block.lives > 0);
            player.draw();
        }
        for block in &blocks {
            block.draw();
        }
        if is_active(Input::A) && !ball_spawned {
            ball_spawned = true;
            already_hit_lower_wall = false;
            ball = Ball::new(vec2(player.rect.x, screen_height() as f32 / 2.));
            hits = 0;
            ball_speed = 300;
        }
        if ball_spawned {
            ball.update(ball_speed, player.dead);
            ball.draw();
        }

        let score_text = format!("{score:0>#3}");
        let lives_text = format!("{player_lives}");
        let text_params = &Properties {
            font,
            scale: 1.,
            rotation: 0.,
            colour: WHITE,
        };
        #[allow(clippy::cast_possible_truncation)]
        if ((get_time() * 6.) as i32 % 2 == 0) && !player.dead {
            draw_text_ex(&score_text, 60, 40, text_params);
        }
        if player.dead {
            player.rect.y = screen_height() as f32 + 10.;
            draw_text_ex(&score_text, 60, 40, text_params);
        }
        draw_text_ex(&lives_text, screen_width() - 60, 40, text_params);
        next_frame()?;
    }
}
