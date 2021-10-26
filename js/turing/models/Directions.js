/**
 * Helper class for common directions in Radians.
 */

const up = 0;
const right = Math.PI / 4;
const down = right * 2;
const left = down + right;

const upright = right / 2;
const rightup = upright;
const rightdown = down - upright;
const downright = rightdown;
const downleft = down + rightup;
const leftdown = downleft;
const leftup = left + rightup;
const upleft = leftup;