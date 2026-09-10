/* Bitloom FR117 typed IDE waveform viewer (subset B — not Tywaves). */
(function () {
  var root = document.getElementById("root");
  if (!root || root.getAttribute("data-bitloom-typed-wave") !== "1") return;
  var raw = document.getElementById("typed-wave-data");
  if (!raw) return;
  var data = JSON.parse(raw.textContent);
  var signals = data.signals || [];
  var times = data.times || [];
  var values = data.values || {};
  var tree = document.getElementById("signal-tree");
  var meta = document.getElementById("signal-meta");
  var search = document.getElementById("typed-search");
  var canvas = document.getElementById("typed-wave-canvas");
  var selected = signals.length ? signals[0].name : null;

  function matches(sig, q) {
    if (!q) return true;
    q = q.toLowerCase();
    return (
      (sig.name && sig.name.toLowerCase().indexOf(q) >= 0) ||
      (sig.ty && sig.ty.toLowerCase().indexOf(q) >= 0) ||
      (sig.kind && sig.kind.toLowerCase().indexOf(q) >= 0) ||
      (sig.module && sig.module.toLowerCase().indexOf(q) >= 0)
    );
  }

  function renderTree() {
    if (!tree) return;
    var q = search ? search.value : "";
    tree.innerHTML = "";
    var byMod = {};
    signals.forEach(function (sig) {
      if (!matches(sig, q)) return;
      var mod = sig.module || "(top)";
      if (!byMod[mod]) byMod[mod] = [];
      byMod[mod].push(sig);
    });
    Object.keys(byMod)
      .sort()
      .forEach(function (mod) {
        var li = document.createElement("li");
        li.className = "mod-node";
        li.innerHTML =
          '<span class="mod-label" data-module="' +
          mod +
          '">' +
          mod +
          "</span>";
        var ul = document.createElement("ul");
        byMod[mod].forEach(function (sig) {
          var sli = document.createElement("li");
          sli.className =
            "sig-node" + (sig.name === selected ? " selected" : "");
          sli.setAttribute("data-signal-name", sig.name);
          sli.setAttribute("data-signal-type", sig.ty || "");
          sli.setAttribute("data-signal-kind", sig.kind || "");
          sli.innerHTML =
            '<button type="button" class="sig-btn">' +
            '<span class="sig-name">' +
            sig.name +
            "</span> " +
            '<span class="sig-type">' +
            (sig.ty || "?") +
            "</span> " +
            '<span class="sig-kind">' +
            (sig.kind || "") +
            "</span>" +
            "</button>";
          sli.querySelector("button").addEventListener("click", function () {
            selected = sig.name;
            renderTree();
            renderMeta();
            draw();
          });
          ul.appendChild(sli);
        });
        li.appendChild(ul);
        tree.appendChild(li);
      });
  }

  function renderMeta() {
    if (!meta) return;
    var sig = signals.find(function (s) {
      return s.name === selected;
    });
    if (!sig) {
      meta.innerHTML = "<em>Select a typed signal</em>";
      return;
    }
    meta.innerHTML =
      "<dl>" +
      "<dt>Name</dt><dd>" +
      sig.name +
      "</dd>" +
      "<dt>Type</dt><dd data-signal-type=\"" +
      sig.ty +
      "\">" +
      sig.ty +
      "</dd>" +
      "<dt>Kind</dt><dd data-signal-kind=\"" +
      sig.kind +
      "\">" +
      sig.kind +
      "</dd>" +
      "<dt>Module</dt><dd>" +
      (sig.module || "") +
      "</dd>" +
      "<dt>Width</dt><dd>" +
      (sig.width != null ? sig.width : "") +
      "</dd>" +
      "</dl>";
  }

  function draw() {
    if (!canvas || !selected) return;
    var ctx = canvas.getContext("2d");
    var w = canvas.width;
    var h = canvas.height;
    ctx.clearRect(0, 0, w, h);
    ctx.fillStyle = "#fff";
    ctx.fillRect(0, 0, w, h);
    var series = values[selected] || [];
    if (!series.length || !times.length) {
      ctx.fillStyle = "#666";
      ctx.fillText("(no samples)", 16, 24);
      return;
    }
    var maxV = 1;
    series.forEach(function (v) {
      if (v > maxV) maxV = v;
    });
    var pad = 24;
    var plotW = w - pad * 2;
    var plotH = h - pad * 2;
    ctx.strokeStyle = "#0b3d5c";
    ctx.lineWidth = 2;
    ctx.beginPath();
    for (var i = 0; i < series.length; i++) {
      var x =
        pad +
        (times.length === 1 ? plotW / 2 : (i / (times.length - 1)) * plotW);
      var y = pad + plotH - (series[i] / maxV) * plotH;
      if (i === 0) ctx.moveTo(x, y);
      else ctx.lineTo(x, y);
    }
    ctx.stroke();
    ctx.fillStyle = "#456";
    ctx.font = "12px system-ui,sans-serif";
    ctx.fillText(selected + " · typed timeline", pad, 14);
  }

  if (search) {
    search.addEventListener("input", function () {
      renderTree();
    });
  }
  renderTree();
  renderMeta();
  draw();
})();
