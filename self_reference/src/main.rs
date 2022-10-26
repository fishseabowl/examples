fn main() {
    let mut test1 = Test::new("test1");
    test1.init();
    let mut test2 = Test::new("test2");
    test2.init();

    println!("a: {}, b: {}", test1.a(), test1.b());
    std::mem::swap(&mut test1, &mut test2);
    println!("a: {}, b: {}", test2.a(), test2.b());

    let squared: Vec<usize> = (1..10).map(|x| x * x).collect();
    println!("{:?}", squared);

    let squared = (1..10).fold(1, |total, next| total * next);
    println!("{:?}", squared);

    let squared: Vec<usize> = (1..10).filter(|x| *x > 5).collect();
    println!("{:?}", squared);

    let squared = (1..)
        .filter(|x| *x > 5)
        .take(5)
        .map(|x| x * x)
        .fold(1, |sum: u64, next| sum + next);
    println!("{:?}", squared);
}

#[derive(Debug)]
struct Test {
    a: String,
    b: *const String,
}

impl Test {
    fn new(txt: &str) -> Self {
        Test {
            a: String::from(txt),
            b: std::ptr::null(),
        }
    }

    // We need an `init` method to actually set our self-reference
    fn init(&mut self) {
        let self_ref: *const String = &self.a;
        self.b = self_ref;
    }

    fn a(&self) -> &str {
        &self.a
    }

    fn b(&self) -> &String {
        assert!(
            !self.b.is_null(),
            "Test::b called without Test::init being called first"
        );
        unsafe { &*(self.b) }
    }
}

pub struct FlatMap<I, F, B>
where
    I: Iterator,
    F: FnMut(I::Item) -> B,
    B: IntoIterator,
{
    iter: I,
    f: F,
    inner: Option<B::IntoIter>,
}

impl<I, F, B> FlatMap<I, F, B>
where
    I: Iterator,
    F: FnMut(I::Item) -> B,
    B: IntoIterator,
{
    fn new(iter: I, f: F) -> Self {
        Self {
            iter,
            f,
            inner: None,
        }
    }
}

pub fn flat_map<I, F, B>(iter: I, f: F) -> FlatMap<I, F, B>
where
    I: Iterator,
    F: FnMut(I::Item) -> B,
    B: IntoIterator,
{
    FlatMap::new(iter, f)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(flat_map(std::iter::empty(), |x: Vec<()>| { x }).count(), 0);
    }

    #[test]
    fn simple() {
        assert_eq!(
            flat_map(vec!["a", "b"].into_iter(), |x| { x.chars() }).count(),
            2
        );
    }

    #[test]
    fn simple_wide() {
        assert_eq!(
            flat_map(vec!["al", "bet"].into_iter(), |x| x.chars()).count(),
            5
        );
    }

    #[test]
    fn from_std_lib_test() {
        let words = ["alpha", "beta", "gamma"];

        // chars() returns an iterator
        let merged: String = flat_map(words.iter(), |s| s.chars()).collect();
        assert_eq!(merged, "alphabetagamma");
    }

    #[test]
    fn empty_middle() {
        let words = ["alpha", "", "beta", "", "", "gamma"];
        let merged: String = flat_map(words.iter(), |s| s.chars()).collect();
        assert_eq!(merged, "alphabetagamma");
    }
}
