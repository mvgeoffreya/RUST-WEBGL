const rust = import('./rust_webgl')
let value = 1;
const canvas = document.getElementById("init");
canvas.width = window.screen.width-30;
canvas.height = window.screen.height-140;
window.onload= async() => {
  await rust.then(data => data.start("init",value))
};

rust.then(data => data.start("init",value))
document.getElementById("init").addEventListener("wheel", async (event) => {
  if (event.deltaY > 0) {
    value = value + 1;
  }
  if (event.deltaY < 0) {
    if (value > 1) {
      value = value - 1;
    }
  }
  const test =await rust.then(data => data.start("init",value))
  console.log(test);
});
