/// ## Ring Mean
/// 
/// Message passing in Rust under collective communication (scatter and reduce) on a ring.
/// 
/// ### Message Passing
/// 
/// Message passing is another framework of concurrent programming other than shared memory.
/// Rust has built-in support for message passing in the standard library, at module `std::sync::mpsc`.
/// 
/// `mpsc` stands for multiple producer, single consumer channel.
/// The producer side, often called *sender* or *transmitter* (`tx`), can be cloned and sent to other threads,
/// thus all senders in Rust standard library are `Send`, `Sync`, and `Clone`.
/// You don't need to wrap them within `Arc`.
/// While the consumer side, often called *receiver* (`rx`), is `Send` but neither `Sync` nor `Clone`.
/// When a message is sent through a MPSC channel, the message is eventually moved to the receiver side.
/// 
/// Rust exposes two types of MPSC channels: `channel` and `sync_channel`.
/// Under the hood, there are three types of them.
/// 
/// - Unbounded channel `channel`.
///   It has a linked list as buffer and can grow indefinitely.
///   The sender side will never block, but the receiver side may block to wait for messages.
/// - Bounded channel `sync_channel` of non-zero buffer size.
///   It has a fixed-size array buffer.
///   The sender side will block if the buffer is full, and the receiver side will block if the buffer is empty.
/// - Rendezvous channel `sync_channel` of zero buffer size.
///   It has no buffer.
///   The sender side will block until the paired receiver is ready to receive the message, and vice versa.
/// 
/// Senders that will never block are of type `Sender<T>`, those that may block are of type `SyncSender<T>`.
/// All receivers are of type `Receiver<T>`.
/// An `iter` method is provided for `Receiver<T>`, which returns an iterator that consumes the received messages.
/// 
/// For all channels, if either side is dropped (*hung up*), operations on the other side will return `Err`.
/// 
/// A common pitfall is to forget to drop the sender side after all messages are sent.
/// 
/// ### Scatter and Reduce
/// 
/// Scatter and reduce are two collective communication operations.
/// 
/// Scattering sends data from one node to all nodes.
/// Each node gets a share of the data.
/// 
/// ```
/// [A B C D] [ ]   [ ]   [ ]
///    [A]    [B]   [C]   [D]
/// ```
/// 
/// In contrast, reducing collects data from all nodes into one node and combine them to a single result.
/// 
/// ```
///    [A]    [B]  [C]   [D]
///   [A+B]   [ ] [C+D]  [ ]
/// [A+B+C+D] [ ]  [ ]   [ ]
/// ```
/// 
/// A very simple (and inefficient) topology for arranging the nodes is a ring.
/// Each node has one receiver from its left neighbor and one sender to its right neighbor.
/// **In the scope of this exercise, we assume that the number of nodes is a power of 2.**
/// The nodes are connected like this:
/// 
/// ```
/// 0 -> 1 -> 2 -> 3
/// ^              v
/// 7 <- 6 <- 5 <- 4
/// ```
/// 
/// ### Quiz
/// 
/// Given an array of integers, calculate the mean of the array concurrently on a ring of threads.
/// 
/// #### Input
/// 
/// The input consists of two lines.
/// The first line contains two integers `M` and `O` separated by a space.
/// 
/// - `M` is the number of threads. Always a power of 2.
/// - `O` is the output type.
///     - `0` for reporting the mean data on each thread after scattering.
///     - `1` for reporting the mean data on the root thread after reducing.
/// 
/// The second line contains `N` integers separated by spaces, where `N` is guaranteed to be a multiple of `M`.
/// 
/// During scattering, the `i`-th thread will receive the `i`-th chunk of `N/M` consecutive integers from the input array.
/// 
/// Example 1:
/// 
/// ```
/// 8 0
/// 76 96 4 15 83 95 25 32 87 43 28 83 26 41 65 55 9 3 87 76 84 54 82 33 46 79 13 31 13 46 98 14 39 41 91 21 51 27 2 76 7 29 50 49 12 99 75 97
/// ```
/// 
/// Example 2:
/// 
/// ```
/// 8 1
/// 76 96 4 15 83 95 25 32 87 43 28 83 26 41 65 55 9 3 87 76 84 54 82 33 46 79 13 31 13 46 98 14 39 41 91 21 51 27 2 76 7 29 50 49 12 99 75 97
/// ```
/// 
/// #### Output
/// 
/// If `O` is `0`, output `M` lines, each line contains the thread number and the mean of the integers received by the thread.
/// 
/// If `O` is `1`, output one line containing the thread number of the root thread (0) and the mean of all integers received by the root thread.
/// 
/// Example 1: Scattering over 8 threads
/// 
/// As an example, 0-th thread will receive 6 integers `[76 96 4 15 83 95]`, and the mean is `61.500`.
/// 
/// ```
/// 0 61.500
/// 1 49.667
/// 2 33.167
/// 3 69.333
/// 4 38.000
/// 5 50.667
/// 6 32.000
/// 7 63.667
/// ```
/// 
/// Example 2: Reducing over 8 threads
/// 
/// ```
/// 0 49.750
/// ```
/// 
/// #### Template
/// 
/// ```no_run
/// #![cfg(not(oj_no_merge))]
/// 
/// 
/// use std::{
///     collections::VecDeque,
///     iter,
///     sync::mpsc::{channel, sync_channel, Receiver, Sender, SyncSender},
///     thread,
/// };
/// 
/// #[derive(Debug)]
/// enum Output {
///     Scatter(f64),
///     Reduce(f64),
/// }
/// 
/// fn task(
///     idx: usize,
///     scatter_tx: Sender<Vec<f64>>,
///     scatter_rx: Receiver<Vec<f64>>,
///     reduce_tx: SyncSender<f64>,
///     reduce_rx: Receiver<f64>,
///     output_tx: Sender<(usize, Output)>,
///     thread_count: usize,
///     test_type: usize,
/// ) {
///     // Data received
///     let buffer: Vec<f64>;
/// 
///     // Number of data to be processed
///     let data_count: usize;
/// 
///     if idx == 0 {
///         todo!("Read input and scatter within root thread") 
///     } else {
///         todo!("Other threads: receive data and continue to scatter to other threads")
///     };
/// 
///     // Number of data the thread will process
///     let data_count = data_count / (thread_count - idx);
/// 
///     // Mean value on the thread
///     let mut mean = buffer.iter().take(data_count).sum::<f64>() / data_count as f64;
/// 
///     // Send the scatter result to the main thread
///     output_tx.send((idx, Output::Scatter(mean))).unwrap();
///     if test_type == 0 {
///         return;
///     }
/// 
///     // HINT: Use a loop
///     todo!("Reduce the mean value from all threads")
/// 
///     // Send the reduce result to the main thread
///     if idx == 0 {
///         output_tx.send((idx, Output::Reduce(mean))).unwrap();
///     }
/// }
/// 
/// /// Read second line input from stdin
/// fn read_input() -> Vec<f64> {
///     let mut buf_stdin = String::new();
///     std::io::stdin().read_line(&mut buf_stdin).unwrap();
///     buf_stdin
///         .trim()
///         .split_whitespace()
///         .map(|x| x.parse::<f64>().unwrap())
///         .collect()
/// }
/// 
/// /// Read first line input from stdin
/// fn read_test_meta() -> (usize, usize) {
///     let mut buf_stdin = String::new();
///     std::io::stdin().read_line(&mut buf_stdin).unwrap();
///     let mut meta = buf_stdin
///         .trim()
///         .split_whitespace()
///         .map(|x| x.parse::<usize>().unwrap());
///     let thread_count = meta.next().unwrap();
///     let test_type = meta.next().unwrap();
///     assert!(thread_count.is_power_of_two());
///     (thread_count, test_type)
/// }
/// 
/// fn main() {
///     // M and O
///     let (thread_count, test_type) = read_test_meta();
/// 
///     // Channels for scatter
///     let (all_tx_scatter, mut all_rx_scatter): (Vec<_>, VecDeque<_>) =
///         iter::repeat_with(channel::<Vec<f64>>)
///             .take(thread_count)
///             .unzip();
///     all_rx_scatter.rotate_right(1);
/// 
///     // Channels for reduce
///     let (all_tx_reduce, mut all_rx_reduce): (Vec<_>, VecDeque<_>) =
///         iter::repeat_with(|| sync_channel::<f64>(0))
///             .take(thread_count)
///             .unzip();
///     all_rx_reduce.rotate_right(1);
/// 
///     // Channel for output
///     let (tx_output, rx_output) = channel::<(usize, Output)>();
/// 
///     // Use scope to spawn threads
///     // So all threads will be joined automatically
///     thread::scope(|s| {
///         for (i, ((tx_scatter, rx_scatter), (tx_reduce, rx_reduce))) in iter::zip(
///             iter::zip(all_tx_scatter, all_rx_scatter),
///             iter::zip(all_tx_reduce, all_rx_reduce),
///         )
///         .enumerate()
///         {
///             // Because the closure will capture the sender,
///             // we need to clone the sender
///             let tx_output = tx_output.clone();
///             s.spawn(move || {
///                 task(
///                     i,
///                     tx_scatter,
///                     rx_scatter,
///                     tx_reduce,
///                     rx_reduce,
///                     tx_output,
///                     thread_count,
///                     test_type,
///                 )
///             });
///         }
///     });
/// 
///     // !IMPORTANT
///     // If don't drop the sender, the receiver will wait forever
///     // This sender handle is on the main thread
///     drop(tx_output);
/// 
///     // Collect the output
///     let mut output: Vec<_> = rx_output
///         .iter()
///         .filter(|(_, o)| match o {
///             Output::Scatter(_) => test_type == 0,
///             Output::Reduce(_) => test_type == 1,
///         })
///         .map(|(idx, o)| {
///             (
///                 idx,
///                 match o {
///                     Output::Scatter(x) => x,
///                     Output::Reduce(x) => x,
///                 },
///             )
///         })
///         .collect();
///     output.sort_by(|(a, _), (b, _)| a.cmp(b));
///     output.iter().for_each(|(idx, x)| println!("{idx} {x:.3}"));
/// }
/// ```

