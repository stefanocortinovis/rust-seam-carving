use std::cmp::Ordering;
use std::fmt;
use std::ops::{Index, IndexMut};

use image::{Rgb, RgbImage};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Orientation {
    Vertical,
    Horizontal,
}

#[derive(Eq, PartialEq)]
pub struct Array2d<T> {
    width: usize,
    data: Vec<T>,
    orientation: Orientation,
}

impl<T: fmt::Debug> fmt::Debug for Array2d<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{:?} Array2d {{", self.orientation)?;
        for row in self.data.chunks_exact(self.width) {
            writeln!(f, "{:?}", row)?;
        }
        writeln!(f, "}}")
    }
}

impl<T> Array2d<T> {
    pub fn new(width: usize, data: Vec<T>, orientation: Orientation) -> Result<Self, &'static str> {
        match data.len() % width {
            0 => Ok(Self {
                width,
                data,
                orientation,
            }),
            _ => Err("length of data and width provided are not compatible"),
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.data.len() / self.width
    }

    pub fn size(&self) -> usize {
        self.width * self.height()
    }

    pub fn dimensions(&self) -> (usize, usize) {
        (self.width, self.height())
    }

    pub fn orientation(&self) -> Orientation {
        self.orientation
    }

    pub fn raw_data(&self) -> &[T] {
        &self.data
    }

    pub fn raw_data_mut(&mut self) -> &mut [T] {
        &mut self.data
    }
}

impl Array2d<Rgb<u8>> {
    pub fn from_image(img: &RgbImage) -> Result<Self, &'static str> {
        let width = img.dimensions().0 as usize;
        let mut data = Vec::new();
        img.pixels().for_each(|&p| data.push(p));
        Ok(Self {
            width,
            data,
            orientation: Orientation::Vertical,
        })
    }

    pub fn to_image(&self) -> RgbImage {
        let (width, height) = self.dimensions();
        let mut img = RgbImage::new(width as u32, height as u32);
        for (i, &p) in self.data.iter().enumerate() {
            // TODO: implement iterator
            let (x, y) = (i % width, i / width);
            img.put_pixel(x as u32, y as u32, p);
        }
        img
    }
}

impl<T: Copy + std::fmt::Debug> Array2d<T> {
    pub fn transpose(&mut self) {
        self.orientation = match self.orientation {
            Orientation::Vertical => Orientation::Horizontal,
            Orientation::Horizontal => Orientation::Vertical,
        };
        self.width = self.height();
    }

    // TODO: change implementation when horizontal seam introduced
    pub fn remove_seam_default(&mut self, seam: &[usize], default: T) -> Result<(), &'static str> {
        let (width, height) = self.dimensions();
        if seam.len() != self.height() {
            return Err("seam length should be equal to image height");
        }

        // Copy to new array instead of modifying in place, approximately 3x faster in release binary
        let mut new_data = vec![default; self.size() - height];
        for y in 0..height {
            let to_remove = seam[y];
            for x in 0..width {
                if let Some(i) = match (x.cmp(&to_remove), self.orientation) {
                    (Ordering::Less, Orientation::Vertical) => Some(x + y * (width - 1)),
                    (Ordering::Less, Orientation::Horizontal) => Some(y + x * height),
                    (Ordering::Greater, Orientation::Vertical) => Some((x - 1) + y * (width - 1)),
                    (Ordering::Greater, Orientation::Horizontal) => Some(y + (x - 1) * height),
                    _ => None,
                } {
                    new_data[i] = self[(x, y)];
                }
            }
        }
        self.data = new_data;

        self.width -= 1;
        Ok(())
    }
}

impl Array2d<u32> {
    pub fn remove_seam(&mut self, seam: &[usize]) -> Result<(), &'static str> {
        self.remove_seam_default(seam, u32::default())
    }
}

impl Array2d<Rgb<u8>> {
    pub fn remove_seam(&mut self, seam: &[usize]) -> Result<(), &'static str> {
        self.remove_seam_default(seam, Rgb([0, 0, 0]))
    }
}

