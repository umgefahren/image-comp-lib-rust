use cogset::{Euclid, Kmeans};

pub fn kmeans_clustering(cloud: &[Euclid<[f64; 5]>], k: usize) -> Vec<(Euclid<[f64; 5]>, Vec<usize>)> {
    let kmeans = Kmeans::new(cloud, k);
    kmeans.clusters()
}