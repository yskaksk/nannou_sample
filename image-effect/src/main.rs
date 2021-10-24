use std::cmp;

use nannou::prelude::*;
use nannou::image;
use nannou::image::{GenericImageView, DynamicImage};

use ndarray::{self, Array2};


struct Model {
    texture: wgpu::Texture,
}

fn main() {
    nannou::app(model).run();
}

fn cumsum(mat: &Array2<u8>) -> Array2<i32> {
    let nrows = mat.nrows();
    let ncols = mat.ncols();
    let mut sum_mat: Array2<i32> = Array2::zeros((nrows, ncols));

    for x in 0..=nrows-1 {
        for y in 0..=ncols-1 {
            if (x==0) && (y==0) {
                sum_mat[[x as usize, y as usize]] = mat[[x as usize, y as usize]] as i32;
            } else if x==0 {
                sum_mat[[x as usize, y as usize]] = sum_mat[[x as usize, (y-1) as usize]] + mat[[x as usize, y as usize]] as i32;
            } else if y==0 {
                sum_mat[[x as usize, y as usize]] = sum_mat[[(x-1) as usize, y as usize]] + mat[[x as usize, y as usize]] as i32;
            } else {
                sum_mat[[x as usize, y as usize]] = mat[[x as usize, y as usize]] as i32
                    + sum_mat[[(x-1) as usize, y as usize]]
                    + sum_mat[[x as usize, (y-1) as usize]]
                    - sum_mat[[(x-1) as usize, (y-1) as usize]];
            }
        }
    }
    return sum_mat
}

fn model(app: &App) -> Model {

    let assets = app.assets_path().unwrap();
    //let img_path = assets.join("images").join("neko_sample.jpg");
    let img_path = assets.join("images").join("mountains.jpg");
    let img = image::open(img_path).unwrap();
    let (img_w, img_h) = img.dimensions();
    app.new_window().size(img_w, img_h).view(view).build().unwrap();

    let wsize = 15;

    let ib = img.to_rgb8();

    let mut r_mat: Array2<u8> = Array2::zeros((img_w as usize, img_h as usize));
    let mut g_mat: Array2<u8> = Array2::zeros((img_w as usize, img_h as usize));
    let mut b_mat: Array2<u8> = Array2::zeros((img_w as usize, img_h as usize));
    for x in 0..=img_w-1 {
        for y in 0..=img_h-1 {
            let pixel = ib.get_pixel(x, y);
            r_mat[[x as usize, y as usize]] = pixel[0];
            g_mat[[x as usize, y as usize]] = pixel[1];
            b_mat[[x as usize, y as usize]] = pixel[2];
        }
    }

    let r_mat_sum = cumsum(&r_mat);
    let g_mat_sum = cumsum(&g_mat);
    let b_mat_sum = cumsum(&b_mat);

    let img = image::ImageBuffer::from_fn(img_w, img_h, |x, y| {
        let x_min = x.checked_sub(wsize).unwrap_or(0) as usize;
        let x_max = cmp::min(x+wsize, img_w-1) as usize;
        let y_min = y.checked_sub(wsize).unwrap_or(0) as usize;
        let y_max = cmp::min(y+wsize, img_h-1) as usize;
        let count = (x_max - x_min) * (y_max - y_min);

        let mut r = r_mat_sum[[x_max,y_max]] - r_mat_sum[[x_min,y_max]] - r_mat_sum[[x_max,y_min]] + r_mat_sum[[x_min,y_min]];
        let mut g = g_mat_sum[[x_max,y_max]] - g_mat_sum[[x_min,y_max]] - g_mat_sum[[x_max,y_min]] + g_mat_sum[[x_min,y_min]];
        let mut b = b_mat_sum[[x_max,y_max]] - b_mat_sum[[x_min,y_max]] - b_mat_sum[[x_max,y_min]] + b_mat_sum[[x_min,y_min]];

        r /= count as i32;
        g /= count as i32;
        b /= count as i32;

        image::Rgb([r as u8, g as u8, b as u8])
    });
    let di = DynamicImage::ImageRgb8(img);
    let texture = wgpu::Texture::from_image(app, &di);

    Model {
        texture: texture,
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(BLACK);

    let draw = app.draw();
    draw.texture(&model.texture);

    draw.to_frame(app, &frame).unwrap();
}
