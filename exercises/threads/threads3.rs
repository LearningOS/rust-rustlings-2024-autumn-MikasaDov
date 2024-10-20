// threads3.rs
//
// Execute `rustlings hint threads3` or use the `hint` watch subcommand for a
// hint.



// use std::sync::mpsc;
// use std::sync::Arc;
// use std::thread;
// use std::time::Duration;

// struct Queue {
//     length: u32,
//     first_half: Vec<u32>,
//     second_half: Vec<u32>,
// }

// impl Queue {
//     fn new() -> Self {
//         Queue {
//             length: 10,
//             first_half: vec![1, 2, 3, 4, 5],
//             second_half: vec![6, 7, 8, 9, 10],
//         }
//     }
// }

// fn send_tx(q: Queue, tx: mpsc::Sender<u32>) -> () {
//     let qc = Arc::new(q);
//     let qc1 = Arc::clone(&qc);
//     let qc2 = Arc::clone(&qc);

//     thread::spawn(move || {
//         for val in &qc1.first_half {
//             println!("sending {:?}", val);
//             tx.send(*val).unwrap();
//             thread::sleep(Duration::from_secs(1));
//         }
//     });

//     thread::spawn(move || {
//         for val in &qc2.second_half {
//             println!("sending {:?}", val);
//             tx.send(*val).unwrap();
//             thread::sleep(Duration::from_secs(1));
//         }
//     });
// }

// fn main() {
//     let (tx, rx) = mpsc::channel();
//     let queue = Queue::new();
//     let queue_length = queue.length;

//     send_tx(queue, tx);

//     let mut total_received: u32 = 0;
//     for received in rx {
//         println!("Got: {}", received);
//         total_received += 1;
//     }

//     println!("total numbers received: {}", total_received);
//     assert_eq!(total_received, queue_length)
// }
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Queue {
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

fn send_tx(q: Queue, tx: Arc<Mutex<mpsc::Sender<u32>>>) {
    let qc = Arc::new(q);
    let qc1 = Arc::clone(&qc);
    let qc2 = Arc::clone(&qc);
    let tx1 = Arc::clone(&tx);
    let tx2 = Arc::clone(&tx);

    // 第一个线程发送 first_half 的数据
    thread::spawn(move || {
        for val in &qc1.first_half {
            println!("sending {:?}", val);
            tx1.lock().unwrap().send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // 第二个线程发送 second_half 的数据
    thread::spawn(move || {
        for val in &qc2.second_half {
            println!("sending {:?}", val);
            tx2.lock().unwrap().send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let queue = Queue::new();
    let queue_length = queue.length;

    // 将 Sender 包裹在 Arc 和 Mutex 中，使其在线程间共享
    let tx = Arc::new(Mutex::new(tx));

    // 启动线程来发送数据
    send_tx(queue, Arc::clone(&tx));

    let mut total_received: u32 = 0;
    for received in rx {
        println!("Got: {}", received);
        total_received += 1;

        // 当收到所有消息后，退出循环
        if total_received == queue_length {
            break;
        }
    }

    println!("total numbers received: {}", total_received);
    assert_eq!(total_received, queue_length);
}
