import {Universe, Cell} from "wasm-game-of-life";
import {memory} from "wasm-game-of-life/wasm_game_of_life_bg";
// Cell format
const CELL_SIZE = 5; // px
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

// create a new universe
const universe = Universe.new();
// get the dims of universe
const height = universe.height();
const width = universe.width();
console.log(height);
// Get the canvas element to render the universe
const canvas = document.getElementById("game-of-life-canvas");
// set the canvas height and width
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;
// get contex of canvas
const ctx = canvas.getContext("2d");
// keep track of the animation to enable it play pause
let animationId = null;
// the render loop
const renderLoop = () => {
    // Add the debugger
    // DEbugger will pause the renderLoop at
    // each iteration
    //debugger;
    universe.tick();
    // draw a grid on the canvas
    drawGrid();
    // draw respective dead and alive cells on the grid
    drawCell();
    // update animationId with the current annimation
    animationId = requestAnimationFrame(renderLoop);
};

const drawGrid = () => {
    // start the draw
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;
    // draw vertical lines
    for (let i = 0; i < width; i++) {
        // keep y=0 and move on x direction (along the width)
        ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
        // draw a line through y axis from the current position
        ctx.lineTo(i * (CELL_SIZE + 1) + 1, height * (CELL_SIZE + 1) + 1);
    }
    // draw horizontal lines
    for (let j = 0; j < height; j++) {
        // keep x=0 and move on y direction (along the height)
        ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
        // draw a line through x axis from the current position
        ctx.lineTo(width * (CELL_SIZE + 1) + 1, j * (CELL_SIZE + 1) + 1);
    }
    // draw the grid
    ctx.stroke();
};

/*
Book para:
We can directly access WebAssembly's linear memory via memory, 
which is defined in the raw wasm module wasm_game_of_life_bg. To draw the cells, 
we get a pointer to the universe's cells, construct a Uint8Array overlaying the cells buffer, 
iterate over each cell, and draw a white or black rectangle depending on whether the cell is dead or alive, 
respectively. By working with pointers and overlays, we avoid copying the cells across the 
boundary on every tick.
 */
// get the idx of flat mem vect
const getIndex = (row, col) => {
    return row * width + col;
};
const drawCell = () => {
    // get the pointer to the start of cell vector
    const cellsPtr = universe.cells();
    // init a Uint8Array memory buffer and overlay the wasm linear memory on it
    const cells = new Uint8Array(memory.buffer, cellsPtr, height * width);
    // start the draw path
    ctx.beginPath();
    // now draw the cells according to their status
    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            // get the cell index
            const idx = getIndex(row, col);
            // get the color of the cell corresponds to the state of the cell
            ctx.fillStyle = cells[idx] === Cell.Dead 
                                ? DEAD_COLOR 
                                : ALIVE_COLOR;
            ctx.fillRect(
                // x pos
                row * (CELL_SIZE + 1) + 1,
                // y pos
                col * (CELL_SIZE + 1) + 1,
                // cell width
                CELL_SIZE,
                // cell height
                CELL_SIZE, 
            );
        }
    }
    ctx.stroke();
};

// check if the animation is paused
// return true when id is null (paused)
const isPaused = () => {
    return animationId === null;
};
// get the play pause button dom elem
const playPauseButton = document.getElementById("play-pause");
// function that play the annimation when its stopped
const play = () => {
    // change the button to pause symbol
    playPauseButton.textContent = "⏸";
    // resume animation
    renderLoop();
};
// function that pause the annimation, set the id = null and
// change the button to play symbol
const pause = () => {
    // change to play symb
    playPauseButton.textContent = "▶";
    cancelAnimationFrame(animationId);
    animationId = null;
};
// on button click call functions accordingly
playPauseButton.addEventListener('click', event => {
    if (isPaused()) {
        play();
    } else {
        pause();
    }
});

// Toggle the cell on the canvas when clicked
canvas.addEventListener('click', event => {
    // coordinate transformation from webpage (global)
    // to canvas location (local) so we know the cell index
    const boundingRect = canvas.getBoundingClientRect();
    
    const scaleX = canvas.width / boundingRect.width;
    const scaleY = canvas.height / boundingRect.height;

    const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
    const canvasTop = (event.clientY - boundingRect.top) * scaleY;

    const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), height - 5);
    const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), width - 5);

    universe.toggle_cell(row, col);
    drawGrid();
    drawCell();
});
// initiate the grid, cells and the render loop and run it
drawGrid();
drawCell();
//requestAnimationFrame(renderLoop);
play();


//import * as wasm from "wasm-game-of-life";

//wasm.greet();
/*
import {Universe} from 'wasm-game-of-life';

// get the pre tag from html dom
const pre = document.getElementById("game-of-life-canvas");
// initialize the universe
const universe = Universe.new();
*/
/*The JavaScript runs in a requestAnimationFrame loop. 
On each iteration, it draws the current universe to the <pre>, 
and then calls Universe::tick. 
*/
// render loop; annonymous recursive function
/*
const renderLoop = () => {
    // render the universe
    // this string conversion from Rust -> wasm -> Js
    // makes unnecessary copies. Modify the render method
    pre.textContent = universe.render();
    // tick the unvierse
    universe.tick();
    // recurse the function
    requestAnimationFrame(renderLoop);
}

requestAnimationFrame(renderLoop);
*/

