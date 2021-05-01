use std::fmt;
use std::ops::{Index, IndexMut};

use image::{Rgb, RgbImage};

#[derive(Eq, PartialEq)]
pub struct Array2d<T> {
    width: usize,
    data: Vec<T>,
}

impl<T: fmt::Debug> fmt::Debug for Array2d<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Array2d {{")?;
        for row in self.data.chunks_exact(self.width) {
            writeln!(f, "{:?}", row)?;
        }
        writeln!(f, "}}")
    }
}

impl<T> Index<(usize, usize)> for Array2d<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (x, y) = index; // col, row
        &self.data[x + y * self.width]
    }
}

impl<T> IndexMut<(usize, usize)> for Array2d<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (x, y) = index;
        &mut self.data[x + y * self.width]
    }
}

impl<T> Array2d<T> {
    pub fn new(width: usize, data: Vec<T>) -> Result<Self, &'static str> {
        match data.len() % width {
            0 => Ok(Self { width, data }),
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

    pub fn raw_data(&self) -> &[T] {
        &self.data
    }

    pub fn raw_data_mut(&mut self) -> &mut [T] {
        &mut self.data
    }
}

impl<T: Copy> Array2d<T> {
    pub fn transpose(&mut self) {
        let (width, height) = self.dimensions();
        let mut new_data = Vec::with_capacity(self.size());
        for x in 0..width {
            for y in 0..height {
                new_data.push(self[(x, y)])
            }
        }
        self.width = height;
        self.data = new_data;
    }

    // TODO: change implementation when horizontal seam introduced
    pub fn remove_seam(&mut self, seam: &[usize]) -> Result<(), &'static str> {
        let (width, height) = self.dimensions();
        if seam.len() != height {
            return Err("seam length should be equal to image height");
        }

        // Copy to new array instead of modifying in place, approximately 3x faster in release binary
        let mut new_data = Vec::with_capacity(self.size() - height);
        seam.iter().enumerate().for_each(|(y, &to_remove_x)| {
            for x in 0..width {
                if x != to_remove_x {
                    new_data.push(self[(x, y)]);
                }
            }
        });

        self.data = new_data;
        self.width -= 1;
        Ok(())
    }
}

impl Array2d<Rgb<u8>> {
    pub fn from_image(img: &RgbImage) -> Result<Self, &'static str> {
        let (width, height) = img.dimensions();
        let mut data = Vec::with_capacity((width * height) as usize);
        img.pixels().for_each(|&p| data.push(p));
        Ok(Self {
            width: width as usize,
            data,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn array2d_new() {
        let _arr = Array2d::new(3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]).unwrap();
    }

    #[test]
    #[should_panic]
    fn array2d_new_incompatible() {
        let _arr = Array2d::new(4, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]).unwrap();
    }

    #[test]
    fn indexing() {
        let arr = Array2d::new(3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]).unwrap();
        assert_eq!(1, arr[(0, 0)]);
        assert_eq!(2, arr[(1, 0)]);
        assert_eq!(3, arr[(2, 0)]);
        assert_eq!(4, arr[(0, 1)]);
        assert_eq!(5, arr[(1, 1)]);
        assert_eq!(6, arr[(2, 1)]);
        assert_eq!(7, arr[(0, 2)]);
        assert_eq!(8, arr[(1, 2)]);
        assert_eq!(9, arr[(2, 2)]);
    }

    #[test]
    fn indexing_mut() {
        let mut arr = Array2d::new(3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]).unwrap();
        assert_eq!(1, arr[(0, 0)]);
        arr[(0, 0)] = 2;
        assert_eq!(2, arr[(0, 0)]);
    }

    #[test]
    fn transposition() {
        let mut arr = Array2d::new(3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]).unwrap();
        arr.transpose();
        assert_eq!(
            Array2d::new(3, vec![1, 4, 7, 2, 5, 8, 3, 6, 9]).unwrap(),
            arr
        );
    }

    #[test]
    fn seam_removal_vertical() {
        let mut arr = Array2d::new(3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]).unwrap();
        let seam = vec![1, 2, 1];
        arr.remove_seam(&seam).unwrap();
        assert_eq!(Array2d::new(2, vec![1, 3, 4, 5, 7, 9]).unwrap(), arr);
    }

    #[test]
    fn seam_removal_horizontal() {
        let mut arr = Array2d::new(3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]).unwrap();
        arr.transpose();
        let seam = vec![1, 2, 1];
        arr.remove_seam(&seam).unwrap();
        arr.transpose();
        assert_eq!(Array2d::new(3, vec![1, 2, 3, 7, 5, 9]).unwrap(), arr);
    }
}
