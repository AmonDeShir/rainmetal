use std::hash::Hash;
use bevy::prelude::{Vec2, Vec3};
use bevy::utils::HashMap;
use ordered_float::OrderedFloat;

pub trait Distance {
    fn distance(&self, other: &Self) -> usize;
}

impl Distance for usize {
    fn distance(&self, other: &Self) -> usize {
        self.abs_diff(*other)
    }
}

impl Distance for u64 {
    fn distance(&self, other: &Self) -> usize {
        self.abs_diff(*other) as usize
    }
}

impl Distance for u32 {
    fn distance(&self, other: &Self) -> usize {
        self.abs_diff(*other) as usize
    }
}

impl Distance for u16 {
    fn distance(&self, other: &Self) -> usize {
        self.abs_diff(*other) as usize
    }
}

impl Distance for u8 {
    fn distance(&self, other: &Self) -> usize {
        self.abs_diff(*other) as usize
    }
}

impl Distance for i64 {
    fn distance(&self, other: &Self) -> usize {
        (self - other).abs() as usize
    }
}

impl Distance for i32 {
    fn distance(&self, other: &Self) -> usize {
        (self - other).abs() as usize
    }
}

impl Distance for i16 {
    fn distance(&self, other: &Self) -> usize {
        (self - other).abs() as usize
    }
}

impl Distance for i8 {
    fn distance(&self, other: &Self) -> usize {
        (self - other).abs() as usize
    }
}

impl Distance for f64 {
    fn distance(&self, other: &Self) -> usize {
        (self - other).abs() as usize
    }
}

impl Distance for f32 {
    fn distance(&self, other: &Self) -> usize {
        (self - other).abs() as usize
    }
}

impl<T: Distance> Distance for OrderedFloat<T> {
    fn distance(&self, other: &Self) -> usize {
        self.0.distance(&other.0)
    }
}

impl<K: Eq + Hash, V: Distance + Default> Distance for HashMap<K, V> {
    fn distance(&self, other: &Self) -> usize {
        let mut distance = 0;

        for (key, value) in self.iter() {
            match other.get(key) {
                Some(other) => { distance += value.distance(other)},
                None => { distance += value.distance(&V::default()) }
            }
        }

        distance
    }
}

impl Distance for Vec3 {
    fn distance(&self, other: &Self) -> usize {
        Vec3::distance(*self, *other) as usize
    }
}

impl Distance for Vec2 {
    fn distance(&self, other: &Self) -> usize {
        Vec2::distance(*self, *other) as usize
    }
}

impl<V: Distance + Ord + Default + Clone> Distance for Vec<V> {
    fn distance(&self, other: &Self) -> usize {
        let mut self_sorted = self.clone();
        let mut other_sorted = other.clone();

        self_sorted.sort();
        other_sorted.sort();

        let mut total_distance = 0;

        let mut iter_a = self_sorted.iter();
        let mut iter_b = other_sorted.iter();

        let mut val_a = iter_a.next();
        let mut val_b = iter_b.next();

        while val_a.is_some() || val_b.is_some() {
            match (val_a, val_b) {
                (Some(a), Some(b)) => {
                    total_distance += a.distance(b);
                    val_a = iter_a.next();
                    val_b = iter_b.next();
                }
                (Some(a), None) => {
                    total_distance += a.distance(&V::default());
                    val_a = iter_a.next();
                }
                (None, Some(b)) => {
                    total_distance += V::default().distance(b);
                    val_b = iter_b.next();
                }
                (None, None) => break,
            }
        }

        total_distance
    }
}
