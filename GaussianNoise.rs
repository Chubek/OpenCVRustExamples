use opencv::imgproc::warp_polar;
use opencv::core::{CV_32FC2, Scalar, Mat, MatTrait, Vec3f, DataType, MatExprTrait};
use std::iter;
use rand::Rng;
use rand_distr::StandardNormal;


fn main() {
    let mut mat = Mat::new_rows_cols_with_default(300, 300, Vec3f::typ(), Scalar::all(1.23)).unwrap();
    let mut rng = rand::thread_rng();

    for i in 0..150 {
        for j in 0..150 {
            let red = rng.gen_range(0, 200) as f32 / 100.00;
            let green = rng.gen_range(0, 150) as f32 / 100.00;
            let blue = rng.gen_range(0, 100) as f32 / 100.0;
            let eq = rng.gen_range(0, 50) as f32 / 100.0;

            println!("({}, {}, {}. {})", red, green, blue, eq);

            *mat.at_2d_mut::<Vec3f>(i, j).unwrap() = Vec3f {
                0: [red,
                    eq,
                    blue]
            };
            *mat.at_2d_mut::<Vec3f>(i, j + 150).unwrap() = Vec3f {
                0: [green,
                    red,
                    blue]
            };
            *mat.at_2d_mut::<Vec3f>(i + 150, j).unwrap() = Vec3f {
                0: [blue,
                    green,
                    red]
            };
            *mat.at_2d_mut::<Vec3f>(i + 150, i + 150).unwrap() = Vec3f {
                0: [red,
                    blue,
                    green]
            };
        }
    }

   
    let output = gaussian_noise(&mat);

    opencv::highgui::named_window("my_window", opencv::highgui::WINDOW_NORMAL).unwrap();

    opencv::highgui::imshow("my_window", &output).unwrap();

    opencv::highgui::wait_key(22021).unwrap();
}

fn gaussian_noise(image: &Mat) -> Mat {
    let rows_size = image.rows();
    let cols_size = image.cols();
    let channels = 3;

    let mut output_mat = Mat::new_rows_cols_with_default(rows_size, cols_size,
                                                         Vec3f::typ(), Scalar::all(1.23))
        .unwrap();


    for i in 0..rows_size {
        for j in 0..cols_size {
            *output_mat.at_2d_mut::<Vec3f>(i, j).unwrap() = Vec3f {
                0: [rand::thread_rng().sample(StandardNormal),
                    rand::thread_rng().sample(StandardNormal),
                    rand::thread_rng().sample(StandardNormal)]
            };
        }
    }


    let return_mat = image.mul(&output_mat, 1.0).unwrap().to_mat().unwrap();

    return_mat
}
