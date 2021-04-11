// const rust = import('./rust_webgl')
// let value = 10;
// const canvas = document.getElementById("init");
// canvas.width = window.screen.width-30;
// canvas.height = window.screen.height-140;
// window.onload= async() => {
//   await rust.then(data => data.inside("init"));
//   await rust.then(data => data.base("init",value));
// };
//  rust.then(data => data.inside("init"));
// rust.then(data => data.base("init",value));
// document.getElementById("init").addEventListener("wheel", async (event) => {
//   if (event.deltaY > 0) {
//     value = value + 1;
//   }
//   if (event.deltaY < 0) {
//     if (value > 10) {
//       value = value - 1;
//     }
//   }
//   await rust.then(data => data.inside("init"));
//   const test =await rust.then(data => data.base("init",value));
//   console.log(test);
// });

const rust = import('./rust_webgl')
let value = 10;
const canvas = document.getElementById("init");
canvas.width = window.screen.width-30;
canvas.height = window.screen.height-140;
window.onload= async() => {
  await rust.then(data => data.draw_it("init",value));
};
rust.then(data => data.draw_it("init",value));
document.getElementById("init").addEventListener("wheel", async (event) => {
  if (event.deltaY > 0) {
    value = value + 1;
  }
  if (event.deltaY < 0) {
    if (value > 10) {
      value = value - 1;
    }
  }
  const test =await rust.then(data => data.draw_it("init",value));
  console.log(test);
});
