// const rust = import('./rust_webgl')
let value = 10;
import('./rust_webgl').then(data => console.log(data.draw_it("init", value)));
const canvas = document.getElementById("init");
canvas.width = window.screen.width-30;
canvas.height = window.screen.height-140;
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

// const rust = import('./rust_webgl')
// let value = 10;
// let delta_x = 0.0;
// let delta_y = 0.0;
// const canvas = document.getElementById("init");
// canvas.width = window.screen.width - 30;
// canvas.height = window.screen.height - 140;
// window.onload = async () => {
//   await rust.then(data => data.draw_it("init", value));
// };
// rust.then(data => data.draw_it("init", value));
// const move = async (event) => {
//   if (event.deltaY<0) {
//   delta_y = delta_y - 0.01;
//   } else if (event.deltaY>0){
//     delta_y = delta_y + 0.01;
//   }
//   if (event.deltaX<0) {
//     delta_x = delta_x + 0.01;
//     } else if (event.deltaX >0) {
//       delta_x = delta_x - 0.01;
//     }

//   const test = await rust.then(data => data.draw_it("init", value));
//   console.log(test);
// };


// canvas.onwheel = move;




// let downListener = (event) => {
//   console.log(event.clientY, event.clientX);
//   position_x = event.clientX;
//   position_y = event.clientY;
//   click = true
// }
// document.getElementById("init").addEventListener('mousedown', (event) => downListener(event))
// let moveListener = async (event) => {
//   delta_x = delta_x - (position_x - event.clientX)/200000;
//   delta_y = delta_y - (event.clientY- position_y)/200000;;
  
//   if (click) {
//     console.log(delta_y,delta_x)
//     const test = await rust.then(data => data.draw_it("init", value));
//     console.log(test);
//   }
// }
// document.getElementById("init").addEventListener('mousemove', (event) => moveListener(event))
// let upListener = (event) => {
//   position_x = 0.0;
//   position_y = 0.0;
//   click= false;
// }
// document.getElementById("init").addEventListener('mouseup', (event) => upListener(event))
