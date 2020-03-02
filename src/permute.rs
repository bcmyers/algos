use crate::error::Error;

pub trait Permute {
    type Error;

    fn permute(&mut self, permutation: &mut [usize]) -> Result<(), Self::Error>;
}

/// # Examples
///
/// ```
/// use algos::Permute;
/// # use algos::Error;
///
/// # fn example() -> Result<(), Error> {
/// let mut array = [
///     'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
///     'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
///     'q', 'r', 's', 't',
/// ];
/// let mut permutation = [
///       2,  13,   1,   5,   3,  15,  14,  12,
///       8,  10,   4,  19,  16,  11,   9,   7,
///      18,   6,  17,   0,
/// ];
/// let expected_array = [
///     't', 'c', 'a', 'e', 'k', 'd', 'r', 'p',
///     'i', 'o', 'j', 'n', 'h', 'b', 'g', 'f',
///     'm', 's', 'q', 'l',
/// ];
/// let expected_permutation = permutation.clone();
///
/// array.permute(&mut permutation[..])?;
///
/// assert_eq!(array, expected_array);
/// assert_eq!(permutation, expected_permutation);
///
/// # Ok(()) }
/// ```
impl<T> Permute for [T]
where
    T: Copy,
{
    type Error = Error;

    fn permute(&mut self, permutation: &mut [usize]) -> Result<(), Self::Error> {
        let len = self.len();
        if len != permutation.len() {
            bail!("length of permutation slice must equal length of array.");
        }

        for i in 0..len {
            if (permutation[i] as isize) < 0 {
                permutation[i] = !permutation[i];
                continue;
            }

            let mut v = self[i];
            let mut j = permutation[i];

            while j != i {
                let tmp = self[j];
                self[j] = v;
                v = tmp;

                let tmp = permutation[j];
                permutation[j] = !permutation[j];
                j = tmp;
            }
            self[i] = v;
        }

        Ok(())
    }
}
