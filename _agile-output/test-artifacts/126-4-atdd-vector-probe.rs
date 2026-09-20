use std::collections::VecDeque;
const SEEDS: [u64; 3] = [0x1264_1950_a551, 0xdead_beef_8012, 0x7359_2401_ffff];
#[derive(Clone, Copy, Debug, Default)]
struct Input {
    rst: bool,
    flush: bool,
    valid: bool,
    data: u64,
    ready: bool,
}
#[derive(Clone, Copy, Debug)]
struct Output {
    ready: bool,
    data: Option<u64>,
}
#[derive(Clone, Debug)]
struct Frame {
    input: Input,
    before: Output,
    after: Output,
}
fn next(seed: &mut u64) -> u64 {
    *seed ^= *seed << 13;
    *seed ^= *seed >> 7;
    *seed ^= *seed << 17;
    *seed
}
fn mask(width: u32) -> u64 {
    u64::MAX >> (64 - width)
}
fn output(q: &VecDeque<u64>, depth: usize) -> Output {
    Output {
        ready: q.len() < depth,
        data: q.front().copied(),
    }
}

// This queue is an independent transaction model, with no knowledge of RTL registers.
fn trace(width: u32, depth: usize, initial_seed: u64) -> Vec<Frame> {
    let mut seed = initial_seed;
    let mut q = VecDeque::new();
    let mut frames = Vec::new();
    let (mut accepted, mut delivered, mut cancelled) = (0usize, 0usize, 0usize);
    let (mut epoch_in, mut epoch_out) = (0usize, 0usize);
    let mut pending = None;
    let mut occupancy = vec![0; depth + 1];
    let (mut seen_full, mut full_drains) = (false, 0usize);
    let (mut simultaneous, mut full_release, mut reset_live, mut flush_live, mut overlap) =
        (0, 0, 0, 0, 0);
    let (
        mut stall_run,
        mut longest_stall,
        mut high_delivered,
        mut throughput_run,
        mut longest_throughput,
    ) = (0, 0, 0, 0, 0);
    for cycle in 0..420 {
        let before = output(&q, depth);
        occupancy[q.len()] += 1;
        let draining = cycle >= 400 || (180..200).contains(&cycle) || (300..320).contains(&cycle);
        let rst = matches!(cycle, 0 | 72 | 73 | 201);
        let flush = matches!(cycle, 64 | 73 | 74 | 160 | 202);
        let ready = if draining {
            true
        } else if cycle < 40 || (60..76).contains(&cycle) || (140..180).contains(&cycle) || (260..300).contains(&cycle) {
            false
        } else if cycle < 120 {
            true
        } else {
            next(&mut seed) & 3 != 0
        };
        if pending.is_none() && !draining && (cycle < 120 || (140..180).contains(&cycle) || (260..300).contains(&cycle) || next(&mut seed) & 3 != 0) {
            // Initial directed words exercise bit 63; later words vary every bit.
            let high = if cycle < 3 { 1u64 << (width - 1) } else { 0 };
            pending = Some((next(&mut seed) | high) & mask(width));
        }
        let input = Input {
            rst,
            flush,
            valid: pending.is_some(),
            data: pending.unwrap_or_else(|| next(&mut seed) & mask(width)),
            ready,
        };
        let pop = before.data.is_some() && ready && !rst && !flush;
        let push = input.valid && before.ready && !rst && !flush;
        if rst || flush {
            assert_eq!(
                epoch_in,
                epoch_out + q.len(),
                "epoch conservation before cancellation"
            );
            cancelled += q.len();
            reset_live += usize::from(rst && !q.is_empty());
            flush_live += usize::from(flush && !rst && !q.is_empty());
            overlap += usize::from(rst && flush);
            q.clear();
            seen_full = false;
            pending = None;
            epoch_in = 0;
            epoch_out = 0;
        } else {
            simultaneous += usize::from(pop && push);
            full_release += usize::from(q.len() == depth && pop && input.valid && !push);
            if pop {
                let word = q.pop_front().unwrap();
                high_delivered += usize::from(word & (1u64 << (width - 1)) != 0);
                delivered += 1;
                epoch_out += 1;
            }
            if push {
                q.push_back(input.data);
                accepted += 1;
                epoch_in += 1;
                pending = None;
            }
        }
        if before.data.is_some() && !ready && !rst && !flush {
            stall_run += 1;
        } else {
            stall_run = 0;
        }
        longest_stall = longest_stall.max(stall_run);
        if pop && push {
            throughput_run += 1;
        } else {
            throughput_run = 0;
        }
        longest_throughput = longest_throughput.max(throughput_run);
        if q.len() == depth { seen_full = true; }
        if seen_full && q.is_empty() { full_drains += 1; seen_full = false; }
        assert_eq!(accepted, delivered + cancelled + q.len());
        assert_eq!(epoch_in, epoch_out + q.len());
        frames.push(Frame {
            input,
            before,
            after: output(&q, depth),
        });
    }
    assert!(
        q.is_empty() && pending.is_none(),
        "must fully drain producer and DUT"
    );
    assert_eq!(accepted, delivered + cancelled);
    assert!(full_drains >= 2, "must repeatedly fill and drain: {full_drains}");
    assert!(occupancy.iter().all(|n| *n > 0));
    assert!(
        (if depth == 1 {
            simultaneous == 0
        } else {
            simultaneous > 0
        }) && full_release > 0
            && reset_live > 0
            && flush_live > 0
            && overlap > 0
    );
    assert!(longest_stall >= 30 && (depth == 1 || longest_throughput >= 20) && high_delivered > 0);
    // Enforce valid/data hold under backpressure, unless an edge cancels the producer epoch.
    for pair in frames.windows(2) {
        let a = &pair[0];
        let b = &pair[1];
        if a.input.valid && !a.before.ready && !a.input.rst && !a.input.flush {
            assert!(b.input.valid);
            assert_eq!(a.input.data, b.input.data);
        }
    }
    println!(
        "FR195 width={width} depth={depth} seed={initial_seed:#x} accepted={accepted} delivered={delivered} cancelled={cancelled} occupancy={occupancy:?} simultaneous={simultaneous} full_release={full_release} reset_live={reset_live} flush_live={flush_live} overlap={overlap} longest_stall={longest_stall} longest_throughput={longest_throughput} full_drains={full_drains}"
    );
    frames
}

fn main(){for w in [1,8,32,64]{for d in [1,2,3,4,7,16]{for seed in SEEDS{trace(w,d,seed);}}}}