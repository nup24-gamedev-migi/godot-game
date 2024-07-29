
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Table<T> {
    width: usize,
    height: usize,
    mem: Vec<T>, // Invariant: mem.len() == width * height
}

impl<T> Table<T> {
    pub fn new() -> Self {
        Self::from_raw_parts(0, 0, Vec::new())
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn set(&mut self, x: usize, y: usize, v: T) {
        let Some(r) = self.get_mut(x, y)
            else { return; };

        *r = v;
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        let idx = self.xy_to_idx(x, y)?;
        self.mem.get(idx)
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        let idx = self.xy_to_idx(x, y)?;
        self.mem.get_mut(idx)
    }

    fn fill_with<F>(&mut self, f: F)
    where
        F: FnMut() -> T,
    {
        self.mem.fill_with(f)
    }

    fn xy_to_idx(&self, x: usize, y: usize) -> Option<usize> {
        if x >= self.width { return None; }
        if y >= self.height { return None; }

        let idx = self.width * y + x;
        debug_assert!(idx < self.mem.len());

        Some(idx)
    }

    fn from_raw_parts(width: usize, height: usize, mem: Vec<T>) -> Self {
        debug_assert!(mem.len() == width * height);

        Self {
            width,
            height,
            mem,
        }
    }
}

impl<T: Default> Table<T> {
    pub fn new_filled(width: usize, height: usize) -> Self {
        Self::from_raw_parts(
            width,
            height,
            (0..width*height).map(|_| T::default())
                .collect()
        )
    }

    pub fn reset(&mut self) {
        self.fill_with(Default::default)
    }
}

impl<T: Clone> Table<T> {
    pub fn new_filled_with(width: usize, height: usize, v: T) -> Self {
        Self::from_raw_parts(
            width,
            height,
            vec![v; width * height]
        )
    }

    pub fn fill(&mut self, v: T) {
        self.fill_with(|| v.clone())
    }
}

#[cfg(test)]
mod tests {
    use rand::{distributions::Standard, prelude::*};
    use super::Table;
    use std::{collections::HashMap, fmt::Debug};

    /*
        Tested on types:
        * i32
        * bool
        * u8
        * u32
    */

    const BAD_KICK_ROUNDS: usize = 100_000;
    const CREATION_ROUNDS: usize = 10;
    const MODIFICATION_ROUNDS: usize = 3;
    const TABLE_SIZES: [(usize, usize); 17] = [
        (0, 0),
        (1, 1),
        (10, 10),
        (10, 2),
        (2, 10),
        (100, 2),
        (2, 100),
        (11, 34),
        (34, 11),
        (70, 2),
        (2, 70),
        (0, 12),
        (0, 20),
        (0, 50),
        (12, 0),
        (20, 0),
        (50, 0),
    ];

    fn fill_table_randomly<T>(
        table: &mut Table<T>,
        rng: &mut ThreadRng,
        mut store_mods: Option<&mut HashMap<(usize, usize), T>>,
    )
    where
        T: Copy + Default + Eq + Debug,
        Standard: Distribution<T>,
    {
        let width = table.width();
        let height = table.height();

        for _ in 0..(width * height * MODIFICATION_ROUNDS) {
            let (x, y) = (rng.gen_range(0..width), rng.gen_range(0..height));
            let v = rng.r#gen();

            if let Some(store_mods) = store_mods.as_mut() {
                store_mods.insert((x, y), v);
            }

            table.set(x, y, v);
        }
    }

    fn check_table_contents<T, F>(
        table: &mut Table<T>,
        content: F,
    )
    where
        T: Eq + Debug,
        F: Fn(usize, usize) -> T,
    {
        for x in 0..table.width {
            for y in 0..table.height {
                let v = table.get(x, y).expect("Must be in range");
                assert_eq!(v, &content(x, y));

                let v = table.get_mut(x, y).expect("Must be in range");
                assert_eq!(v, &content(x, y));
            }
        }
    }

    #[test]
    fn test_new() {
        Table::<i32>::new();
        Table::<bool>::new();
        Table::<u8>::new();
        Table::<u32>::new();
    }

    fn test_new_filled_impl<T>()
    where
        T: Default + Eq + Debug,
    {
        for (width, height) in TABLE_SIZES {
            let mut table = Table::<T>::new_filled(width, height);
            check_table_contents(&mut table, |_, _| T::default());
        }
    }

    #[test]
    fn test_new_filled() {
        test_new_filled_impl::<i32>();
        test_new_filled_impl::<bool>();
        test_new_filled_impl::<u8>();
        test_new_filled_impl::<u32>();
    }

