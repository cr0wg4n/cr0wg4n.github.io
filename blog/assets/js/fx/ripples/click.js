// Effect "ripples", click part: the orange rings of the home avatar, bursting from wherever the user clicks or
// taps. Styles: .click-ripple in assets/css/custom.css. Loaded only when [effects] ripples = true.
(function () {
  var reduce = window.matchMedia("(prefers-reduced-motion: reduce)");
  document.addEventListener(
    "pointerdown",
    function (e) {
      if (reduce.matches || (e.pointerType === "mouse" && e.button !== 0)) return;
      var ripple = document.createElement("span");
      ripple.className = "click-ripple";
      ripple.style.left = e.clientX + "px";
      ripple.style.top = e.clientY + "px";
      ripple.addEventListener("animationend", function () { ripple.remove(); });
      document.body.appendChild(ripple);
    },
    { passive: true }
  );
})();
