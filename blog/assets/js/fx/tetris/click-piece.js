// Effect "tetris", click part: a random tetromino at every click or tap. On most pages it falls from the top of
// the window and lands where the user clicked; on the home pages (/ and /es/) it does not fall, it appears
// centred on the click and fades. Styles: .click-piece in assets/css/custom.css. Loaded only when
// [effects] tetris = true.
(function () {
  var reduce = window.matchMedia("(prefers-reduced-motion: reduce)");
  var isHome = /^\/(es\/)?$/.test(location.pathname);
  var CELL = 14;
  // [column, row] of each square, and the primary shade (the same shades as home-wells.js)
  var PIECES = [
    { shade: 300, cells: [[0, 0], [1, 0], [2, 0], [3, 0]] },
    { shade: 400, cells: [[0, 0], [1, 0], [0, 1], [1, 1]] },
    { shade: 500, cells: [[0, 0], [1, 0], [2, 0], [1, 1]] },
    { shade: 600, cells: [[1, 0], [2, 0], [0, 1], [1, 1]] },
    { shade: 700, cells: [[0, 0], [1, 0], [1, 1], [2, 1]] },
    { shade: 400, cells: [[0, 0], [0, 1], [1, 1], [2, 1]] },
    { shade: 600, cells: [[2, 0], [0, 1], [1, 1], [2, 1]] }
  ];

  function size(piece, axis) {
    return Math.max.apply(null, piece.cells.map(function (c) { return c[axis]; })) + 1;
  }

  function burst(x, y) {
    var piece = PIECES[Math.floor(Math.random() * PIECES.length)];
    var cols = size(piece, 0), rows = size(piece, 1);
    var el = document.createElement("span");
    el.className = "click-piece" + (isHome ? " click-piece-still" : "");
    el.style.left = x + "px";
    el.style.top = y + "px";
    el.style.setProperty("--fall", y + rows * CELL + "px"); // from just above the window to the click
    piece.cells.forEach(function (c) {
      var square = document.createElement("i");
      square.style.left = (c[0] - cols / 2) * CELL + "px";
      square.style.top = (c[1] - rows / 2) * CELL + "px";
      square.style.background = "rgb(var(--color-primary-" + piece.shade + "))";
      el.appendChild(square);
    });
    el.addEventListener("animationend", function () { el.remove(); });
    document.body.appendChild(el);
  }

  document.addEventListener(
    "pointerdown",
    function (e) {
      if (reduce.matches || (e.pointerType === "mouse" && e.button !== 0)) return;
      if (e.target.closest && e.target.closest(".tetris-well")) return; // the wells have their own drop
      burst(e.clientX, e.clientY);
    },
    { passive: true }
  );
})();
