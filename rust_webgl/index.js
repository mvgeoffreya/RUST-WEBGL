// const rust = import('./rust_webgl')
let scale = 10;
let vertices1 = [
    -(0.05 / 10.0) * scale,
    (0.05 / 10.0) * scale,
    0.00,
    -(0.1 / 10.0) * scale,
    -(0.00 / 10.0) * scale,
    0.00,
    -(0.05 / 10.0) * scale,
    -(0.05 / 10.0) * scale,
    0.00,
    (0.00 / 10.0) * scale,
    (0.00 / 10.0) * scale,
    0.00,
  ]
  ;
  let colors1 = [0.0, 0.0, 0.1, 0.1, 0.0, 0.0, 0.0, 0.1, 0.0, 0.1, 0.0, 0.1];
  let indices1 = [3, 2, 1, 3, 1, 0];
  let vertices2  = [
    -(1.5 / 10.0) * scale,
    (1.5 / 10.0) * scale,
    0.00,
    -(1.45 / 10.0) * scale,
    (1.45 / 10.0) * scale,
   0.00,
    (0.7 / 10.0) * scale,
    (0.7 / 10.0) * scale,
   0.00,
    (1.45 / 10.0) * scale,
    (1.45 / 10.0) * scale,
   0.00
  ];

  let vertices3  = [
    -(1.5 / 10.0) * (scale+1.0),
    (1.5 / 10.0) * (scale+1.0),
    0.00,
    -(1.45 / 10.0) * (scale+1.0),
    (1.45 / 10.0) * (scale+1.0),
   0.00,
    (0.7 / 10.0) * (scale+1.0),
    (0.7 / 10.0) * (scale+1.0),
   0.00,
    (1.45 / 10.0) * (scale+1.0),
    (1.45 / 10.0) * (scale+1.0),
   0.00
  ];

  let colors2  = [0.0, 0.0, 0.1, 0.1, 0.0, 0.0, 0.0, 0.1, 0.0, 0.1, 0.0, 0.1];
  let indices2 = [3, 2, 1, 3, 1, 0];
  let  payload =[{ vertices:vertices1,
    colors: colors1,
    indices: indices1,
   },{
    vertices:vertices2,
    colors: colors2,
    indices: indices2,
   },
   {
    vertices:vertices3,
    colors: colors2,
    indices: indices2,
   }
  ];


// // vertex positions
// var vertexPos_first = new Float32Array(
//   [  0.3, 0.3, 0.3,  -0.3, 0.3, 0.3,  -0.3,-0.3, 0.3,   0.3,-0.3, 0.3,
//      0.3, 0.3, 0.3,   0.3,-0.3, 0.3,   0.3,-0.3,-0.3,   0.3, 0.3,-0.3,
//      0.3, 0.3, 0.3,   0.3, 0.3,-0.3,  -0.3, 0.3,-0.3,  -0.3, 0.3, 0.3,
//     -0.3, 0.3, 0.3,  -0.3, 0.3,-0.3,  -0.3,-0.3,-0.3,  -0.3,-0.3, 0.3,
//     -0.3,-0.3,-0.3,   0.3,-0.3,-0.3,   0.3,-0.3, 0.3,  -0.3,-0.3, 0.3,
//      0.3,-0.3,-0.3,  -0.3,-0.3,-0.3,  -0.3, 0.3,-0.3,   0.3, 0.3,-0.3 ]);

// var vertexPos_second = new Float32Array(
//   [  0.3+0.3, 0.3+0.3, 0.3+0.3,   0.3+0.3, 0.3+0.3, 0.3+0.3,   0.1+0.3, 0.1+0.3, 0.3+0.3,   0.3+0.3, 0.1+0.3, 0.3+0.3,
//      0.3+0.3, 0.3+0.3, 0.3+0.3,   0.3+0.3, 0.1+0.3, 0.3+0.3,   0.3+0.3, 0.1+0.3, 0.1+0.3,   0.3+0.3, 0.3+0.3, 0.1+0.3,
//      0.3+0.3, 0.3+0.3, 0.3+0.3,   0.3+0.3, 0.3+0.3, 0.1+0.3,   0.1+0.3, 0.3+0.3, 0.1+0.3,   0.1+0.3, 0.3+0.3, 0.3+0.3,
//      0.1+0.3, 0.3+0.3, 0.3+0.3,   0.1+0.3, 0.3+0.3, 0.1+0.3,   0.1+0.3, 0.1+0.3, 0.1+0.3,   0.1+0.3, 0.1+0.3, 0.3+0.3,
//      0.1+0.3, 0.1+0.3, 0.1+0.3,   0.3+0.3, 0.1+0.3, 0.1+0.3,   0.3+0.3, 0.1+0.3, 0.3+0.3,   0.1+0.3, 0.1+0.3, 0.3+0.3,
//      0.3+0.3, 0.1+0.3, 0.1+0.3,   0.1+0.3, 0.1+0.3, 0.1+0.3,   0.1+0.3, 0.3+0.3, 0.1+0.3,   0.3+0.3, 0.3+0.3, 0.1+0.3 ]);

