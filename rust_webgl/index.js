const rust = import('./rust_webgl')
console.log(rust.then(data=>data.init('init', 10000, 4)));