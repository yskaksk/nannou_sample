use std::cmp;

use nannou::image;
use nannou::image::{DynamicImage, GenericImageView};
use nannou::prelude::*;

use ndarray::{self, Array2};

struct Model {
    texture: wgpu::Texture,
    width: u32,
    height: u32,
    r_mat_sum: Array2<i32>,
    g_mat_sum: Array2<i32>,
    b_mat_sum: Array2<i32>,
    wsize: u32,
    fc: i32,
}

fn main() {
    nannou::app(model).update(update).view(view).run();
}

fn cumsum(mat: &Array2<u8>) -> Array2<i32> {
    let nrows = mat.nrows();
    let ncols = mat.ncols();
    let mut sum_mat: Array2<i32> = Array2::zeros((nrows, ncols));

    for x in 0..=nrows - 1 {
        for y in 0..=ncols - 1 {
            if (x == 0) && (y == 0) {
                sum_mat[[x as usize, y as usize]] = mat[[x as usize, y as usize]] as i32;
            } else if x == 0 {
                sum_mat[[x as usize, y as usize]] =
                    sum_mat[[x as usize, (y - 1) as usize]] + mat[[x as usize, y as usize]] as i32;
            } else if y == 0 {
                sum_mat[[x as usize, y as usize]] =
                    sum_mat[[(x - 1) as usize, y as usize]] + mat[[x as usize, y as usize]] as i32;
            } else {
                sum_mat[[x as usize, y as usize]] = mat[[x as usize, y as usize]] as i32
                    + sum_mat[[(x - 1) as usize, y as usize]]
                    + sum_mat[[x as usize, (y - 1) as usize]]
                    - sum_mat[[(x - 1) as usize, (y - 1) as usize]];
            }
        }
    }
    return sum_mat;
}

fn model(app: &App) -> Model {
    let assets = app.assets_path().unwrap();
    //let img_path = assets.join("images").join("neko_sample.jpg");
    //let img_path = assets.join("images").join("mountains.jpg");
    let img_path = assets.join("images").join("neko-2.jpg");
    let img = image::open(img_path).unwrap();
    let (img_w, img_h) = img.dimensions();
    app.new_window().size(img_w, img_h).build().unwrap();

    let wsize = 20;

    let ib = img.to_rgb8();

    let mut r_mat: Array2<u8> = Array2::zeros((img_w as usize, img_h as usize));
    let mut g_mat: Array2<u8> = Array2::zeros((img_w as usize, img_h as usize));
    let mut b_mat: Array2<u8> = Array2::zeros((img_w as usize, img_h as usize));
    for x in 0..=img_w - 1 {
        for y in 0..=img_h - 1 {
            let pixel = ib.get_pixel(x, y);
            r_mat[[x as usize, y as usize]] = pixel[0];
            g_mat[[x as usize, y as usize]] = pixel[1];
            b_mat[[x as usize, y as usize]] = pixel[2];
        }
    }

    let r_mat_sum = cumsum(&r_mat);
    let g_mat_sum = cumsum(&g_mat);
    let b_mat_sum = cumsum(&b_mat);

    let texture = wgpu::Texture::from_image(app, &img);

    Model {
        texture: texture,
        width: img_w,
        height: img_h,
        r_mat_sum,
        g_mat_sum,
        b_mat_sum,
        wsize,
        fc: 0,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let r_mat_sum = model.r_mat_sum.clone();
    let g_mat_sum = model.g_mat_sum.clone();
    let b_mat_sum = model.b_mat_sum.clone();
    let img_w = model.width;
    let img_h = model.height;
    if (model.fc % 5 == 4) && (model.wsize > 1) {
        model.wsize -= 1;
    }
    let wsize = model.wsize;
    let img = image::ImageBuffer::from_fn(img_w, img_h, |x, y| {
        let x_min = x.checked_sub(wsize).unwrap_or(0) as usize;
        let x_max = cmp::min(x + wsize, img_w - 1) as usize;
        let y_min = y.checked_sub(wsize).unwrap_or(0) as usize;
        let y_max = cmp::min(y + wsize, img_h - 1) as usize;
        let count = (x_max - x_min) * (y_max - y_min);

        let mut r =
            r_mat_sum[[x_max, y_max]] - r_mat_sum[[x_min, y_max]] - r_mat_sum[[x_max, y_min]]
                + r_mat_sum[[x_min, y_min]];
        let mut g =
            g_mat_sum[[x_max, y_max]] - g_mat_sum[[x_min, y_max]] - g_mat_sum[[x_max, y_min]]
                + g_mat_sum[[x_min, y_min]];
        let mut b =
            b_mat_sum[[x_max, y_max]] - b_mat_sum[[x_min, y_max]] - b_mat_sum[[x_max, y_min]]
                + b_mat_sum[[x_min, y_min]];

        r /= count as i32;
        g /= count as i32;
        b /= count as i32;

        image::Rgb([r as u8, g as u8, b as u8])
    });
    let di = DynamicImage::ImageRgb8(img);
    model.texture = wgpu::Texture::from_image(app, &di);
    model.fc += 1;
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(BLACK);

    let draw = app.draw();
    draw.texture(&model.texture);

    draw.to_frame(app, &frame).unwrap();
}