use std::{
    collections::VecDeque,
    iter,
    sync::mpsc::{channel, sync_channel, Receiver, Sender, SyncSender},
    thread,
};

#[derive(Debug)]
enum Output {
    Scatter(f64),
    Reduce(f64),
}

fn task(
    idx: usize,
    scatter_tx: Sender<Vec<f64>>,
    scatter_rx: Receiver<Vec<f64>>,
    reduce_tx: SyncSender<f64>,
    reduce_rx: Receiver<f64>,
    output_tx: Sender<(usize, Output)>,
    thread_count: usize,
    test_type: usize,
) {
    let buffer;
    let data_count;

    if idx == 0 {
        buffer = read_input();
        data_count = buffer.len();
        scatter_tx
            .send(buffer[(data_count / thread_count)..].to_owned())
            .unwrap();
    } else {
        buffer = scatter_rx.recv().unwrap();
        data_count = buffer.len();
        if idx != thread_count - 1 {
            scatter_tx
                .send(buffer[data_count / (thread_count - idx)..].to_owned())
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

    for i in 0..turns {
        let step = 2 << i;
        let step_half = step / 2;

        let should_recv = (idx as i32 + step - 1) % step >= step_half;
        let should_send = (idx as i32) % step >= step_half;

        if should_recv {
            let recv = reduce_rx.recv().unwrap();

            mean = if should_send {
                recv
            } else {
                (recv + mean) / 2.0
            };
        }

        if should_send {
            reduce_tx.send(mean).unwrap();
        }
    }

    if idx == 0 {
        output_tx.send((idx, Output::Reduce(mean))).unwrap();
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

    let (all_tx_scatter, mut all_rx_scatter): (Vec<_>, VecDeque<_>) =
        iter::repeat_with(channel::<Vec<f64>>)
            .take(thread_count)
            .unzip();
    all_rx_scatter.rotate_right(1);

    let (all_tx_reduce, mut all_rx_reduce): (Vec<_>, VecDeque<_>) =
        iter::repeat_with(|| sync_channel::<f64>(0))
            .take(thread_count)
            .unzip();
    all_rx_reduce.rotate_right(1);

    let (tx_output, rx_output) = channel::<(usize, Output)>();

    thread::scope(|s| {
        for (i, ((tx_scatter, rx_scatter), (tx_reduce, rx_reduce))) in iter::zip(
            iter::zip(all_tx_scatter, all_rx_scatter),
            iter::zip(all_tx_reduce, all_rx_reduce),
        )
        .enumerate()
        {
            let tx_output = tx_output.clone();
            s.spawn(move || {
                task(
                    i,
                    tx_scatter,
                    rx_scatter,
                    tx_reduce,
                    rx_reduce,
                    tx_output,
                    thread_count,
                    test_type,
                )
            });
        }
    });

    drop(tx_output);
    let mut output: Vec<_> = rx_output
        .iter()
        .filter(|(_, o)| match o {
            Output::Scatter(_) => test_type == 0,
            Output::Reduce(_) => test_type == 1,
        })
        .map(|(idx, o)| {
            (
                idx,
                match o {
                    Output::Scatter(x) => x,
                    Output::Reduce(x) => x,
                },
            )
        })
        .collect();
    output.sort_by(|(a, _), (b, _)| a.cmp(b));
    output.iter().for_each(|(idx, x)| println!("{idx} {x:.3}"));
}
