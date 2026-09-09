(function () {
  var data = JSON.parse(document.getElementById("wave-data").textContent);
  var canvas = document.getElementById("wave-canvas");
  var ctx = canvas.getContext("2d");
  var search = document.getElementById("signal-search");
  var viewportLabel = document.getElementById("viewport-label");
  var allNames = Object.keys(data.signals).sort();
  var times = data.times;
  var tMin = times.length ? times[0] : 0;
  var tMax = times.length ? times[times.length - 1] : 1;
  if (tMax <= tMin) tMax = tMin + 1;
  var viewStart = tMin;
  var viewEnd = tMax;
  var drag = null;

  function filteredNames() {
    var q = (search.value || "").trim().toLowerCase();
    if (!q) return allNames.slice();
    return allNames.filter(function (n) {
      return n.toLowerCase().indexOf(q) !== -1;
    });
  }

  function zoom(factor, center) {
    var mid = center != null ? center : (viewStart + viewEnd) / 2;
    var half = (viewEnd - viewStart) / 2 / factor;
    viewStart = Math.max(tMin, mid - half);
    viewEnd = Math.min(tMax, mid + half);
    if (viewEnd - viewStart < 1e-9) {
      viewStart = mid - 0.5;
      viewEnd = mid + 0.5;
    }
    draw();
  }

  function pan(deltaFrac) {
    var span = viewEnd - viewStart;
    var d = span * deltaFrac;
    if (viewStart + d < tMin) d = tMin - viewStart;
    if (viewEnd + d > tMax) d = tMax - viewEnd;
    viewStart += d;
    viewEnd += d;
    draw();
  }

  function xForTime(t) {
    return ((t - viewStart) / (viewEnd - viewStart)) * canvas.width;
  }

  function timeForX(x) {
    return viewStart + (x / canvas.width) * (viewEnd - viewStart);
  }

  function draw() {
    var names = filteredNames();
    var w = canvas.width;
    var h = canvas.height;
    ctx.clearRect(0, 0, w, h);
    ctx.fillStyle = "#fff";
    ctx.fillRect(0, 0, w, h);
    var rowH = names.length
      ? Math.max(18, Math.min(40, (h - 24) / names.length))
      : 24;
    ctx.strokeStyle = "#dde3ea";
    ctx.beginPath();
    for (var gx = 0; gx <= 10; gx++) {
      var xg = (gx / 10) * w;
      ctx.moveTo(xg, 0);
      ctx.lineTo(xg, h);
    }
    ctx.stroke();
    names.forEach(function (name, i) {
      var y0 = 12 + i * rowH;
      var midY = y0 + rowH * 0.55;
      ctx.fillStyle = "#234";
      ctx.font = "12px system-ui,sans-serif";
      ctx.fillText(name, 6, y0 + 12);
      var vals = data.signals[name] || [];
      ctx.strokeStyle = "#0b3d5c";
      ctx.lineWidth = 1.5;
      ctx.beginPath();
      var started = false;
      for (var ti = 0; ti < times.length; ti++) {
        var t = times[ti];
        if (t < viewStart && ti + 1 < times.length && times[ti + 1] < viewStart)
          continue;
        if (t > viewEnd && ti > 0 && times[ti - 1] > viewEnd) break;
        var v = vals[ti] || 0;
        var x = xForTime(t);
        var y = midY - (v ? rowH * 0.25 : -rowH * 0.15);
        if (!started) {
          ctx.moveTo(x, y);
          started = true;
        } else {
          ctx.lineTo(x, y);
        }
        if (ti + 1 < times.length) {
          var x2 = xForTime(
            Math.min(times[ti + 1], viewEnd + (viewEnd - viewStart))
          );
          ctx.lineTo(x2, y);
        }
      }
      ctx.stroke();
    });
    viewportLabel.textContent =
      "viewport [" +
      viewStart.toFixed(0) +
      " .. " +
      viewEnd.toFixed(0) +
      "] · " +
      names.length +
      "/" +
      allNames.length +
      " signals";
  }

  search.addEventListener("input", draw);
  document.getElementById("zoom-in").addEventListener("click", function () {
    zoom(1.5);
  });
  document.getElementById("zoom-out").addEventListener("click", function () {
    zoom(1 / 1.5);
  });
  document.getElementById("pan-left").addEventListener("click", function () {
    pan(-0.25);
  });
  document.getElementById("pan-right").addEventListener("click", function () {
    pan(0.25);
  });
  document.getElementById("zoom-fit").addEventListener("click", function () {
    viewStart = tMin;
    viewEnd = tMax;
    draw();
  });
  canvas.addEventListener(
    "wheel",
    function (e) {
      e.preventDefault();
      zoom(e.deltaY < 0 ? 1.2 : 1 / 1.2, timeForX(e.offsetX));
    },
    { passive: false }
  );
  canvas.addEventListener("mousedown", function (e) {
    drag = { x: e.offsetX, start: viewStart, end: viewEnd };
  });
  window.addEventListener("mouseup", function () {
    drag = null;
  });
  canvas.addEventListener("mousemove", function (e) {
    if (!drag) return;
    var span = drag.end - drag.start;
    var dx = ((e.offsetX - drag.x) / canvas.width) * span;
    viewStart = Math.max(tMin, Math.min(tMax - span, drag.start - dx));
    viewEnd = viewStart + span;
    draw();
  });
  draw();
})();