impl<T> Index<(usize, usize)> for Array2d<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (x, y) = index; // col, row
        let (width, height) = self.dimensions();
        match self.orientation {
            Orientation::Vertical => &self.data[x + y * width],
            Orientation::Horizontal => &self.data[y + x * height],
        }
    }
}

impl<T> IndexMut<(usize, usize)> for Array2d<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (x, y) = index;
        let (width, height) = self.dimensions();
        match self.orientation {
            Orientation::Vertical => &mut self.data[x + y * width],
            Orientation::Horizontal => &mut self.data[y + x * height],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn array2d_new() {
        let _arr = Array2d::new(3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9], Orientation::Vertical).unwrap();
    }

    #[test]
    #[should_panic]
    fn array2d_new_incompatible() {
        let _arr = Array2d::new(4, vec![1, 2, 3, 4, 5, 6, 7, 8, 9], Orientation::Vertical).unwrap();
    }

    #[test]
    fn indexing_vertical() {
        let arr = Array2d::new(
            3,
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            Orientation::Vertical,
        )
        .unwrap();
        assert_eq!(1, arr[(0, 0)]);
        assert_eq!(2, arr[(1, 0)]);
        assert_eq!(3, arr[(2, 0)]);
        assert_eq!(4, arr[(0, 1)]);
        assert_eq!(5, arr[(1, 1)]);
        assert_eq!(6, arr[(2, 1)]);
        assert_eq!(7, arr[(0, 2)]);
        assert_eq!(8, arr[(1, 2)]);
        assert_eq!(9, arr[(2, 2)]);
        assert_eq!(10, arr[(0, 3)]);
        assert_eq!(11, arr[(1, 3)]);
        assert_eq!(12, arr[(2, 3)]);
    }

    #[test]
    fn indexing_horizontal() {
        let arr = Array2d::new(
            3,
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            Orientation::Horizontal,
        )
        .unwrap();
        assert_eq!(1, arr[(0, 0)]);
        assert_eq!(5, arr[(1, 0)]);
        assert_eq!(9, arr[(2, 0)]);
        assert_eq!(2, arr[(0, 1)]);
        assert_eq!(6, arr[(1, 1)]);
        assert_eq!(10, arr[(2, 1)]);
        assert_eq!(3, arr[(0, 2)]);
        assert_eq!(7, arr[(1, 2)]);
        assert_eq!(11, arr[(2, 2)]);
        assert_eq!(4, arr[(0, 3)]);
        assert_eq!(8, arr[(1, 3)]);
        assert_eq!(12, arr[(2, 3)]);
    }

    #[test]
    fn indexing_mut() {
        let mut arr =
            Array2d::new(3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9], Orientation::Vertical).unwrap();
        assert_eq!(1, arr[(0, 0)]);
        arr[(0, 0)] = 2;
        assert_eq!(2, arr[(0, 0)]);
    }

    #[test]
    fn transposition() {
        let mut arr =
            Array2d::new(3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9], Orientation::Vertical).unwrap();
        arr.transpose();
        assert_eq!(
            Array2d::new(3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9], Orientation::Horizontal).unwrap(),
            arr
        );
    }

    #[test]
    fn seam_removal_vertical() {
        let mut arr =
            Array2d::new(3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9], Orientation::Vertical).unwrap();
        let seam = vec![1, 2, 1];
        arr.remove_seam(&seam).unwrap();
        assert_eq!(
            Array2d::new(2, vec![1, 3, 4, 5, 7, 9], Orientation::Vertical).unwrap(),
            arr
        );
    }

    #[test]
    fn seam_removal_horizontal() {
        let mut arr =
            Array2d::new(3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9], Orientation::Horizontal).unwrap();
        let seam = vec![1, 2, 1];
        arr.remove_seam(&seam).unwrap();
        assert_eq!(
            Array2d::new(2, vec![1, 2, 3, 7, 5, 9], Orientation::Horizontal).unwrap(),
            arr
        );
    }
}
