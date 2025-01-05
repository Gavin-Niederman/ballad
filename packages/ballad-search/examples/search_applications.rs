use ballad_search::{applications_results, files_results};
use futures::join;

fn main() {
    let now = std::time::Instant::now();
    let results =
        smol::block_on(async { join!(applications_results(), files_results(".png")) });
    let results = results.0.into_iter().chain(results.1.into_iter()).collect::<Vec<_>>();
    let completed = now.elapsed();
    println!(
        "found {} results in {}ms",
        results.len(),
        completed.as_millis()
    );
    for result in results {
        println!("{:?}", result);
    }
}
