use warp::Filter;

pub async  fn  run() {
    let hi = warp::path("hello").map(|| format!("hello world"));

    warp::serve(hi).run(([127, 0, 0, 1], 3030)).await;
}
