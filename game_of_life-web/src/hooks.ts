import { RefObject, useCallback, useEffect, useRef, useState } from "react";

type CanvasDrawer<CtxId extends keyof CanvasCtxs> = (ctx: CanvasCtxs[CtxId][1], canvas: HTMLCanvasElement) => void;

interface CanvasCtxs {
    "bitmaprenderer": [ImageBitmapRenderingContextSettings, ImageBitmapRenderingContext],
    "2d": [CanvasRenderingContext2DSettings, CanvasRenderingContext2D],
    "webgl2": [WebGLContextAttributes, WebGL2RenderingContext],
    "webgl": [WebGLContextAttributes, WebGLRenderingContext],
}

export function useCanvas<CtxId extends keyof CanvasCtxs>(ctxId: CtxId, draw: CanvasDrawer<CtxId>, deps?: React.DependencyList, ctxAttrs?: CanvasCtxs[CtxId][0]) {
    const canvasRef = useRef<HTMLCanvasElement>(null);
    const animFrameReq = useRef<null | number>(null);

    useEffect(() => {
        const canvas = canvasRef.current!;
        if (!canvas) return;

        const ctx = canvas.getContext(ctxId, ctxAttrs)!;
        if (!ctx) return;

        function renderFrame() {
            draw(ctx, canvas);
            animFrameReq.current = requestAnimationFrame(renderFrame);
        }

        canvas.height = innerHeight;
        canvas.width = innerWidth;
        renderFrame();

        return () => {
            if (animFrameReq.current) {
                cancelAnimationFrame(animFrameReq.current);
            }
        };
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [draw, animFrameReq, ctxAttrs, ctxId, ...deps || []]);

    return canvasRef as RefObject<HTMLCanvasElement>;
}

export function useFps(): [fps: number, tick: () => void] {
    const lastTickRef = useRef(performance.now());
    const [fps, setFps] = useState(0);

    return [fps, useCallback(() => {
        const now = performance.now();
        const delta = now - lastTickRef.current;
        const fps = Math.round(1000 / delta);

        lastTickRef.current = now;
        setFps(fps);
    }, [])];
}
