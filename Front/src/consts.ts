export const backIp = "http://localhost:8888"
export const timeScale = 1;

export function easeOutCirc(x: number): number { // https://easings.net/#easeOutCirc
  return Math.sqrt(1 - Math.pow(x - 1, 2));
}

export function easeInOutCubic(x: number): number {
  return x < 0.5 ? 4 * x * x * x : 1 - Math.pow(-2 * x + 2, 3) / 2;
}
