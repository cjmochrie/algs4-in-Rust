use rand::{random, Closed01};

pub fn random_sequence(n: i32, lo: f32, hi: f32) {
	let shift = lo;
	let ratio = hi - lo;
	for i in 0..n {
		let Closed01(val) = random::<Closed01<f32>>();
		println!("{}", val * ratio + shift);
	}
}
