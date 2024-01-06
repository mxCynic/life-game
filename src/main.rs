use std::process;
use std::thread;
use std::time::Duration;

// use ndarray::prelude::*;
use life_game::{Cell, Universe};

fn main() {
    // 自定义的400个死亡细胞的宇宙
    let test = vec![Cell::Dead; 900];
    let mut test_u = Universe {
        width: 30,
        height: 30,
        cells: test,
    };

    // // this is Boat module 稳定状态
    // test_u.fresh_cell_alive(3, 2);
    // test_u.fresh_cell_alive(4, 2);
    // test_u.fresh_cell_alive(2, 3);
    // test_u.fresh_cell_alive(4, 3);
    // test_u.fresh_cell_alive(2, 4);
    // test_u.fresh_cell_alive(3, 4);

    // this is 滑翔机
    test_u.fresh_cell_alive(3, 2);
    test_u.fresh_cell_alive(3, 3);
    test_u.fresh_cell_alive(3, 4);
    test_u.fresh_cell_alive(2, 4);
    test_u.fresh_cell_alive(1, 3);

    loop {
        print!("{}", test_u);
        thread::sleep(Duration::from_millis(50));
        process::Command::new("clear").status().unwrap();
        test_u.next_frame();
    }

    // 下面是随机宇宙的图像
    // let mut rand_u = Universe::init(30, 30);

    // loop {
    //     print!("{}", rand_u);
    //     thread::sleep(Duration::from_millis(100));
    //     process::Command::new("clear").status().unwrap();
    //     rand_u.next_frame();
    // }
}
