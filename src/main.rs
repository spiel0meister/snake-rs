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
use std::io::{stdout, Result, Stdout, Write};
use std::thread::sleep;
use std::time::Duration;

mod snake;
use snake::{draw_food, Direction, Pos, Snake};

const FPS: u64 = 15;

fn rand_pos(rng: &mut impl Rng, max_w: u16, max_h: u16) -> Pos {
    Pos(rng.gen::<u16>() % max_w, rng.gen::<u16>() % max_h)
}

fn draw_score(out: &mut Stdout, score: usize) {
    use crossterm::{cursor::MoveTo, style::Print};
    queue!(out, MoveTo(0, 0)).unwrap();
    queue!(out, Print(format!("Score: {}", score))).unwrap();
}

fn main() -> Result<()> {
    let mut rng = thread_rng();
    let mut out = stdout();
    enable_raw_mode()?;
    execute!(out, EnterAlternateScreen)?;
    execute!(out, Hide)?;

    let (mut max_w, mut max_h) = size()?;
    let side = std::cmp::min(max_w, max_h);
    let mut player = Snake::new(side / 4, side / 2);
    let mut food = rand_pos(&mut rng, side, side);
    let mut score = 0usize;

    loop {
        if poll(Duration::ZERO)? {
            match read()? {
                Event::Key(event) => match event.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Up => player.change_direction(Direction::Up, (max_w, max_h)),
                    KeyCode::Down => player.change_direction(Direction::Down, (max_w, max_h)),
                    KeyCode::Left => player.change_direction(Direction::Left, (max_w, max_h)),
                    KeyCode::Right => player.change_direction(Direction::Right, (max_w, max_h)),
                    _ => {}
                },
                Event::Resize(w, h) => {
                    max_w = w;
                    max_h = h;
                }
                _ => {}
            }
        }
        let out_ = player.update(&food, (max_w, max_h));
        if let Ok(true) = out_ {
            score += 1;
            food = rand_pos(&mut rng, side, side);
        };
        if let Err(_) = out_ {
            break;
        };

        queue!(out, Clear(ClearType::All))?;
        draw_score(&mut out, score);
        draw_food(&mut out, &food)?;
        player.draw(&mut out)?;

        out.flush()?;

        sleep(Duration::from_millis(1000 / FPS));
    }

    execute!(out, LeaveAlternateScreen)?;
    execute!(out, Show)?;
    disable_raw_mode()?;
    println!("FIN. SCORE: {score}");

    Ok(())
}
