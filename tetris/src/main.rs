use nannou::prelude::*;
use nannou::rand::seq::SliceRandom;
use nannou::rand::thread_rng;

const BLOCK_SIZE: f32 = 30.0;
const BOARD_WIDTH: usize = 10;
const BOARD_HEIGHT: usize = 20;

const T_WHITE: Rgb8 = WHITESMOKE;
const T_BG: Rgb8 = DARKGRAY;

const C_LOCATED: Rgb8 = SILVER;

const C_TMINO: Rgb8 = ROYALBLUE;
const C_SMINO: Rgb8 = CORAL;
const C_ZMINO: Rgb8 = DEEPSKYBLUE;
const C_OMINO: Rgb8 = GREEN;
const C_JMINO: Rgb8 = OLIVE;
const C_LMINO: Rgb8 = SKYBLUE;
const C_IMINO: Rgb8 = ORANGERED;

fn main() {
    nannou::app(model)
        .event(event)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {
    mino: Mino,
    board: Board,
    next_minos: Vec<Mino>,
    fc: i32
}

fn model(_app: &App) -> Model {
    let board = Board::new();
    let mut minos = generate_mino_pool();
    let mino = minos.remove(0);
    Model {
        mino: mino,
        board: board,
        next_minos: minos,
        fc: 0
    }
}

fn event(_app: &App, model: &mut Model, event: Event) {
    match event {
        Event::WindowEvent {
            id: _,
            simple: some_we
        } => {
            if let Some(we) = some_we {
                window_event(model, we);
            }
        }
        _other => {}
    }
}

fn window_event(model: &mut Model, event: WindowEvent) {
    match event {
        KeyPressed(key) => {
            match key {
                Key::A => { model.mino.move_left_if_possible(&model.board) }
                Key::S => { model.mino.move_down_if_possible(&model.board) }
                Key::D => { model.mino.move_right_if_possible(&model.board) }
                Key::W => {
                    model.mino.y += 1;
                }
                Key::R => { model.mino.rotate_if_possible(&model.board) }
                _ => {}
            }
        }
        _other => {}
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    if model.fc % 30 == 0 {
        if !model.mino.can_move_down(&model.board) {
            model.board.put_mino(&model.mino);
            model.mino = get_next_mino(model);
        }
        model.mino.move_down();
    }
    model.fc += 1;
}

fn get_next_mino(model: &mut Model) -> Mino {
    match model.next_minos.pop() {
        Some(mino) => mino,
        None => {
            model.next_minos = generate_mino_pool();
            let mino = model.next_minos.remove(0);
            return mino
        },
    }
}

fn generate_mino_pool() -> Vec<Mino> {
    let mut minos = vec![
        Mino::new(5, 20, 0, Shape::T),
        Mino::new(5, 20, 0, Shape::I),
        Mino::new(5, 20, 0, Shape::J),
        Mino::new(5, 20, 0, Shape::L),
        Mino::new(5, 20, 0, Shape::S),
        Mino::new(5, 20, 0, Shape::Z),
        Mino::new(5, 20, 0, Shape::O)
    ];
    let mut rng = thread_rng();
    minos.shuffle(&mut rng);
    return minos
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(T_BG);
    let draw = app.draw();
    model.board.draw(&draw);
    model.mino.draw(&draw);
    draw.to_frame(app, &frame).unwrap();
}

struct Block {
    x: i32,
    y: i32,
    color: Rgb8
}

impl Block {
    fn new(x: i32, y: i32, color: Rgb8) -> Self {
        Block {
            x: x,
            y: y,
            color: color
        }
    }

    fn draw(&self, draw: &Draw) {
        let loc_x = BLOCK_SIZE * (self.x - BOARD_WIDTH as i32 / 2) as f32;
        let loc_y = BLOCK_SIZE * (self.y - BOARD_HEIGHT as i32 / 2) as f32;
        let block_size = BLOCK_SIZE * 0.95;
        draw.rect()
            .x_y(loc_x, loc_y)
            .w(block_size)
            .h(block_size)
            .color(self.color);
    }

    fn rotate(&self, rot: i32) -> Block {
        let mut x = self.x;
        let mut y = self.y;
        for _ in (0..rot).into_iter() {
            let tmp = x;
            x = -y;
            y = tmp;
        }
        return Block::new(x, y, self.color)
    }

    fn shift(&self, shift_x: i32, shift_y: i32) -> Block {
        return Block::new(self.x + shift_x, self.y + shift_y, self.color)
    }
}

#[derive(Copy, Clone)]
enum Shape {
    T,
    Z,
    S,
    L,
    J,
    O,
    I
}

struct Mino {
    x: i32,
    y: i32,
    rot: i32,
    shape: Shape,
}

impl Mino {
    fn new(x: i32, y: i32, rot: i32, shape: Shape) -> Self {
        Mino {
            x: x,
            y: y,
            rot: rot,
            shape: shape
        }
    }

    fn get_blocks(&self) -> Vec<Block> {
        let blocks = match self.shape {
            Shape::T => vec![
                Block::new(-1, 0, C_TMINO),
                Block::new(0, 0, C_TMINO),
                Block::new(0, -1, C_TMINO),
                Block::new(1, 0, C_TMINO)
            ],
            Shape::S => vec![
                Block::new(-1, -1, C_SMINO),
                Block::new(0, -1, C_SMINO),
                Block::new(0, 0, C_SMINO),
                Block::new(1, 0, C_SMINO)
            ],
            Shape::Z => vec![
                Block::new(-1, 0, C_ZMINO),
                Block::new(0, 0, C_ZMINO),
                Block::new(0, -1, C_ZMINO),
                Block::new(1, -1, C_ZMINO)
            ],
            Shape::J => vec![
                Block::new(-1, -2, C_JMINO),
                Block::new(-1, -1, C_JMINO),
                Block::new(-1, 0, C_JMINO),
                Block::new(0, 0, C_JMINO)
            ],
            Shape::L => vec![
                Block::new(0, -2, C_LMINO),
                Block::new(0, -1, C_LMINO),
                Block::new(-1, 0, C_LMINO),
                Block::new(0, 0, C_LMINO)
            ],
            Shape::O => vec![
                Block::new(0, 0, C_OMINO),
                Block::new(-1, 0, C_OMINO),
                Block::new(0, -1, C_OMINO),
                Block::new(-1, -1, C_OMINO)
            ],
            Shape::I => vec![
                Block::new(-2, 0, C_IMINO),
                Block::new(-1, 0, C_IMINO),
                Block::new(0, 0, C_IMINO),
                Block::new(1, 0, C_IMINO)
            ],
        };
        let rot = (self.rot + 10000) % 4;
        let ret: Vec<Block> = blocks.iter().map(|b| b.rotate(rot)).map(|b| b.shift(self.x, self.y)).collect();
        return ret
    }

    fn can_move_left(&self, board: &Board) -> bool {
        let mino: Mino = Mino::new(self.x - 1, self.y, self.rot, self.shape);
        return mino.get_blocks().iter().all(|b| board.is_blank_at(b.x, b.y))
    }

    fn can_move_right(&self, board: &Board) -> bool {
        let mino: Mino = Mino::new(self.x + 1, self.y, self.rot, self.shape);
        return mino.get_blocks().iter().all(|b| board.is_blank_at(b.x, b.y))
    }

    fn can_move_down(&self, board: &Board) -> bool {
        let mino: Mino = Mino::new(self.x, self.y - 1, self.rot, self.shape);
        return mino.get_blocks().iter().all(|b| board.is_blank_at(b.x, b.y))
    }

    fn can_rotate(&self, board: &Board) -> bool {
        let mino: Mino = Mino::new(self.x, self.y, self.rot + 1, self.shape);
        return mino.get_blocks().iter().all(|b| board.is_blank_at(b.x, b.y))
    }

    fn move_left_if_possible(&mut self, board: &Board) {
        if self.can_move_left(board) {
            self.x -= 1;
        }
    }

    fn move_right_if_possible(&mut self, board: &Board) {
        if self.can_move_right(board) {
            self.x += 1;
        }
    }

    fn move_down_if_possible(&mut self, board: &Board) {
        if self.can_move_down(board) {
            self.y -= 1;
        }
    }

    fn move_down(&mut self) {
        self.y -= 1;
    }

    fn rotate_if_possible(&mut self, board: &Board) {
        if self.can_rotate(board) {
            self.rot += 1;
        }
    }

    fn draw(&self, draw: &Draw) {
        let blocks: Vec<Block> = self.get_blocks();
        for b in blocks {
            b.draw(&draw);
        }
    }
}

struct Board {
    blocks: [[i32; BOARD_WIDTH]; BOARD_HEIGHT]
}

impl Board {
    fn new() -> Self {
        let blocks = [[0; BOARD_WIDTH]; BOARD_HEIGHT];
        Board {
            blocks: blocks
        }
    }

    fn is_blank_at(&self, x: i32, y: i32) -> bool {
        if x < 0 || x > BOARD_WIDTH as i32 - 1 {
            return false
        }
        if y < 0 || y > BOARD_HEIGHT as i32 - 1 {
            return false
        }
        if self.blocks[y as usize][x as usize] > 0 {
            return false
        }
        return true
    }

    fn get_blocks(&self) -> Vec<Block> {
        let mut blocks: Vec<Block> = vec![];
        for x in (0..BOARD_WIDTH).into_iter() {
            for y in (0..BOARD_HEIGHT).into_iter() {
                match self.blocks[y][x] {
                    0 => {
                        blocks.push(Block::new(x as i32, y as i32, T_WHITE))
                    }
                    1 => {
                        blocks.push(Block::new(x as i32, y as i32, C_LOCATED))
                    }
                    _ => ()
                }
            }
        }
        return blocks
    }

    fn put_mino(&mut self, mino: &Mino) {
        for b in mino.get_blocks() {
            self.put_block(&b);
        }
    }

    fn put_block(&mut self, block: &Block) {
        let x = block.x as usize;
        let y = block.y as usize;
        self.blocks[y][x] = 1;
    }

    fn draw(&self, draw: &Draw) {
        let blocks: Vec<Block> = self.get_blocks();
        for b in blocks {
            b.draw(&draw);
        }
    }
}
