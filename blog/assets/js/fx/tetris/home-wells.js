// Two small Tetris wells on both sides of the home avatar, each playing itself. Purely decorative:
// the canvases are hidden from assistive tech, pause when off screen or in a background
// tab, and with "reduce motion" they show one still frame instead of moving. A click on a well drops the falling piece in that column.
(function () {
  var avatar = document.querySelector("main article > header > img.rounded-full");
  if (!avatar) return;
  var header = avatar.parentElement;
  var reduce = window.matchMedia("(prefers-reduced-motion: reduce)");

  // The wells scale to the room beside the photo (see layout()), so ROWS and CELL change with the width.
  var NARROW_COLS = 6, WIDE_COLS = 10; // phones keep the narrow well, wider screens get the wide one
  var WELL_HEIGHT = 165; // px: taller wells let the stack grow higher before the game restarts
  var COLS = NARROW_COLS, ROWS = 10, CELL = 14;
  var SHAPES = {
    I: [[1, 1, 1, 1]], O: [[1, 1], [1, 1]], T: [[0, 1, 0], [1, 1, 1]],
    S: [[0, 1, 1], [1, 1, 0]], Z: [[1, 1, 0], [0, 1, 1]], J: [[1, 0, 0], [1, 1, 1]], L: [[0, 0, 1], [1, 1, 1]]
  };
  var TYPES = Object.keys(SHAPES);

  function rotate(m) {
    return m[0].map(function (_, c) { return m.map(function (r) { return r[c]; }).reverse(); });
  }
  // Pieces use a range of shades of the brand colour (the theme's primary, orange in the "fire" scheme), so
  // they look related but not flat.
  var SHADES = { I: "300", O: "400", T: "500", S: "600", Z: "700", J: "400", L: "600" };
  function palette() {
    var style = getComputedStyle(document.documentElement), out = {};
    TYPES.forEach(function (t) {
      var v = style.getPropertyValue("--color-primary-" + SHADES[t]).trim();
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
      return -0.25 * sum - 0.4 * holes * 3 - 0.2 * bump + 0.8 * fullRows(g).length * 1.5 + Math.random() * 0.9;
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
      if (piece.hard) { // dropped by a click: slide over, then fall fast until it lands
        if (!collide(grid, piece.shape, piece.target, piece.y)) piece.x = piece.target;
        for (var k = 0; k < 3 && !collide(grid, piece.shape, piece.x, piece.y + 1); k++) piece.y++;
        if (!collide(grid, piece.shape, piece.x, piece.y + 1)) return;
      } else if (piece.x !== piece.target) {
        var nx = piece.x + (piece.target > piece.x ? 1 : -1);
        if (!collide(grid, piece.shape, nx, piece.y)) piece.x = nx;
      }
      if (!piece.hard) {
        if (++piece.tick % 2) return; // gravity: one row every second step
        if (!collide(grid, piece.shape, piece.x, piece.y + 1)) { piece.y++; return; }
      }
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
      // A click on column `col`: the falling piece moves there and drops until it lands.
      drop: function (col) {
        if (wait > 0 || flash > 0) return;
        if (!piece) spawn();
        if (!piece) return;
        piece.target = Math.max(0, Math.min(COLS - piece.shape[0].length, col - Math.floor(piece.shape[0].length / 2)));
        piece.hard = true;
        if (reduce.matches) { for (var i = 0; i < 40 && piece; i++) step(); } else step(); // no loop to animate it
        draw();
      },
      still: function () { // "reduce motion": play ahead without drawing, then show one frame
        for (var i = 0; i < 260; i++) step();
        piece = null; draw();
      }
    };
  }

  var canvases = ["left", "right"].map(function (side) {
    var canvas = document.createElement("canvas");
    canvas.className = "tetris-well tetris-" + side;
    canvas.setAttribute("aria-hidden", "true");
    header.appendChild(canvas);
    canvas.addEventListener("pointerdown", function (e) {
      var well = wells[canvases.indexOf(canvas)], box = canvas.getBoundingClientRect();
      if (well && box.width) well.drop(Math.floor((e.clientX - box.left) / box.width * COLS));
    });
    return canvas;
  });
  var wells = [];

  // Fits the wells into the room beside the photo at any width. The room is measured from the photo to the
  // edge of the window (the header itself is narrower than the window). Full size (14 px cells, 40 px away from
  // the photo) when there is space; smaller cells and a tighter gap on narrow screens. Rebuilds the wells only
  // if their size changed.
  function layout() {
    var photo = avatar.getBoundingClientRect();
    var box = header.getBoundingClientRect();
    var radius = (photo.width || 144) / 2;
    var centre = photo.left + photo.width / 2;
    var viewport = document.documentElement.clientWidth;
    var room = Math.min(centre, viewport - centre) - radius - 8; // free space beside the photo, 8 px from the edge
    var cols = room >= WIDE_COLS * 14 + 40 ? WIDE_COLS : NARROW_COLS; // wider wells when the screen is not a phone
    var gap = room >= cols * 14 + 40 ? 40 : 8;
    var cell = Math.max(5, Math.min(14, Math.floor((room - gap) / cols)));
    var rows = Math.max(8, Math.floor(WELL_HEIGHT / cell));
    var w = cols * cell, h = rows * cell;
    var top = avatar.offsetTop + avatar.offsetHeight - h; // bottom of the well level with the bottom of the photo
    var local = centre - box.left; // photo centre, relative to the header
    canvases.forEach(function (canvas, i) {
      canvas.style.width = w + "px";
      canvas.style.height = h + "px";
      canvas.style.top = top + "px";
      canvas.style.left = (i === 0 ? local - radius - gap - w : local + radius + gap) + "px";
    });
    if (wells.length && cell === CELL && rows === ROWS && cols === COLS) return;
    CELL = cell; ROWS = rows; COLS = cols;
    wells = [Well(canvases[0], 0), Well(canvases[1], 14)];
    if (reduce.matches) wells.forEach(function (w) { w.still(); });
  }
  layout();
  // The photo and the fonts finish loading after the script runs, which moves things: measure again.
  if (!avatar.complete) avatar.addEventListener("load", layout);
  window.addEventListener("load", layout);
  if ("ResizeObserver" in window) new ResizeObserver(layout).observe(header);

  var timer = null, onScreen = true;
  function enabled() { return document.documentElement.classList.contains("fx-tetris"); }
  function run() {
    if (reduce.matches) return; // one still frame, no loop
    var should = enabled() && onScreen && !document.hidden;
    if (should && !timer) timer = setInterval(function () { wells.forEach(function (w) { w.tick(); }); }, 90);
    if (!should && timer) { clearInterval(timer); timer = null; }
  }
  if ("IntersectionObserver" in window) {
    new IntersectionObserver(function (e) { onScreen = e[0].isIntersecting; run(); }).observe(header);
  }
  document.addEventListener("visibilitychange", run);
  var resizing = null;
  window.addEventListener("resize", function () {
    clearTimeout(resizing);
    resizing = setTimeout(layout, 120);
  });
  run();
})();
