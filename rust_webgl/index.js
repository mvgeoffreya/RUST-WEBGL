const rust = import('./rust_webgl')
let value =10;
console.log(rust.then(data=>data.start(value)))
const canvas = document.getElementById("init");
canvas.width =window.screen.width;
canvas.height = window.screen.height;
document.getElementById("init").addEventListener("wheel",async (event)=>{
  value=value+1;
  console.log(value);
  
  console.log(await rust.then(data=>data.start(value)))
});

