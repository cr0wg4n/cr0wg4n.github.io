// Reactions and comments for a post. Talks to the API in api/ (see layouts/partials/comments.html).
// Everything from the server is inserted with textContent, never as HTML.
(function () {
  var root = document.getElementById("comments");
  if (!root) return;
  var api = root.dataset.api.replace(/\/$/, "");
  var path = root.dataset.path;
  var lang = root.dataset.lang === "es" ? "es" : "en";
  var T = {
    en: {
      empty: "No comments yet. Be the first.",
      sending: "Sending…",
      sent: "Thanks! Your comment will appear after I review it.",
      tooMany: "Too many attempts. Try again in a while.",
      failed: "Something went wrong. Please try again.",
      already: "You already used that reaction on this page.",
      loadFailed: "Comments could not be loaded right now."
    },
    es: {
      empty: "Todavía no hay comentarios. Sé el primero.",
      sending: "Enviando…",
      sent: "¡Gracias! Tu comentario aparecerá cuando lo revise.",
      tooMany: "Demasiados intentos. Prueba de nuevo más tarde.",
      failed: "Algo salió mal. Inténtalo otra vez.",
      already: "Ya usaste esa reacción en esta página.",
      loadFailed: "No se pudieron cargar los comentarios ahora."
    }
  }[lang];

  function track(name, params) {
    if (typeof window.gtag !== "function") return;
    params.transport_type = "beacon";
    params.language = lang;
    window.gtag("event", name, params);
  }

  function request(method, route, payload) {
    var options = { method: method, headers: {} };
    if (payload) {
      options.headers["content-type"] = "application/json";
      options.body = JSON.stringify(payload);
    }
    return fetch(api + route, options).then(function (res) {
      return res.json().catch(function () { return {}; }).then(function (data) {
        return { status: res.status, data: data };
      });
    });
  }

  // ---- reactions ---------------------------------------------------------------------------------------------
  var reactionButtons = root.querySelectorAll("[data-reaction]");
  function showCounts(counts) {
    Array.prototype.forEach.call(reactionButtons, function (button) {
      var n = counts && counts[button.dataset.reaction];
      button.querySelector(".cm-count").textContent = n ? String(n) : "";
    });
  }
  function remembered(kind) {
    try { return localStorage.getItem("cm:" + path + ":" + kind) === "1"; } catch (e) { return false; }
  }
  function remember(kind) {
    try { localStorage.setItem("cm:" + path + ":" + kind, "1"); } catch (e) { /* private mode */ }
  }
  Array.prototype.forEach.call(reactionButtons, function (button) {
    var kind = button.dataset.reaction;
    if (remembered(kind)) button.setAttribute("aria-pressed", "true");
    button.addEventListener("click", function () {
      if (button.getAttribute("aria-pressed") === "true") return;
      button.disabled = true;
      request("POST", "/reactions", { path: path, kind: kind }).then(function (r) {
        if (r.status === 200) {
          showCounts(r.data.reactions);
          button.setAttribute("aria-pressed", "true");
          remember(kind);
          if (r.data.added) track("reaction", { reaction: kind });
        } else if (r.status === 429) {
          setStatus(T.tooMany);
        } else {
          setStatus(T.failed);
        }
      }).catch(function () { setStatus(T.failed); }).then(function () { button.disabled = false; });
    });
  });

  // ---- comments ----------------------------------------------------------------------------------------------
  var list = root.querySelector(".cm-list");
  var status = root.querySelector(".cm-status");
  function setStatus(text) { status.textContent = text; }

  function renderComments(comments) {
    list.textContent = "";
    if (!comments.length) {
      var empty = document.createElement("p");
      empty.className = "cm-empty";
      empty.textContent = T.empty;
      list.appendChild(empty);
      return;
    }
    comments.forEach(function (c) {
      var item = document.createElement("article");
      item.className = "cm-item";
      var head = document.createElement("header");
      var name = document.createElement("strong");
      name.textContent = c.name;
      var when = document.createElement("time");
      var date = new Date(c.created);
      when.dateTime = date.toISOString();
      when.textContent = date.toLocaleDateString(lang === "es" ? "es-ES" : "en-US", { year: "numeric", month: "short", day: "numeric" });
      head.appendChild(name);
      head.appendChild(when);
      var body = document.createElement("p");
      body.textContent = c.body;
      item.appendChild(head);
      item.appendChild(body);
      list.appendChild(item);
    });
  }

  function load() {
    var q = "?path=" + encodeURIComponent(path);
    request("GET", "/reactions" + q).then(function (r) { if (r.status === 200) showCounts(r.data.reactions); }).catch(function () {});
    request("GET", "/comments" + q).then(function (r) {
      if (r.status === 200) renderComments(r.data.comments || []);
      else setStatus(T.loadFailed);
    }).catch(function () { setStatus(T.loadFailed); });
  }

  var form = root.querySelector("form");
  var shownAt = Date.now();
  form.addEventListener("submit", function (e) {
    e.preventDefault();
    var button = form.querySelector("button[type=submit]");
    button.disabled = true;
    setStatus(T.sending);
    request("POST", "/comments", {
      path: path,
      name: form.elements.name.value,
      body: form.elements.body.value,
      website: form.elements.website.value,
      ts: shownAt
    }).then(function (r) {
      if (r.status === 202) {
        setStatus(T.sent);
        form.reset();
        track("comment_submit", {});
      } else if (r.status === 429) {
        setStatus(T.tooMany);
      } else {
        setStatus(T.failed);
      }
    }).catch(function () { setStatus(T.failed); }).then(function () { button.disabled = false; });
  });

  // Nothing is requested until the section is close to the screen.
  if ("IntersectionObserver" in window) {
    var io = new IntersectionObserver(function (entries) {
      if (entries.some(function (e) { return e.isIntersecting; })) { io.disconnect(); load(); }
    }, { rootMargin: "400px 0px" });
    io.observe(root);
  } else {
    load();
  }
})();
