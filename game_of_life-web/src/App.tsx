import { Cell, Universe } from "@game_of_life/native-lib";
import { memory } from "@game_of_life/native-lib/game_of_life_web_native_lib_bg.wasm";
import { useEffect, useMemo, useRef, useState } from "react";
import "./App.scss";
import { useFps } from "./hooks";
import { randomHexColor } from "./utils";

let aliveCellColor = randomHexColor();
let gridColor = randomHexColor();
const deadCellColor = "#000";
const CELL_SIZE = 10;

function setupCanvas(canvas: HTMLCanvasElement, width: number, height: number) {
    canvas.height = height * window.devicePixelRatio;
    canvas.width = width * window.devicePixelRatio;
    canvas.style.height = `${height}px`;
    canvas.style.width = `${width}px`;
}

// Thanks to this article https://medium.com/@williamsdewi/html-canvas-and-accessibility-ffcfc317fab5
// for guiding me how to make canvas clear and responsive.
export default function App() {
    const [dim, setDim] = useState([innerWidth, innerHeight]);
    const uni = useMemo(() => {
        const w = Math.floor(dim[0] / CELL_SIZE);
        const h = Math.floor(dim[1] / CELL_SIZE);

        return Universe.new(w, h);
    }, [dim]);

    const [fps, tickFps] = useFps();

    const canvasRef = useRef<HTMLCanvasElement>(null);
    const animFrameReq = useRef<null | number>(null);

    useEffect(() => {
        addEventListener("resize", () => {
            setDim([innerWidth, innerHeight]);
            aliveCellColor = randomHexColor();
            gridColor = randomHexColor();
        });
    }, []);

    useEffect(() => {
        setupCanvas(canvasRef.current!, dim[0], dim[1]);
    }, [canvasRef, dim]);

    useEffect(() => {
        const canvas = canvasRef.current!;
        const ctx = canvas.getContext("2d", { alpha: true })!;

        ctx.setTransform(devicePixelRatio, 0, 0, devicePixelRatio, 0, 0);

        function renderFrame() {
            drawGrid(ctx, canvas);
            drawCells(ctx, uni);
            tickFps();
            uni.tick();
            animFrameReq.current = requestAnimationFrame(renderFrame);
        }

        renderFrame();

        return () => {
            if (animFrameReq.current) {
                cancelAnimationFrame(animFrameReq.current);
            }
        };
    }, [animFrameReq, uni, tickFps]);

    return (
        <div id="game">
            <canvas id="game-canvas" ref={canvasRef} />
            <div className="fps-counter">{fps}fps</div>
        </div>
    );
}

function drawGrid(ctx: CanvasRenderingContext2D, canvas: HTMLCanvasElement) {
    const csPlus1 = CELL_SIZE + 1,
        h = canvas.height,
        w = canvas.width;

    ctx.beginPath();
    ctx.strokeStyle = gridColor;

    const horizontalX = csPlus1 * w + 1;
    const verticalY = csPlus1 * h + 1;

    // Vertical lines.
    for (let i = 0; i <= w; ++i) {
        const x = i * csPlus1 + 1;

        ctx.moveTo(x, 0);
        ctx.lineTo(x, verticalY);
    }

    // Horizontal lines.
    for (let j = 0; j <= h; ++j) {
        const y = j * csPlus1 + 1;

        ctx.moveTo(0, y);
        ctx.lineTo(horizontalX, y);
    }

    ctx.stroke();
}

function drawCells(ctx: CanvasRenderingContext2D, uni: Universe) {
    const cellsPtr = uni.cells();
    const cellsLen = uni.width * uni.height;
    const cells = new Uint8Array(memory.buffer, cellsPtr, cellsLen);

    ctx.beginPath();

    ctx.fillStyle = aliveCellColor;
    filteredCells(ctx, cells, uni, Cell.Alive);

    ctx.fillStyle = deadCellColor;
    filteredCells(ctx, cells, uni, Cell.Dead);

    ctx.stroke();
}

function filteredCells(
    ctx: CanvasRenderingContext2D,
    cells: Uint8Array,
    uni: Universe,
    filterState: Cell,
) {
    const csPlus1 = CELL_SIZE + 1,
        h = uni.height,
        w = uni.width;

    for (let row = 0; row < h; ++row) {
        for (let col = 0; col < w; ++col) {
            if (cells[row * w + col] === filterState) {
                ctx.fillRect(
                    col * csPlus1 + 1,
                    row * csPlus1 + 1,
                    CELL_SIZE,
                    CELL_SIZE,
                );
            }
        }
    }
}
