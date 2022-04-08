use macroquad::prelude::*; // use everything from the macroquad cargo module

struct MainState {
    ball: Rect,
    x_vel: f32,
    y_vel: f32,
    top_paddle: Rect,
    bottom_paddle: Rect,

    // scores of food
    top_score: usize,
    bottom_score: usize,

    // my addition to color
    color: Color,
}

// add methods to mainstate
impl MainState {
    fn draw(&self) {
        // draw the ball
        draw_rectangle(
            self.ball.x,
            self.ball.y,
            self.ball.w,
            self.ball.h,
            self.color);

        // draw paddles
        draw_rectangle(
            self.top_paddle.x,
            self.top_paddle.y,
            self.top_paddle.w,
            self.top_paddle.h,
            self.color);
        draw_rectangle(        
            self.bottom_paddle.x,
            self.bottom_paddle.y,
            self.bottom_paddle.w,
            self.bottom_paddle.h,
            self.color);

        // draw text
        draw_text(
            &self.top_score.to_string(), 
            10.0,
            100.0,
            100.0,
            WHITE
        );

        // draw text
        draw_text(
            &self.bottom_score.to_string(), 
            10.0,
            screen_height() - 100.0,
            100.0,
            WHITE
                 );
 
    }

    // only accepts a mutable reference
    // only allowed to pass one reference at a time
    fn update(&mut self) {
        self.ball.x += self.x_vel;
        self.ball.y += self.y_vel;

        // touches the top of the screen, flip y_velocity
        if self.ball.overlaps(&self.top_paddle)
        || self.ball.overlaps(&self.bottom_paddle) {
                self.y_vel *= -1.0;
                self.color = RED;
            }


        // bouncing off horizontal borders
        if self.ball.left()  <= 0.0 
            || self.ball.right() >= screen_width() {
                self.x_vel *= -1.0;
                self.color = BLUE;
        }


        // update paddles
        if is_key_down(KeyCode::Right) 
        && self.top_paddle.right() <= screen_width()
        {
            self.top_paddle.x += 10.0;
        }
        if is_key_down(KeyCode::Left) 
        && self.top_paddle.left() >= 0.0
        {
            self.top_paddle.x -= 10.0;
        }
        if is_key_down(KeyCode::D) 
        && self.bottom_paddle.right() <= screen_width()
        {
            self.bottom_paddle.x += 10.0;
        }
        if is_key_down(KeyCode::A) 
        && self.bottom_paddle.left() >= 0.0
        {
            self.bottom_paddle.x -= 10.0;
        }


        // if ball disappears, restart
        if self.ball.bottom() <= 0.0
            || self.ball.top() >= screen_height() {
                self.ball.x = screen_width() / 2.0;
                self.ball.y = screen_height() / 2.0;

                self.x_vel *= -1.0;
                self.y_vel *= -1.0;
            }

    }

}

#[macroquad::main("Pong")]
async fn main() {


    // initialize y as an immutable 32 bit float
    // the type is automatically inferenced

    let mut main_state = MainState {
        ball: Rect::new(
            screen_width() / 2.0,
            screen_height() / 2.0,
            10.0,
            10.0,
        ),
        x_vel: 5.0,
        y_vel: -5.0,
        top_paddle: Rect::new(
            screen_width() / 2.0,
            0.0,
            50.0,
            10.0,
                             ),
        bottom_paddle: Rect::new(
            screen_width() / 2.0,
            screen_height()-10.0,
            50.0,
            10.0,
                             ),
        top_score: 0,
        bottom_score: 0,
        color: WHITE,
    };

    // loop while(true)
    loop {
        // clear screen
        clear_background(BLACK);

        main_state.draw();

        // draw the rectangle at x, y (10, 10 rectangle)
        main_state.update();

        // check for collisions
        
        // wait for next frame
        next_frame().await;
    }
}
