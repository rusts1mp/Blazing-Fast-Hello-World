use std::sync::atomic::{AtomicUsize, Ordering};
use std::io::Write;
// i used rayon so it's blazing fast 🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀
use rayon::prelude::*;

static INDEX: AtomicUsize = AtomicUsize::new(0);
static MSG: &[u8] = b"Hello, world!";

fn main() {
    (0..MSG.len()).into_par_iter().for_each(|_| {
        let i = INDEX.fetch_add(1, Ordering::Relaxed);
        let c = &MSG[i..][..1];
        std::io::stdout().lock().write_all(c).unwrap();
    })
}
