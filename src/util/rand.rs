use rand::Rng;

pub fn shuffle<R, T>(rand: &mut R, vec: &mut Vec<T>)
where
    R: Rng,
{
    for i in (1..vec.len()).rev() {
        let r = rand.gen::<usize>() % i;
        vec.swap(i, r);
    }
}
