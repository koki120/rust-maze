use std::cell::RefCell;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{assert, env};

const MAX_WIDTH: usize = 30;
const MAX_HIGHT: usize = 30;
const BUF_SIZE: usize = 900;
const DIRECTIONS: [Point; 4] = [
    Point { x: 1, y: 0 },
    Point { x: -1, y: 0 },
    Point { x: 0, y: 1 },
    Point { x: 0, y: -1 },
];

thread_local!(
    static QUEUE: RefCell<Queue> = {
        let q: Queue = Queue {
            head: 0,
            tail: 0,
            size: 0,
            buf: [Point { x: 0, y: 0 }; BUF_SIZE],
        };
        RefCell::new(q)
    };

    // maze information
    static WIDTH: RefCell<i32> = {
        let w = 0;
        RefCell::new(w)
    };
    static HEIGHT: RefCell<i32> = {
        let h = 0;
        RefCell::new(h)
    };
    static CAN_GO_X: RefCell<[[i32; MAX_HIGHT + 2]; MAX_WIDTH + 2]> = {
        let x: [[i32; MAX_HIGHT + 2]; MAX_WIDTH + 2] = [[0; MAX_HIGHT + 2]; MAX_WIDTH + 2];
        RefCell::new(x)
    };
    static CAN_GO_Y: RefCell<[[i32; MAX_HIGHT + 2]; MAX_WIDTH + 2]> = {
        let x: [[i32; MAX_HIGHT + 2]; MAX_WIDTH + 2] = [[0; MAX_HIGHT + 2]; MAX_WIDTH + 2];
        RefCell::new(x)
    };
);

// 迷路の各場所を表現するのに使う
#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

struct Queue {
    head: i32,
    tail: i32,
    size: i32,
    buf: [Point; BUF_SIZE],
}

#[allow(dead_code)]
// pointを方向に足して、たどり着くpointを返す関数
fn move_point(from: Point, direction: Point) -> Point {
    Point {
        x: from.x + direction.x,
        y: from.y + direction.y,
    }
}

fn reset_queue() {
    QUEUE.with(|q| {
        q.borrow_mut().head = 0;
        q.borrow_mut().size = 0;
        q.borrow_mut().tail = 0;
        q.borrow_mut().buf = [Point { x: 0, y: 0 }; BUF_SIZE]
    })
}

// queueへの要素追加
fn enqueue(data: Point) {
    QUEUE.with(|q: &RefCell<Queue>| {
        assert!(q.borrow().size < BUF_SIZE as i32, "Cannot save");
        q.borrow_mut().tail += 1;
        q.borrow_mut().buf[q.borrow().tail as usize] = data;
        q.borrow_mut().size += 1;
        if q.borrow().tail as usize == BUF_SIZE {
            q.borrow_mut().tail = 0;
        }
    });
}

// queueへの要素取り出し
fn dequeue() -> Point {
    QUEUE.with(|q: &RefCell<Queue>| {
        assert!(q.borrow().size > 0);
        let result = q.borrow().buf[q.borrow().head as usize];
        q.borrow_mut().head += 1;
        q.borrow_mut().size -= 1;
        if q.borrow().head == BUF_SIZE as i32 {
            q.borrow_mut().head = 0;
        }
        result
    })
}

#[allow(dead_code)]
fn can_go(from: Point, direction: Point) -> i32 {
    let result;
    if direction.y == 0 && direction.x > 0 {
        result = CAN_GO_X.with(|x: &RefCell<[[i32; MAX_HIGHT + 2]; MAX_WIDTH + 2]>| {
            x.borrow()[1 + from.y as usize][1 + from.x as usize]
        });
    } else if direction.y == 0 {
        result = CAN_GO_X.with(|x: &RefCell<[[i32; MAX_HIGHT + 2]; MAX_WIDTH + 2]>| {
            x.borrow()[1 + from.y as usize][from.x as usize]
        });
    } else if direction.y > 0 {
        result = CAN_GO_Y.with(|y: &RefCell<[[i32; MAX_HIGHT + 2]; MAX_WIDTH + 2]>| {
            y.borrow()[1 + from.y as usize][1 + from.x as usize]
        });
    } else {
        result = CAN_GO_Y.with(|y: &RefCell<[[i32; MAX_HIGHT + 2]; MAX_WIDTH + 2]>| {
            y.borrow()[from.y as usize][1 + from.x as usize]
        })
    }
    result
}

