use crossterm::{
    cursor::MoveTo,
    queue,
    style::{Color, PrintStyledContent, Stylize},
};
use std::io::{Result, Stdout};

#[derive(Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub struct Pos(pub u16, pub u16);

pub fn draw_food(out: &mut Stdout, pos: &Pos) -> Result<()> {
    queue!(
        out,
        MoveTo(pos.0, pos.1),
        PrintStyledContent("@".with(Color::Red))
    )
}

pub struct Snake {
    body: Vec<Pos>,
    dir: Direction,
}

impl Snake {
    pub fn new(x: u16, y: u16) -> Self {
        Self {
            body: vec![Pos(x, y)],
            dir: Direction::Right,
        }
    }

    fn check_collisions(&self) -> bool {
        let head = self.body.get(0).unwrap();
        for part in self.body.iter().skip(1) {
            if part.0 == head.0 && part.1 == head.1 {
                return true;
            }
        }
        false
    }

    pub fn change_direction(&mut self, dir: Direction, size: (u16, u16)) {
        let (w, h) = size;
        let head = self.body.get(0).unwrap();
        if let Some(second) = self.body.get(1) {
            if self.dir == dir
                || matches!(dir, Direction::Up) && second.1 + 1 == head.1 && second.0 == head.0
                || matches!(dir, Direction::Down)
                    && second.1.checked_sub(1).unwrap_or(0) == head.1
                    && second.0 == head.0
                || matches!(dir, Direction::Left) && second.0 + 1 == head.0 && second.1 == head.1
                || matches!(dir, Direction::Right)
                    && second.0.checked_sub(1).unwrap_or(0) == head.0
                    && second.1 == head.1
            {
                return;
            }
        }
        self.dir = dir
    }

    pub fn check_eat(&mut self, food: &Pos) -> bool {
        let Pos(head_x, head_y) = self.body.get(0).unwrap();

        if food.0 == *head_x && food.1 == *head_y {
            self.body.insert(0, food.clone());
            return true;
        }

        false
    }

    pub fn update(&mut self, food: &Pos, size: (u16, u16)) -> std::result::Result<bool, ()> {
        let head = self.body.get_mut(0).unwrap();
        let (w, h) = (size.0 - 1, size.1 - 1);
        let mut last_pos = head.clone();
        match self.dir {
            Direction::Up => head.1 = head.1.checked_sub(1).unwrap_or(h),
            Direction::Down => {
                head.1 += 1;
                if head.1 > h {
                    head.1 = 0
                }
            }
            Direction::Left => head.0 = head.0.checked_sub(1).unwrap_or(w),
            Direction::Right => {
                head.0 += 1;
                if head.0 > w {
                    head.0 = 0
                }
            }
        }
        if self.check_collisions() {
            return Err(());
        }
        for part in self.body.iter_mut().skip(1) {
            let tmp = part.clone();
            part.0 = last_pos.0;
            part.1 = last_pos.1;
            last_pos = tmp;
        }
        Ok(self.check_eat(food))
    }

    pub fn draw(&self, out: &mut Stdout) -> Result<()> {
        for part in &self.body {
            let Pos(x, y) = part;
            queue!(out, MoveTo(*x, *y))?;
            queue!(out, PrintStyledContent("#".with(Color::Green)))?;
        }

        Ok(())
    }
}
