use std::assert;
use std::cell::RefCell;
use std::fs::File;
use std::io::{BufRead, BufReader};

const MAX_WIDTH: usize = 30;
const MAX_HIGHT: usize = 30;
const BUF_SIZE: usize = 900;

thread_local!(
    static DIRECTION: RefCell<[Point; 4]> = {
        let d: [Point; 4] = [
            Point { x: 1, y: 0 },
            Point { x: -1, y: 0 },
            Point { x: 0, y: 1 },
            Point { x: 0, y: -1 },
        ];
        RefCell::new(d)
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
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone)]
struct Elem {
    x: i32,
    y: i32,
}

struct Queue {
    head: i32,
    tail: i32,
    size: i32,
    buf: [Elem; BUF_SIZE],
}

// pointを方向に足して、たどり着くpointを返す関数
fn move_point(from: Point, direction: Point) -> Point {
    Point {
        x: from.x + direction.x,
        y: from.y + direction.y,
    }
}

fn reset_queue(queue: &mut Queue) {
    queue.head = 0;
    queue.tail = 0;
    queue.size = 0;
}

fn get_queue_size(queue: &Queue) -> i32 {
    queue.size
}

// queueへの要素追加
fn enqueue(queue: &mut Queue, data: Elem) {
    assert!(queue.size < BUF_SIZE as i32, "Cannot save");
    queue.buf[1 + queue.tail as usize] = data;
    queue.size = queue.size + 1;
    if queue.size as usize == BUF_SIZE {
        queue.tail = 0;
    }
}

// queueへの要素取り出し
fn dequeue(queue: &mut Queue) -> Elem {
    assert!(queue.size > 0);
    let result = queue.buf[queue.head as usize + 1].clone();
    queue.size = queue.size - 1;

    if queue.head == BUF_SIZE as i32 {
        queue.head = 0;
    }
    return result;
}

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
        .trim()
        .split(' ')
        .map(|x: &str| x.parse::<i32>().unwrap());
    let width = numbers.next().unwrap();
    let height = numbers.next().unwrap();
    buf.clear();

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
            .trim()
            .split(' ')
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
        let mut numbers = buf
            .trim()
            .split(' ')
            .map(|x: &str| x.parse::<i32>().unwrap());
        for j in 1..=(width - 1) {
            CAN_GO_Y.with(|y| {
                y.borrow_mut()[i as usize][j as usize] = 1 - numbers.next().unwrap();
            })
        }
        buf.clear();
    }
}

fn main() {}
