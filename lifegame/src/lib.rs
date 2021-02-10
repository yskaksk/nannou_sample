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
    cell_mat: [[i32; WIDTH]; HEIGHT]
}

impl Cells {
    fn new() -> Self {
        let mut mat = [[0; WIDTH]; HEIGHT];
        let mut rng = rand::thread_rng();
        for x in (0..WIDTH).into_iter() {
            for y in (0..HEIGHT).into_iter() {
                if rng.gen_range(0.0, 1.0) < 0.25 {
                    mat[y][x] = 1;
                }
            }
        }
        Cells {
            cell_mat: mat
        }
    }

    fn draw(&self, draw: &Draw) {
        for x_ind in (0..WIDTH).into_iter() {
            for y_ind in (0..HEIGHT).into_iter() {
                let loc_x = CELL_SIZE * (x_ind as i32 - WIDTH as i32 / 2) as f32;
                let loc_y = CELL_SIZE * (y_ind as i32 - HEIGHT as i32 / 2) as f32;
                let block_size = CELL_SIZE * 0.9;
                let color = match self.cell_mat[y_ind][x_ind] {
                    0 => WHITE,
                    1 => BLACK,
                    _ => RED,
                };
                draw.rect()
                    .x_y(loc_x, loc_y)
                    .w(block_size)
                    .h(block_size)
                    .color(color);
            }
        }
    }

    fn next_cell_at(&self, x: usize, y: usize) -> i32 {
        let env_sum = self.cell_mat[(y + HEIGHT - 1) % HEIGHT][(x + WIDTH - 1) % WIDTH]
                    + self.cell_mat[(y + HEIGHT - 1) % HEIGHT][x]
                    + self.cell_mat[(y + HEIGHT - 1) % HEIGHT][(x + 1) % WIDTH]
                    + self.cell_mat[y][(x + WIDTH - 1) % WIDTH]
                    + self.cell_mat[y][(x + 1) % WIDTH]
                    + self.cell_mat[(y + 1) % HEIGHT][(x + WIDTH - 1) % WIDTH]
                    + self.cell_mat[(y + 1) % HEIGHT][x]
                    + self.cell_mat[(y + 1) % HEIGHT][(x  + 1) % WIDTH];
        let next_cell = match env_sum {
            0 | 1 => 0,
            2 => self.cell_mat[y][x],
            3 => 1,
            _other => 0
        };
        return next_cell
    }

    fn next_gen(&mut self) {
        let mut mat = [[0; WIDTH]; HEIGHT];
        for x_ind in (0..WIDTH).into_iter() {
            for y_ind in (0..HEIGHT).into_iter() {
                mat[y_ind][x_ind] = self.next_cell_at(x_ind, y_ind);
            }
        }
        self.cell_mat = mat;
    }
}
