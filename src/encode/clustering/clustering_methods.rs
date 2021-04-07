use kmeans::*;

pub fn kmeans_clustering(cloud: &[[f64; 5]], k: usize) -> Vec<Vec<usize>> {
    let mut inp = vec![];
    for e in cloud.iter() {
        for i in e {
            inp.push(*i);
        }
    }
    let kmeans = KMeans::new(inp, cloud.len(), 5);
    let result = kmeans.kmeans_lloyd(k, 10, KMeans::init_kmeanplusplus, &KMeansConfig::default());
    println!("Centroids => {:?}", result.centroids);
    println!("Points => {}", cloud.len());
    println!("Cluster-Assingments => {:?}", result.assignments.len());
    let mut assingnments = vec![];
    for idx in 0..result.centroids.len() {
        let assigns: Vec<usize> = result.assignments.iter().enumerate()
            .filter(|e| {
                *e.1 == idx
            })
            .map(|e| {
                e.0
            })
            .collect();
        assingnments.push(assigns);

    }
    assingnments
}