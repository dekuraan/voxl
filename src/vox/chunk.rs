use std::{
    fmt::Debug,
    marker::PhantomData,
    ops::{Index, IndexMut},
};

/// Generic chunk data
#[derive(Clone, Copy, Debug)]
pub struct Chunk<D: Accessor, T: Unit, const N: usize> {
    data: [T; N],
    state: PhantomData<D>,
}

impl<D: Accessor, T: Unit, const N: usize> Chunk<D, T, N> {
    /// Reassigns a new Accessor data for query
    pub fn new_accessor<A: Accessor>(self) -> Chunk<A, T, N> {
        Chunk {
            data: self.data,
            state: PhantomData::default(),
        }
    }
}

impl<D: Accessor, T: Unit, const N: usize> Default for Chunk<D, T, N> {
    fn default() -> Self {
        Self {
            data: [T::default(); N],
            state: std::marker::PhantomData::default(),
        }
    }
}

/*
impl<D: Accessor, T: Unit, const N: usize> IntoIterator for Chunk<D, T, N> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;
}
*/

/// Random Docs
#[derive(Debug)]
pub struct Data;
impl Accessor for Data {
    const SIDE_LEN: usize = 8;
}

impl<D: Accessor, T: Unit, const N: usize> Index<[usize; 3]> for Chunk<D, T, N> {
    type Output = T;

    fn index(&self, [y, x, z]: [usize; 3]) -> &Self::Output {
        &self.data[y * D::QUAD_LEN + x + z * D::SIDE_LEN]
    }
}

impl<D: Accessor, T: Unit, const N: usize> IndexMut<[usize; 3]> for Chunk<D, T, N> {
    fn index_mut(&mut self, [y, x, z]: [usize; 3]) -> &mut Self::Output {
        &mut self.data[y * D::QUAD_LEN + x + z * D::SIDE_LEN]
    }
}

/// Used for tracking Chunk Dimensions.
/// An accessor trait is necessary because of a limitation
/// of min_const_generics where we can't predetermine the Chunk size
/// of the data type through 'static expressions `struct Chunk<D, T, N * N * N>`
pub trait Accessor {
    /// No. of elements in a row/column
    const SIDE_LEN: usize;
    /// No. of elements in a plane
    const QUAD_LEN: usize = Self::SIDE_LEN * Self::SIDE_LEN;
    /// No. of all elements in represented with a cube
    const CUBE_LEN: usize = Self::QUAD_LEN * Self::SIDE_LEN;
}

/// Ideal type to represent each chunk element for safely working with ECS
pub trait Unit: 'static + Debug + Default + Clone + Copy + Send + Sync {}
impl<T> Unit for T where T: 'static + Debug + Default + Clone + Copy + Send + Sync {}

/// Chunk Loading Strategy as you see fit
#[derive(Debug, Clone, Copy)]
pub enum LoadStrat {
    /// Sphere
    RuggedSphere {
        /// Radius
        radius: usize,
    },
    /// Cube
    Cube {
        /// Side length
        side: usize,
    },
}

#[test]
fn testing() {
    struct Data;
    impl Accessor for Data {
        const SIDE_LEN: usize = 8;
    }

    let mut chunk: Chunk<Data, u32, 512> = Chunk::default();
    chunk[[7, 7, 7]] = 100;

    assert_eq!(chunk[[7, 7, 7]], 100);
}