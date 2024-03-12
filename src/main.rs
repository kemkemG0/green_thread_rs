mod green;

fn producer() {
    let id = green::spawn(consumer, 2 * 1024 * 1024);
    for i in 0..10 {
        println!("Produce: {}", i);
        green::send(id, i);
    }
}

fn consumer() {
    for _ in 0..10 {
        let msg = green::recv().unwrap();
        println!("Consume: {}", msg);
    }
}

fn actor() {
    green::spawn_from_main(producer, 2 * 1024 * 1024);
}

// fn mash() {
//     green::spawn(ortega, 2 * 1024 * 1024);
//     for i in 0..10 {
//         println!("Mash: {}", i);
//         green::schedule();
//     }
// }

// fn ortega() {
//     for _ in 0..10 {
//         println!("Ortega");
//         green::schedule();
//     }
// }

// fn gaia() {
//     green::spawn(mash, 2 * 1024 * 1024);
//     for i in 0..10 {
//         println!("Gaia: {}", i);
//         green::schedule();
//     }
// }

fn main() {
    actor();
}
