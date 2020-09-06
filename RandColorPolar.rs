use opencv::imgproc::warp_polar;
use opencv::core::{CV_32FC2, Scalar, Mat, MatTrait, Vec3f, DataType};
use std::iter;
use rand::Rng;

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

     let mut output = Mat::new_rows_cols_with_default(300, 300, Vec3f::typ(), Scalar::all(1.23)).unwrap();;


    opencv::imgproc::warp_polar(&mat, &mut output,
                                opencv::core::Size::new(300,
                                                        300),
                                opencv::core::Point2f::new(150.0,
                                                           150.0),
                                255_f64,
                                opencv::imgproc::WARP_INVERSE_MAP).unwrap();


    opencv::highgui::named_window("my_window", opencv::highgui::WINDOW_NORMAL).unwrap();

    opencv::highgui::imshow("my_window", &output).unwrap();

    opencv::highgui::wait_key(22021).unwrap();







}