    fn test_set_vals_impl<T>(rng: &mut ThreadRng)
    where
        T: Copy + Default + Eq + Debug,
        Standard: Distribution<T>,
    {
        for (width, height) in TABLE_SIZES {
            let mut store_mods = HashMap::new();
            let mut table = Table::<T>::new_filled(width, height);

            fill_table_randomly(&mut table, rng, Some(&mut store_mods));
            check_table_contents(&mut table, |x, y| match store_mods.get(&(x,y)) {
                Some(x) => *x,
                None => T::default(),
            });
        }
    }

    #[test]
    fn test_set_vals() {
        let mut rng = ThreadRng::default();
        test_set_vals_impl::<i32>(&mut rng);
        test_set_vals_impl::<bool>(&mut rng);
        test_set_vals_impl::<u8>(&mut rng);
        test_set_vals_impl::<u32>(&mut rng);
    }

    fn test_new_filled_with_impl<T>(rng: &mut ThreadRng)
    where
        T: Eq + Debug + Copy,
        Standard: Distribution<T>,
    {
        for (width, height) in TABLE_SIZES {
            for _ in 0..CREATION_ROUNDS {
                let x = rng.r#gen();
                let mut table = Table::<T>::new_filled_with(width, height, x);
                check_table_contents(&mut table, |_, _| x);
            }
        }
    }

    #[test]
    fn test_new_filled_with() {
        let mut rng = ThreadRng::default();
        test_new_filled_with_impl::<i32>(&mut rng);
        test_new_filled_with_impl::<bool>(&mut rng);
        test_new_filled_with_impl::<u8>(&mut rng);
        test_new_filled_with_impl::<u32>(&mut rng);
    }

    fn test_fill_with_impl<T>(rng: &mut ThreadRng)
    where
        T: Eq + Debug + Copy + Default,
        Standard: Distribution<T>,
    {
        for (width, height) in TABLE_SIZES {
            let mut table = Table::<T>::new_filled(width, height);
            for _ in 0..CREATION_ROUNDS {
                let x = rng.r#gen();
                table.fill(x);
                check_table_contents(&mut table, |_, _| x);
            }
        }
    }

    #[test]
    fn test_fill_with() {
        let mut rng = ThreadRng::default();
        test_fill_with_impl::<i32>(&mut rng);
        test_fill_with_impl::<bool>(&mut rng);
        test_fill_with_impl::<u8>(&mut rng);
        test_fill_with_impl::<u32>(&mut rng);
    }

    fn test_bad_gets_impl<T>(rng: &mut ThreadRng)
    where
        T: Eq + Debug + Copy + Default,
        Standard: Distribution<T>,
    {
        for (width, height) in TABLE_SIZES {
            for _ in 0..CREATION_ROUNDS {
                let mut table: Table<_> = Table::new_filled(width, height);
                fill_table_randomly(&mut table, rng, None);

                for _ in 0..BAD_KICK_ROUNDS {
                    let (x, y) = (rng.gen_range(width..usize::MAX), rng.gen_range(height..usize::MAX));
                    assert!(table.get(x, y).is_none(), "table.get({x}, {y})");
                    assert!(table.get_mut(x, y).is_none(), "table.get_mut({x}, {y})");
                }
            }
        }
    }

    #[test]
    fn test_bad_gets() {
        let mut rng = ThreadRng::default();
        test_bad_gets_impl::<i32>(&mut rng);
        test_bad_gets_impl::<bool>(&mut rng);
        test_bad_gets_impl::<u8>(&mut rng);
        test_bad_gets_impl::<u32>(&mut rng);
    }

    fn test_bad_sets_impl<T>(rng: &mut ThreadRng)
    where
        T: Eq + Debug + Copy + Default,
        Standard: Distribution<T>,
    {
        for (width, height) in TABLE_SIZES {
            for _ in 0..CREATION_ROUNDS {
                let mut table: Table<_> = Table::new_filled(width, height);
                fill_table_randomly(&mut table, rng, None);

                for _ in 0..BAD_KICK_ROUNDS {
                    let (x, y) = (rng.gen_range(width..usize::MAX), rng.gen_range(height..usize::MAX));
                    let v = rng.r#gen();

                    table.set(x, y, v);
                }
            }
        }
    }

    #[test]
    fn test_bad_sets() {
        let mut rng = ThreadRng::default();
        test_bad_sets_impl::<i32>(&mut rng);
        test_bad_sets_impl::<bool>(&mut rng);
        test_bad_sets_impl::<u8>(&mut rng);
        test_bad_sets_impl::<u32>(&mut rng);
    }
}