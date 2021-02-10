use nannou::prelude::*;
use nannou::rand;
use nannou::rand::Rng;

const HEIGHT: usize = 50;
const WIDTH: usize = 80;

const CELL_SIZE: f32 = 10.0;

pub struct Model {
    cells: Cells,
}

pub fn model(_app: &App) -> Model {
    let cells = Cells::new();
    Model {
        cells: cells,
    }
}

pub fn event(_app: &App, _model: &mut Model, _event: Event) {
}

pub fn update(_app: &App, model: &mut Model, _update: Update) {
    model.cells.next_gen();
}

pub fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(DARKGRAY);
    let draw = app.draw();
    model.cells.draw(&draw);
    draw.to_frame(app, &frame).unwrap();
}

struct Cells {
    r_mat: [[i32; WIDTH]; HEIGHT],
    g_mat: [[i32; WIDTH]; HEIGHT],
    b_mat: [[i32; WIDTH]; HEIGHT]
}

impl Cells {
    fn new() -> Self {
        let mut r_mat = [[0; WIDTH]; HEIGHT];
        let mut g_mat = [[0; WIDTH]; HEIGHT];
        let mut b_mat = [[0; WIDTH]; HEIGHT];
        let mut rng = rand::thread_rng();
        for x in (0..WIDTH).into_iter() {
            for y in (0..HEIGHT).into_iter() {
                if rng.gen_range(0.0, 1.0) < 0.4 {
                    r_mat[y][x] = 1;
                    g_mat[(y + 1) % HEIGHT][(x + 1) % WIDTH] = 1;
                    b_mat[(y + 2) % HEIGHT][(x + 2) % WIDTH] = 1;
                }
            }
        }
        Cells {
            r_mat: r_mat,
            g_mat: g_mat,
            b_mat: b_mat,
        }
    }

    fn draw(&self, draw: &Draw) {
        for x_ind in (0..WIDTH).into_iter() {
            for y_ind in (0..HEIGHT).into_iter() {
                let loc_x = CELL_SIZE * (x_ind as i32 - WIDTH as i32 / 2) as f32;
                let loc_y = CELL_SIZE * (y_ind as i32 - HEIGHT as i32 / 2) as f32;
                let block_size = CELL_SIZE * 0.9;

                let r = self.r_mat[y_ind][x_ind];
                let g = self.g_mat[y_ind][x_ind];
                let b = self.b_mat[y_ind][x_ind];
                let color = rgba(
                    244.0 * (1 - r) as f32 / 255.0,
                    244.0 * (1 - g) as f32 / 255.0,
                    244.0 * (1 - b) as f32 / 255.0,
                    1.0
                );
                draw.rect()
                    .x_y(loc_x, loc_y)
                    .w(block_size)
                    .h(block_size)
                    .color(color);
            }
        }
    }

    fn next_cell_at(x: usize, y: usize, cell_mat: &[[i32; WIDTH]; HEIGHT]) -> i32 {
        let env_sum = cell_mat[(y + HEIGHT - 1) % HEIGHT][(x + WIDTH - 1) % WIDTH]
                    + cell_mat[(y + HEIGHT - 1) % HEIGHT][x]
                    + cell_mat[(y + HEIGHT - 1) % HEIGHT][(x + 1) % WIDTH]
                    + cell_mat[y][(x + WIDTH - 1) % WIDTH]
                    + cell_mat[y][(x + 1) % WIDTH]
                    + cell_mat[(y + 1) % HEIGHT][(x + WIDTH - 1) % WIDTH]
                    + cell_mat[(y + 1) % HEIGHT][x]
                    + cell_mat[(y + 1) % HEIGHT][(x  + 1) % WIDTH];
        let next_cell = match env_sum {
            0 | 1 => 0,
            2 => cell_mat[y][x],
            3 => 1,
            _other => 0
        };
        return next_cell
    }

    fn next_gen(&mut self) {
        let mut r_mat = [[0; WIDTH]; HEIGHT];
        let mut g_mat = [[0; WIDTH]; HEIGHT];
        let mut b_mat = [[0; WIDTH]; HEIGHT];
        for x_ind in (0..WIDTH).into_iter() {
            for y_ind in (0..HEIGHT).into_iter() {
                r_mat[y_ind][x_ind] = Cells::next_cell_at(x_ind, y_ind, &self.r_mat);
                g_mat[y_ind][x_ind] = Cells::next_cell_at(x_ind, y_ind, &self.g_mat);
                b_mat[y_ind][x_ind] = Cells::next_cell_at(x_ind, y_ind, &self.b_mat);
            }
        }
        self.r_mat = r_mat;
        self.g_mat = g_mat;
        self.b_mat = b_mat;
    }
}
