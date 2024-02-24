use crossterm::{
    cursor::{Hide, Show},
    event::{poll, read, Event, KeyCode},
    execute, queue,
    terminal::{
        disable_raw_mode, enable_raw_mode, size, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use rand::{thread_rng, Rng};
use std::io::{stdout, Result, Write};
use std::thread::sleep;
use std::time::Duration;

mod snake;
use snake::{draw_food, Direction, Pos, Snake};

const FPS: u64 = 15;

fn rand_pos(rng: &mut impl Rng, max_w: u16, max_h: u16) -> Pos {
    Pos(rng.gen::<u16>() % max_w, rng.gen::<u16>() % max_h)
}

fn main() -> Result<()> {
    let mut rng = thread_rng();
    let mut out = stdout();
    enable_raw_mode()?;
    execute!(out, EnterAlternateScreen)?;
    execute!(out, Hide)?;

    let (max_w, max_h) = size()?;
    let mut player = Snake::new(max_w / 4, max_h / 2);
    let mut food = rand_pos(&mut rng, max_w, max_h);

    loop {
        if poll(Duration::ZERO)? {
            match read()? {
                Event::Key(event) => match event.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Up => player.change_direction(Direction::Up),
                    KeyCode::Down => player.change_direction(Direction::Down),
                    KeyCode::Left => player.change_direction(Direction::Left),
                    KeyCode::Right => player.change_direction(Direction::Right),
                    _ => {}
                },
                _ => {}
            }
        }
        let out_ = player.update(&food, (max_w, max_h));
        if let Ok(true) = out_ {
            food = rand_pos(&mut rng, max_w, max_h);
        };
        if let Err(_) = out_ {
            break;
        };

        queue!(out, Clear(ClearType::All))?;
        draw_food(&mut out, &food)?;
        player.draw(&mut out)?;

        out.flush()?;

        sleep(Duration::from_millis(1000 / FPS));
    }

    execute!(out, LeaveAlternateScreen)?;
    execute!(out, Show)?;
    disable_raw_mode()?;
    println!("YOU LOST!");

    Ok(())
}
