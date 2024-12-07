use crossterm::{
    cursor,
    event::{self, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{self, ClearType},
    ExecutableCommand,
};
use rand::Rng;
use std::{
    io::{self, stdout, Write},
    result::Result,
    thread,
    time::Duration,
};

const WIDTH: usize = 20;
const HEIGHT: usize = 10;

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Snake {
    body: Vec<(usize, usize)>,
    direction: Direction,
}

impl Snake {
    fn new() -> Self {
        Self {
            body: vec![(5, 5), (5, 4), (5, 3)],
            direction: Direction::Right,
        }
    }

    fn head(&self) -> (usize, usize) {
        self.body[0]
    }

    fn move_forward(&mut self) {
        let (head_x, head_y) = self.head();
        let new_head = match self.direction {
            Direction::Up => (head_x, head_y - 1),
            Direction::Down => (head_x, head_y + 1),
            Direction::Left => (head_x - 1, head_y),
            Direction::Right => (head_x + 1, head_y),
        };

        self.body.insert(0, new_head);
        self.body.pop();
    }

    fn grow(&mut self) {
        let (tail_x, tail_y) = self.body[self.body.len() - 1];
        self.body.push((tail_x, tail_y));
    }

    fn collision(&self) -> bool {
        let (head_x, head_y) = self.head();

        // Collision with walls
        if head_x >= WIDTH || head_y >= HEIGHT || head_x < 0 || head_y < 0 {
            return true;
        }

        // Collision with itself
        for &segment in self.body.iter().skip(1) {
            if segment == (head_x, head_y) {
                return true;
            }
        }

        false
    }
}

struct Food {
    position: (usize, usize),
}

impl Food {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0..WIDTH);
        let y = rng.gen_range(0..HEIGHT);

        Self { position: (x, y) }
    }

    fn spawn(&mut self) {
        let mut rng = rand::thread_rng();
        self.position = (rng.gen_range(0..WIDTH), rng.gen_range(0..HEIGHT));
    }
}

fn draw(snake: &Snake, food: &Food) -> std::result::Result<(), Box<dyn std::error::Error>> {
    execute!(std::io::stdout(), terminal::Clear(ClearType::All));

    // Draw the border
    for x in 0..=WIDTH as u16 {
        execute!(std::io::stdout(), cursor::MoveTo(x, 0))?;
        print!("-");
        execute!(std::io::stdout(), cursor::MoveTo(x, HEIGHT as u16))?;
        print!("-");
    }

    for y in 0..=HEIGHT as u16 {
        execute!(std::io::stdout(), cursor::MoveTo(0, y))?;
        print!("|");
        execute!(std::io::stdout(), cursor::MoveTo(WIDTH as u16, y))?;
        print!("|");
    }

    // Draw the snake
    for &(x, y) in &snake.body {
        execute!(std::io::stdout(), cursor::MoveTo(x as u16, y as u16),)?;
        print!("■");
    }

    // Draw the food
    let (food_x, food_y) = food.position;
    execute!(
        std::io::stdout(),
        cursor::MoveTo(food_x as u16, food_y as u16),
    )?;
    print!("■");

    Ok(())
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    terminal::enable_raw_mode()?;
    let mut snake = Snake::new();
    let mut food = Food::new();

    loop {
        draw(&snake, &food)?;

        // Check for input events
        if event::poll(Duration::from_millis(100))? {
            if let event::Event::Key(KeyEvent {
                code,
                modifiers: _,
                kind: _,
                state: _,
            }) = event::read()?
            {
                match code {
                    KeyCode::Esc => break,
                    KeyCode::Up => {
                        if snake.direction != Direction::Down {
                            snake.direction = Direction::Up;
                        }
                    }
                    KeyCode::Down => {
                        if snake.direction != Direction::Up {
                            snake.direction = Direction::Down;
                        }
                    }
                    KeyCode::Left => {
                        if snake.direction != Direction::Right {
                            snake.direction = Direction::Left;
                        }
                    }
                    KeyCode::Right => {
                        if snake.direction != Direction::Left {
                            snake.direction = Direction::Right;
                        }
                    }
                    _ => {}
                }
            }
        }

        snake.move_forward();

        // Check if snake eats the food
        if snake.head() == food.position {
            snake.grow();
            food.spawn();
        }

        // Check for collisions
        if snake.collision() {
            println!("\nGame Over!");
            break;
        }

        thread::sleep(Duration::from_millis(100));
    }

    terminal::disable_raw_mode()?;
    Ok(())
}
