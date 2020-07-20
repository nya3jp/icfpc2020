use rand::prelude::*;
use std::thread;
use std::time::SystemTime;

#[derive(Clone)]
pub struct AnnealingOptions {
    pub time_limit: f64,
    pub max_temp: f64,
    pub min_temp: f64,
    pub seed: u64,
    pub restart: usize,
    pub threads: usize,
    pub silent: bool,
}

impl AnnealingOptions {
    pub fn new(time_limit: f64, max_temp: f64, min_temp: f64) -> Self {
        Self {
            time_limit,
            max_temp,
            min_temp,
            seed: 777,
            restart: 1,
            threads: 1,
            silent: false,
        }
    }
}

pub trait Annealer {
    type State: Clone + Send + Sync;
    type Move;

    fn init_state(&self, rng: &mut impl Rng) -> Self::State;

    fn eval(&self, state: &Self::State) -> f64;

    fn neighbour(&self, state: &Self::State, rng: &mut impl Rng, progress: f64) -> Self::Move;

    fn apply(&self, state: &mut Self::State, mov: &Self::Move);
    fn unapply(&self, state: &mut Self::State, mov: &Self::Move);

    fn apply_and_eval(&self, state: &mut Self::State, mov: &Self::Move, _prev_score: f64) -> f64 {
        self.apply(state, mov);
        self.eval(state)
    }
}

pub fn annealing<A: 'static + Annealer + Clone + Send>(
    annealer: &A,
    opt: &AnnealingOptions,
) -> <A as Annealer>::State {
    assert!(opt.threads > 0);

    if opt.threads == 1 {
        do_annealing(None, annealer, opt).1
    } else {
        let mut ths = vec![];
        let mut rng = StdRng::seed_from_u64(opt.seed);

        for i in 0..opt.threads {
            let a = annealer.clone();
            let mut o = opt.clone();
            o.seed = rng.gen();
            ths.push(thread::spawn(move || do_annealing(Some(i), &a, &o)));
        }

        ths.into_iter()
            .map(|th| th.join().unwrap())
            .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
            .unwrap()
            .1
    }
}

fn do_annealing<A: Annealer>(
    thread_id: Option<usize>,
    annealer: &A,
    opt: &AnnealingOptions,
) -> (f64, <A as Annealer>::State) {
    let mut rng = SmallRng::seed_from_u64(opt.seed);

    let mut state = annealer.init_state(&mut rng);
    let mut cur_score = annealer.eval(&state);
    let mut best_score = cur_score;
    let mut best_ans = state.clone();

    macro_rules! progress {
        ($($arg:expr),*) => {
            if !opt.silent {
                if let Some(tid) = thread_id {
                    eprint!("[{:02}] ", tid);
                }
                eprintln!($($arg),*);
            }
        };
    }

    progress!("Initial score: {}", cur_score);

    let t_max = opt.max_temp;
    let t_min = opt.min_temp;

    let now = SystemTime::now();
    let x = (t_min / t_max).ln();

    let mut progress = 0.0;
    let mut temp = t_max;

    for turn in 0.. {
        if turn % 1000 == 0 {
            let elapsed = now.elapsed().unwrap().as_secs_f64();
            if elapsed >= opt.time_limit {
                progress!("{} steps done.", fmt_number(turn));
                break;
            }

            progress = elapsed / opt.time_limit;
            temp = t_max * (x * progress).exp();
        }

        let mov = annealer.neighbour(&state, &mut rng, progress);
        let new_score = annealer.apply_and_eval(&mut state, &mov, cur_score);

        if new_score <= cur_score || rng.gen_bool(((cur_score - new_score) / temp).exp()) {
            cur_score = new_score;
            if cur_score < best_score {
                if best_score - cur_score > 1e-6 {
                    progress!("Best: score = {:.3}, temp = {:.9}", cur_score, temp);
                }
                best_score = cur_score;
                best_ans = state.clone();

                // FIXME:
                if best_score.abs() < 1e-6 {
                    break;
                }
            }
        } else {
            annealer.unapply(&mut state, &mov);
        }
    }

    (best_score, best_ans)
}

fn fmt_number(n: usize) -> String {
    if n < 1_000 {
        format!("{}", n)
    } else if n < 1_000_000 {
        format!("{:.3}K", n as f64 / 1_000.0)
    } else if n < 1_000_000_000 {
        format!("{:.3}M", n as f64 / 1_000_000.0)
    } else if n < 1_000_000_000_000 {
        format!("{:.3}G", n as f64 / 1_000_000_000.0)
    } else if n < 1_000_000_000_000_000 {
        format!("{:.3}T", n as f64 / 1_000_000_000_000.0)
    } else {
        todo!()
    }
}
