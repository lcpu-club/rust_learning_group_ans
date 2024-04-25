use std::{
    cell::RefCell,
    collections::VecDeque,
    iter,
    num::NonZeroUsize,
    sync::{mpsc::{channel, sync_channel, Receiver, Sender, SyncSender}, Arc, Barrier},
    thread,
};

#[derive(Debug)]
enum Output {
    Scatter(f64),
    Gather(f64),
}

fn task(
    idx: usize,
    scatter_tx: Sender<Vec<f64>>,
    scatter_rx: Receiver<Vec<f64>>,
    gather_tx: SyncSender<f64>,
    gather_rx: Receiver<f64>,
    output_tx: Sender<(usize, Output)>,
    thread_count: usize,
    test_type: usize,
) {
    let buffer;
    let data_count;

    if idx == 0 {
        buffer = read_input();
        data_count = buffer.len();
        scatter_tx.send(buffer[(data_count / thread_count)..].to_owned())
            .unwrap();
    } else {
        buffer = scatter_rx.recv().unwrap();
        data_count = buffer.len();
        if idx != thread_count - 1 {
            scatter_tx.send(buffer[data_count / (thread_count - idx)..].to_owned())
                .unwrap();
        }
    };

    let data_count = data_count / (thread_count - idx);
    let mut mean = buffer.iter().take(data_count).sum::<f64>() / data_count as f64;

    output_tx.send((idx, Output::Scatter(mean))).unwrap();
    if test_type == 0 {
        return;
    }

    let turns = (thread_count as f64).log2().ceil() as usize;
    
    dbg!("turns: {}", turns);
    for i in 0..turns {
        let step = 2 << i;
        let step_half = step / 2;

        let should_recv = (idx as i32 + step - 1) % step >= step_half;
        let should_send = (idx as i32) % step >= step_half;
        
        if should_recv {
            dbg!("{}: recv", idx);
            let recv = gather_rx.recv().unwrap();

            mean = if should_send {
                dbg!("{}: -({})->", idx, recv);
                recv
            } else {
                dbg!("{}: ({}, {}), {}", idx, recv, mean, (recv + mean) / 2.0);
                (recv + mean) / 2.0
            };
        }
        
        if should_send {
            dbg!("{}: send", idx);
            gather_tx.send(mean).unwrap();
        }
    }
    
    if idx == 0 {
        output_tx.send((idx, Output::Gather(mean))).unwrap();
    }
}

fn read_input() -> Vec<f64> {
    let mut buf_stdin = String::new();
    std::io::stdin().read_line(&mut buf_stdin).unwrap();
    buf_stdin
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<f64>().unwrap())
        .collect()
}

fn read_test_meta() -> (usize, usize) {
    let mut buf_stdin = String::new();
    std::io::stdin().read_line(&mut buf_stdin).unwrap();
    let mut meta = buf_stdin
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    let thread_count = meta.next().unwrap();
    let test_type = meta.next().unwrap();
    assert!(thread_count.is_power_of_two());
    (thread_count, test_type)
}

fn main() {
    let (thread_count, test_type) = read_test_meta();

    let (all_tx_scatter, mut all_rx_scatter): (Vec<_>, VecDeque<_>) = iter::repeat_with(channel::<Vec<f64>>)
        .take(thread_count)
        .unzip();
    all_rx_scatter.rotate_right(1);

    let (all_tx_gather, mut all_rx_gather): (Vec<_>, VecDeque<_>) = iter::repeat_with(|| sync_channel::<f64>(0))
        .take(thread_count)
        .unzip();
    all_rx_gather.rotate_right(1);

    let (tx_output, rx_output) = channel::<(usize, Output)>();

    thread::scope(|s| {
        for (i, ((tx_scatter, rx_scatter), (tx_gather, rx_gather))) in iter::zip(
            iter::zip(all_tx_scatter, all_rx_scatter),
            iter::zip(all_tx_gather, all_rx_gather),
        ).enumerate() {
            let tx_output = tx_output.clone();
            s.spawn(move || task(i, tx_scatter, rx_scatter, tx_gather, rx_gather, tx_output, thread_count, test_type));
        }
    });

    drop(tx_output);
    let mut output: Vec<_> = rx_output
        .iter()
        .filter(|(_, o)| match o {
            Output::Scatter(_) => test_type == 0,
            Output::Gather(_) => test_type == 1,
        })
        .map(|(idx, o)| (idx, match o {
            Output::Scatter(x) => x,
            Output::Gather(x) => x,
        }))
        .collect();
    output.sort_by(|(a, _), (b, _)| a.cmp(b));
    output.iter().for_each(|(idx, x)| println!("{idx} {x:.3}"));
}