// // vertex colors
// var vertexColors_first = new Float32Array(
//   [  0, 0, 1,   0, 0, 1,   0, 0, 1,   0, 0, 1,
//      1, 0, 0,   1, 0, 0,   1, 0, 0,   1, 0, 0,
//      0, 1, 0,   0, 1, 0,   0, 1, 0,   0, 1, 0,
//      1, 1, 0,   1, 1, 0,   1, 1, 0,   1, 1, 0,
//      1, 0, 1,   1, 0, 1,   1, 0, 1,   1, 0, 1,
//      0, 1, 1,   0, 1, 1,   0, 1, 1,   0, 1, 1 ]);

// var vertexColors_second = new Float32Array(
//   [  0, 0, 1,   0, 0, 1,   0, 0, 1,   0, 0, 1,
//      1, 0, 0,   1, 0, 0,   1, 0, 0,   1, 0, 0,
//      0, 1, 0,   0, 1, 0,   0, 1, 0,   0, 1, 0,
//      1, 1, 0,   1, 1, 0,   1, 1, 0,   1, 1, 0,
//      1, 0, 1,   1, 0, 1,   1, 0, 1,   1, 0, 1,
//      0, 1, 1,   0, 1, 1,   0, 1, 1,   0, 1, 1 ]);

// // element index array
// var triangleIndices_first = new Uint16Array(
//   [  0, 1, 2,   0, 2, 3,    // front
//      4, 5, 6,   4, 6, 7,    // right
//      8, 9,10,   8,10,11,    // top
//     12,13,14,  12,14,15,    // left
//     16,17,18,  16,18,19,    // bottom
//   20,21,22,  20,22,23 ]); // back

// var triangleIndices_second = new Uint16Array(
//   [  0, 1, 2,   0, 2, 3,    // front
//      4, 5, 6,   4, 6, 7,    // right
//      8, 9,10,   8,10,11,   // top
//     12,13,14,  12,14,15,    // left
//     16,17,18,  16,18,19,    // bottom
//   20,21,22,  20,22,23 ]); // back

  // let  payload =[{ vertices:vertexPos_first,
  //   colors: vertexColors_first,
  //   indices: triangleIndices_first,
  //  },{
  //   vertices:vertexPos_second,
  //   colors: vertexColors_second,
  //   indices: triangleIndices_second ,
  //  }
  // ];

import('./rust_webgl').then(data => console.log(data.draw_it("init", scale, payload)));
const canvas = document.getElementById("init");
canvas.width = window.screen.width-30;
canvas.height = window.screen.height-140;
// window.onload= async() => {
//   await rust.then(data => data.inside("init"));
//   await rust.then(data => data.base("init",scale));
// };
//  rust.then(data => data.inside("init"));
// rust.then(data => data.base("init",scale));
// document.getElementById("init").addEventListener("wheel", async (event) => {
//   if (event.deltaY > 0) {
//     scale = scale + 1;
//   }
//   if (event.deltaY < 0) {
//     if (scale > 10) {
//       scale = scale - 1;
//     }
//   }
//   await rust.then(data => data.inside("init"));
//   const test =await rust.then(data => data.base("init",scale));
//   console.log(test);
// });

// const rust = import('./rust_webgl')
// let scale = 10;
// let delta_x = 0.0;
// let delta_y = 0.0;
// const canvas = document.getElementById("init");
// canvas.width = window.screen.width - 30;
// canvas.height = window.screen.height - 140;
// window.onload = async () => {
//   await rust.then(data => data.draw_it("init", scale));
// };
// rust.then(data => data.draw_it("init", scale));
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

//   const test = await rust.then(data => data.draw_it("init", scale));
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
//     const test = await rust.then(data => data.draw_it("init", scale));
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