fn setup_board(file: &mut File) {
    let mut reader: BufReader<&mut File> = BufReader::new(file);

    // 入力は"width height/n"を想定
    let mut buf = String::new();
    reader.read_line(&mut buf).unwrap();
    let mut numbers = buf
        .split_whitespace()
        .map(|x: &str| x.parse::<i32>().unwrap());
    WIDTH.with(|w: &RefCell<i32>| *w.borrow_mut() = numbers.next().unwrap());
    HEIGHT.with(|h: &RefCell<i32>| *h.borrow_mut() = numbers.next().unwrap());
    buf.clear();
    let height: i32 = HEIGHT.with(|h| *h.borrow());
    let width: i32 = WIDTH.with(|w| *w.borrow());

    assert!(height > 0 && width > 0);

    // can_goの初期化
    for i in 0..=(height + 1) {
        for j in 0..=(width + 1) {
            CAN_GO_X.with(|x| {
                x.borrow_mut()[i as usize][j as usize] = 0;
            });
            CAN_GO_Y.with(|y| {
                y.borrow_mut()[i as usize][j as usize] = 0;
            })
        }
    }

    for i in 1..=(height) {
        // 横の壁の有無を取得
        reader.read_line(&mut buf).unwrap();
        let mut numbers = buf
            .split_whitespace()
            .map(|x: &str| x.parse::<i32>().unwrap());
        for j in 1..(width) {
            CAN_GO_X.with(|x| {
                x.borrow_mut()[i as usize][j as usize] = 1 - numbers.next().unwrap();
            })
        }
        buf.clear();

        if i == height {
            break;
        };

        // 縦の壁の有無を取得
        reader.read_line(&mut buf).unwrap();
        let mut numbers = buf
            .split_whitespace()
            .map(|x: &str| x.parse::<i32>().unwrap());
        for j in 1..=(width - 1) {
            CAN_GO_Y.with(|y| {
                y.borrow_mut()[i as usize][j as usize] = 1 - numbers.next().unwrap();
            })
        }
        buf.clear();
    }
}

// fn print_board() {
//     let height: i32 = HEIGHT.with(|h| *h.borrow());
//     let width: i32 = WIDTH.with(|w| *w.borrow());
//     let can_go_x = CAN_GO_X.with(|x| *x.borrow());
//     let can_go_y = CAN_GO_Y.with(|x| *x.borrow());

//     for i in 0..=(height + 1) {
//         for j in 0..=(width + 1) {
//             println!("{} ", can_go_x[i as usize][j as usize]);
//         }
//         println!("\n");
//     }
//     println!("-----------\n");
//     for i in 0..=(height + 1) {
//         for j in 0..=(width + 1) {
//             println!("{} ", can_go_y[i as usize][j as usize]);
//         }
//         println!("\n");
//     }
// }

fn solve() -> i32 {
    let mut shortest_path_length: i32 = 0;
    reset_queue();
    // スタート地点をenqueue
    enqueue(Point { x: 0, y: 0 });
    loop {
        let current_locations = QUEUE.with(|q| q.borrow().size);
        if current_locations == 0 {
            break;
        }
        // 行ける場所を確認し、その地点をenqueue
        // 前の地点追加した地点の間に壁を追加
        for _ in 0..current_locations {
            let here = dequeue();
        }

        // enqueueする前のcurrent_locationsをdequeue

        shortest_path_length += 1;
    }
    shortest_path_length
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut input_file = File::open(&args[1]).unwrap();
    setup_board(&mut input_file);
    let res = solve();
    println!("{}", res);
}
