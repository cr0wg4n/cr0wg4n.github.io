// Two small Tetris wells on both sides of the home avatar, each playing itself. Purely decorative:
// the canvases are hidden from assistive tech, ignore the pointer, pause when off screen or in a background
// tab, and with "reduce motion" they show one still frame instead of moving.
(function () {
  var avatar = document.querySelector("main article > header > img.rounded-full");
  if (!avatar) return;
  var header = avatar.parentElement;
  var reduce = window.matchMedia("(prefers-reduced-motion: reduce)");

  var COLS = 6, ROWS = 10, CELL = 14;
  var SHAPES = {
    I: [[1, 1, 1, 1]], O: [[1, 1], [1, 1]], T: [[0, 1, 0], [1, 1, 1]],
    S: [[0, 1, 1], [1, 1, 0]], Z: [[1, 1, 0], [0, 1, 1]], J: [[1, 0, 0], [1, 1, 1]], L: [[0, 0, 1], [1, 1, 1]]
  };
  var TYPES = Object.keys(SHAPES);
  var TOKENS = { I: "primary-300", O: "primary-400", T: "primary-500", S: "primary-600", Z: "primary-700", J: "secondary-400", L: "secondary-600" };

  function rotate(m) {
    return m[0].map(function (_, c) { return m.map(function (r) { return r[c]; }).reverse(); });
  }
  function palette() {
    var style = getComputedStyle(document.documentElement), out = {};
    TYPES.forEach(function (t) {
      var v = style.getPropertyValue("--color-" + TOKENS[t]).trim();
      out[t] = v ? "rgb(" + v + ")" : "#f97316";
    });
    return out;
  }

  function Well(canvas, delay) {
    var dpr = Math.min(window.devicePixelRatio || 1, 2);
    canvas.width = COLS * CELL * dpr;
    canvas.height = ROWS * CELL * dpr;
    var ctx = canvas.getContext("2d");
    ctx.scale(dpr, dpr);
    var colors = palette();
    var grid, piece, flash, wait;

    function reset() {
      grid = [];
      for (var r = 0; r < ROWS; r++) grid.push(new Array(COLS).fill(null));
      piece = null; flash = 0; wait = delay || 0; delay = 0;
    }
    function collide(g, shape, x, y) {
      for (var r = 0; r < shape.length; r++) for (var c = 0; c < shape[r].length; c++) {
        if (!shape[r][c]) continue;
        var gx = x + c, gy = y + r;
        if (gx < 0 || gx >= COLS || gy >= ROWS) return true;
        if (gy >= 0 && g[gy][gx]) return true;
      }
      return false;
    }
    function lock(g, shape, x, y, type) {
      shape.forEach(function (row, r) { row.forEach(function (v, c) { if (v && y + r >= 0) g[y + r][x + c] = type; }); });
    }
    function fullRows(g) {
      var rows = [];
      g.forEach(function (row, r) { if (row.every(Boolean)) rows.push(r); });
      return rows;
    }
    // Heuristic player: fewer holes, a lower and flatter stack, and lines when possible. A little noise
    // keeps it from looking perfect.
    function score(g) {
      var heights = [], holes = 0, c, r;
      for (c = 0; c < COLS; c++) {
        var top = ROWS;
        for (r = 0; r < ROWS; r++) if (g[r][c]) { top = r; break; }
        heights.push(ROWS - top);
        for (r = top + 1; r < ROWS; r++) if (!g[r][c]) holes++;
      }
      var sum = heights.reduce(function (a, b) { return a + b; }, 0), bump = 0;
      for (c = 1; c < COLS; c++) bump += Math.abs(heights[c] - heights[c - 1]);
      return -0.5 * sum - 0.4 * holes * 3 - 0.2 * bump + 0.8 * fullRows(g).length * 3 + Math.random() * 0.9;
    }
    function plan(type) {
      var best = null, shape = SHAPES[type];
      for (var rot = 0; rot < 4; rot++) {
        for (var x = 0; x + shape[0].length <= COLS; x++) {
          var y = -shape.length;
          if (collide(grid, shape, x, y)) continue;
          while (!collide(grid, shape, x, y + 1)) y++;
          var copy = grid.map(function (row) { return row.slice(); });
          lock(copy, shape, x, y, type);
          var s = score(copy);
          if (!best || s > best.score) best = { score: s, rot: rot, x: x };
        }
        shape = rotate(shape);
      }
      return best;
    }
    function spawn() {
      var type = TYPES[Math.floor(Math.random() * TYPES.length)];
      var best = plan(type), shape = SHAPES[type];
      if (!best) { reset(); wait = 6; return; }
      for (var i = 0; i < best.rot; i++) shape = rotate(shape);
      piece = { type: type, shape: shape, x: Math.floor((COLS - shape[0].length) / 2), y: -shape.length, target: best.x, tick: 0 };
    }

    // One step of the game (called every ~90 ms).
    function step() {
      if (wait > 0) { wait--; return; }
      if (flash > 0) {
        if (--flash === 0) {
          var keep = grid.filter(function (row) { return !row.every(Boolean); });
          while (keep.length < ROWS) keep.unshift(new Array(COLS).fill(null));
          grid = keep;
        }
        return;
      }
      if (!piece) { spawn(); return; }
      if (piece.x !== piece.target) {
        var nx = piece.x + (piece.target > piece.x ? 1 : -1);
        if (!collide(grid, piece.shape, nx, piece.y)) piece.x = nx;
      }
      if (++piece.tick % 2) return; // gravity: one row every second step
      if (!collide(grid, piece.shape, piece.x, piece.y + 1)) { piece.y++; return; }
      lock(grid, piece.shape, piece.x, piece.y, piece.type);
      piece = null;
      if (fullRows(grid).length) flash = 4;
      else if (grid[1].some(Boolean)) { reset(); wait = 8; } // stack reached the top: start over
    }

    function cell(x, y, color) {
      var px = x * CELL, py = y * CELL;
      ctx.fillStyle = color;
      ctx.globalAlpha = 0.85;
      ctx.beginPath();
      if (ctx.roundRect) ctx.roundRect(px + 1, py + 1, CELL - 2, CELL - 2, 3); else ctx.rect(px + 1, py + 1, CELL - 2, CELL - 2);
      ctx.fill();
      ctx.globalAlpha = 0.25;
      ctx.fillStyle = "#fff";
      ctx.fillRect(px + 3, py + 3, CELL - 6, 2);
      ctx.globalAlpha = 1;
    }
    function draw() {
      ctx.clearRect(0, 0, COLS * CELL, ROWS * CELL);
      var full = flash > 0 ? fullRows(grid) : [];
      grid.forEach(function (row, r) {
        row.forEach(function (t, c) {
          if (t) cell(c, r, full.indexOf(r) >= 0 && flash % 2 ? "#fff" : colors[t]);
        });
      });
      if (piece) piece.shape.forEach(function (row, r) {
        row.forEach(function (v, c) { if (v && piece.y + r >= 0) cell(piece.x + c, piece.y + r, colors[piece.type]); });
      });
    }
    reset();
    return {
      tick: function () { step(); draw(); },
      still: function () { // "reduce motion": play ahead without drawing, then show one frame
        for (var i = 0; i < 260; i++) step();
        piece = null; draw();
      }
    };
  }

  function make(side, delay) {
    var canvas = document.createElement("canvas");
    canvas.className = "tetris-well tetris-" + side;
    canvas.setAttribute("aria-hidden", "true");
    canvas.style.width = COLS * CELL + "px";
    canvas.style.height = ROWS * CELL + "px";
    header.appendChild(canvas);
    return Well(canvas, delay);
  }
  var wells = [make("left", 0), make("right", 14)];

  if (reduce.matches) { wells.forEach(function (w) { w.still(); }); return; }

  var timer = null, onScreen = true;
  function run() {
    var should = onScreen && !document.hidden && getComputedStyle(header.querySelector(".tetris-well")).display !== "none";
    if (should && !timer) timer = setInterval(function () { wells.forEach(function (w) { w.tick(); }); }, 90);
    if (!should && timer) { clearInterval(timer); timer = null; }
  }
  if ("IntersectionObserver" in window) {
    new IntersectionObserver(function (e) { onScreen = e[0].isIntersecting; run(); }).observe(header);
  }
  document.addEventListener("visibilitychange", run);
  window.addEventListener("resize", run);
  run();
})();
