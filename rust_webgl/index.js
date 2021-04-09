const rust = import('./rust_webgl')
let value = 1;
console.log(rust.then(data => data.start(value)))
const canvas = document.getElementById("init");
canvas.width = window.screen.width-30;
canvas.height = window.screen.height-140;
document.getElementById("init").addEventListener("wheel", async (event) => {
  if (event.deltaY > 0) {
    value = value + 1;
  }
  if (event.deltaY < 0) {
    if (value > 10) {
      value = value - 1;
    }
  }
  const test =await rust.then(data => data.start(value))
  console.log(test);
});

// document.getElementById("init").addEventListener('gestureend', function (e) {
//   if (e.scale < 1.0) {
//     console.log( 'User moved fingers closer together');
//   } else if (e.scale > 1.0) {
//     console.log( 'User moved fingers further apart');
//   }
// }, false);