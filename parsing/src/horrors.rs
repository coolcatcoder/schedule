macro_rules! type_const {
    ($($any:tt)*) => {
        type const $($any)*;
    };
}

type_const!(A_MINUS_B_MINUS_C<const A: usize, const B: usize, const C: usize>: usize = const { A - B - C });

//type const A_MINUS_B_MINUS_C<const A: usize, const B: usize, const C: usize>: usize = const { A - B - C };

fn extract<const IDX: usize, T, const LEN: usize>(
    arr: [T; LEN],
) -> ([T; IDX], [T; 1], [T; A_MINUS_B_MINUS_C::<LEN, IDX, 1>]) {
    let mut arr = ::core::mem::ManuallyDrop::new(arr);
    let ptr = arr.as_mut_ptr();
    unsafe {
        (
            <*const _>::read(ptr.add(0).cast()),
            <*const _>::read(ptr.add(IDX).cast()),
            <*const _>::read(ptr.add(IDX + 1).cast()),
        )
    }
}

pub fn main() {
    let ss: [String; 6] = ::core::array::from_fn(|i| i.to_string());
    // let [_0, _1, _2, s, _tl @ ..] = ss;
    let (_, [s], _) = dbg!(extract::<3, _, _>(ss));
    dbg!(s);
}

type_const!(DISGUISE<const A: usize>: usize = const { A });

fn weird() {
    let array: [i32; DISGUISE::<2>] = [0; DISGUISE::<2>];
    let [one, two] = array;
}
