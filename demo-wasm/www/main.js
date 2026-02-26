const clamp = (v, min, max) => Math.min(max, Math.max(min, v));
const lerp = (a, b, t) => a + (b - a) * t;

const easeOutBounce = (t) => {
  const n1 = 7.5625;
  const d1 = 2.75;
  if (t < 1 / d1) return n1 * t * t;
  if (t < 2 / d1) return n1 * (t - 1.5 / d1) ** 2 + 0.75;
  if (t < 2.5 / d1) return n1 * (t - 2.25 / d1) ** 2 + 0.9375;
  return n1 * (t - 2.625 / d1) ** 2 + 0.984375;
};

const cubicBezier = (x, x1, y1, x2, y2) => {
  if (x <= 0) return 0;
  if (x >= 1) return 1;

  const component = (s, p1, p2) => {
    const u = 1 - s;
    return 3 * u * u * s * p1 + 3 * u * s * s * p2 + s ** 3;
  };
  const derivative = (s, p1, p2) => {
    const u = 1 - s;
    return 3 * u * u * p1 + 6 * u * s * (p2 - p1) + 3 * s * s * (1 - p2);
  };

  let s = x;
  for (let i = 0; i < 8; i += 1) {
    const bx = component(s, x1, x2);
    const dbx = derivative(s, x1, x2);
    if (Math.abs(dbx) < 1e-7) break;
    s = clamp(s - (bx - x) / dbx, 0, 1);
  }
  if (Math.abs(component(s, x1, x2) - x) > 1e-4) {
    let lo = 0;
    let hi = 1;
    for (let i = 0; i < 20; i += 1) {
      s = (lo + hi) * 0.5;
      const bx = component(s, x1, x2);
      if (bx < x) lo = s;
      else hi = s;
    }
  }
  return component(s, y1, y2);
};

const easingFns = {
  linear: (t) => t,
  easeInQuad: (t) => t * t,
  easeOutQuad: (t) => 1 - (1 - t) ** 2,
  easeInOutQuad: (t) => (t < 0.5 ? 2 * t * t : 1 - ((-2 * t + 2) ** 2) / 2),
  easeInCubic: (t) => t ** 3,
  easeOutCubic: (t) => 1 - (1 - t) ** 3,
  easeInOutCubic: (t) => (t < 0.5 ? 4 * t ** 3 : 1 - ((-2 * t + 2) ** 3) / 2),
  easeInQuart: (t) => t ** 4,
  easeOutQuart: (t) => 1 - (1 - t) ** 4,
  easeInOutQuart: (t) => (t < 0.5 ? 8 * t ** 4 : 1 - ((-2 * t + 2) ** 4) / 2),
  easeInQuint: (t) => t ** 5,
  easeOutQuint: (t) => 1 - (1 - t) ** 5,
  easeInOutQuint: (t) => (t < 0.5 ? 16 * t ** 5 : 1 - ((-2 * t + 2) ** 5) / 2),
  easeInSine: (t) => 1 - Math.cos((t * Math.PI) / 2),
  easeOutSine: (t) => Math.sin((t * Math.PI) / 2),
  easeInOutSine: (t) => -(Math.cos(Math.PI * t) - 1) / 2,
  easeInExpo: (t) => (t === 0 ? 0 : 2 ** (10 * t - 10)),
  easeOutExpo: (t) => (t === 1 ? 1 : 1 - 2 ** (-10 * t)),
  easeInOutExpo: (t) => {
    if (t === 0) return 0;
    if (t === 1) return 1;
    return t < 0.5 ? 2 ** (20 * t - 10) / 2 : (2 - 2 ** (-20 * t + 10)) / 2;
  },
  easeInCirc: (t) => 1 - Math.sqrt(1 - t * t),
  easeOutCirc: (t) => Math.sqrt(1 - (t - 1) ** 2),
  easeInOutCirc: (t) =>
    t < 0.5 ? (1 - Math.sqrt(1 - (2 * t) ** 2)) / 2 : (Math.sqrt(1 - (-2 * t + 2) ** 2) + 1) / 2,
  easeInBack: (t) => {
    const c1 = 1.70158;
    const c3 = c1 + 1;
    return c3 * t ** 3 - c1 * t ** 2;
  },
  easeOutBack: (t) => {
    const c1 = 1.70158;
    const c3 = c1 + 1;
    return 1 + c3 * (t - 1) ** 3 + c1 * (t - 1) ** 2;
  },
  easeInOutBack: (t) => {
    const c1 = 1.70158;
    const c2 = c1 * 1.525;
    return t < 0.5
      ? ((2 * t) ** 2 * ((c2 + 1) * 2 * t - c2)) / 2
      : (((2 * t - 2) ** 2 * ((c2 + 1) * (2 * t - 2) + c2)) + 2) / 2;
  },
  easeInElastic: (t) => {
    if (t === 0 || t === 1) return t;
    const c4 = (2 * Math.PI) / 3;
    return -(2 ** (10 * t - 10)) * Math.sin((10 * t - 10.75) * c4);
  },
  easeOutElastic: (t) => {
    if (t === 0 || t === 1) return t;
    const c4 = (2 * Math.PI) / 3;
    return 2 ** (-10 * t) * Math.sin((10 * t - 0.75) * c4) + 1;
  },
  easeInOutElastic: (t) => {
    if (t === 0 || t === 1) return t;
    const c5 = (2 * Math.PI) / 4.5;
    return t < 0.5
      ? -(2 ** (20 * t - 10) * Math.sin((20 * t - 11.125) * c5)) / 2
      : (2 ** (-20 * t + 10) * Math.sin((20 * t - 11.125) * c5)) / 2 + 1;
  },
  easeInBounce: (t) => 1 - easeOutBounce(1 - t),
  easeOutBounce,
  easeInOutBounce: (t) =>
    t < 0.5 ? (1 - easeOutBounce(1 - 2 * t)) / 2 : (1 + easeOutBounce(2 * t - 1)) / 2,
  cubicBezierEase: (t) => cubicBezier(t, 0.25, 0.1, 0.25, 1.0),
};

const easingNames = Object.keys(easingFns);

const wasmStatusLine = document.getElementById("wasm-status");
const wasmSnapshotLine = document.getElementById("wasm-snapshot");

let wasmDemo = null;
let wasmReady = false;

const setWasmStatus = (text, isError = false) => {
  wasmStatusLine.textContent = text;
  wasmStatusLine.classList.toggle("error", isError);
};

const renderWasmSnapshot = () => {
  if (!wasmDemo || !wasmReady) return;
  const state = wasmDemo.render_state();
  const tween = Number.isFinite(state[0]) ? state[0] : 0;
  const spring = Number.isFinite(state[1]) ? state[1] : 0;
  const active = Number.isFinite(state[2]) ? state[2] : 0;
  wasmSnapshotLine.textContent = `tween=${tween.toFixed(3)} spring=${spring.toFixed(3)} active=${Math.round(active)}`;
};

const initWasm = async () => {
  try {
    const wasmModule = await import("./pkg/easel_demo_wasm.js");
    await wasmModule.default();
    wasmDemo = new wasmModule.Demo();
    wasmReady = true;
    setWasmStatus("WASM core: ready");
    renderWasmSnapshot();
  } catch (error) {
    wasmReady = false;
    wasmDemo = null;
    setWasmStatus("WASM core: unavailable (run wasm-pack build)", true);
    wasmSnapshotLine.textContent = "tween=0.000 spring=0.000 active=0";
    console.warn("WASM init failed:", error);
  }
};

void initWasm();

// Tab switching
const tabButtons = [...document.querySelectorAll(".tab-button")];
const tabPanels = [...document.querySelectorAll(".tab-panel")];
let activeTab = "easing";

for (const button of tabButtons) {
  button.addEventListener("click", () => {
    activeTab = button.dataset.tab;
    for (const item of tabButtons) item.classList.toggle("active", item === button);
    for (const panel of tabPanels) panel.classList.toggle("active", panel.id === `tab-${activeTab}`);
  });
}

// Tab 1: Easing curves
const curveGrid = document.getElementById("curve-grid");
const selectedNameLabel = document.getElementById("selected-easing-name");
const previewCanvas = document.getElementById("easing-preview");
const previewCtx = previewCanvas.getContext("2d");
const easingProgress = document.getElementById("easing-progress");
let selectedEasing = "linear";
let easingStartTs = performance.now();

const drawCurve = (ctx, width, height, fn, stroke) => {
  const pad = 16;
  ctx.beginPath();
  for (let i = 0; i <= 100; i += 1) {
    const t = i / 100;
    const x = pad + t * (width - pad * 2);
    const y = height - pad - fn(t) * (height - pad * 2);
    if (i === 0) ctx.moveTo(x, y);
    else ctx.lineTo(x, y);
  }
  ctx.strokeStyle = stroke;
  ctx.lineWidth = 2;
  ctx.stroke();
};

for (const name of easingNames) {
  const item = document.createElement("button");
  item.type = "button";
  item.className = `curve-item ${name === selectedEasing ? "active" : ""}`;
  item.innerHTML = `<canvas width="116" height="62"></canvas><span>${name}</span>`;
  const canvas = item.querySelector("canvas");
  const ctx = canvas.getContext("2d");
  ctx.clearRect(0, 0, canvas.width, canvas.height);
  drawCurve(ctx, canvas.width, canvas.height, easingFns[name], "#0e7490");
  item.addEventListener("click", () => {
    selectedEasing = name;
    selectedNameLabel.textContent = name;
    for (const node of curveGrid.children) node.classList.remove("active");
    item.classList.add("active");
    easingStartTs = performance.now();
  });
  curveGrid.append(item);
}

const renderEasingPreview = (ts) => {
  const fn = easingFns[selectedEasing];
  const cycleMs = 2200;
  const progress = ((ts - easingStartTs) % cycleMs) / cycleMs;
  const eased = clamp(fn(progress), -0.4, 1.4);
  easingProgress.style.width = `${progress * 100}%`;

  const w = previewCanvas.width;
  const h = previewCanvas.height;
  previewCtx.clearRect(0, 0, w, h);
  previewCtx.fillStyle = "#f8fafc";
  previewCtx.fillRect(0, 0, w, h);
  previewCtx.strokeStyle = "#334155";
  previewCtx.lineWidth = 1.2;
  previewCtx.strokeRect(12, 12, w - 24, h - 24);
  drawCurve(previewCtx, w, h - 50, fn, "#0f766e");

  const graphPad = 16;
  const px = graphPad + progress * (w - graphPad * 2);
  const py = h - 50 - graphPad - clamp(fn(progress), -0.2, 1.2) * (h - 82);
  previewCtx.fillStyle = "#f97316";
  previewCtx.beginPath();
  previewCtx.arc(px, py, 6, 0, Math.PI * 2);
  previewCtx.fill();

  const left = 24;
  const right = w - 24;
  const trackY = h - 30;
  previewCtx.strokeStyle = "#0f172a";
  previewCtx.lineWidth = 2;
  previewCtx.beginPath();
  previewCtx.moveTo(left, trackY);
  previewCtx.lineTo(right, trackY);
  previewCtx.stroke();

  const bx = lerp(left, right, clamp(fn(progress), 0, 1));
  previewCtx.fillStyle = "#06b6d4";
  previewCtx.beginPath();
  previewCtx.arc(bx, trackY, 9, 0, Math.PI * 2);
  previewCtx.fill();
};

// Tab 2: Tween playground
const twFrom = document.getElementById("tw-from");
const twTo = document.getElementById("tw-to");
const twDuration = document.getElementById("tw-duration");
const twDurationVal = document.getElementById("tw-duration-val");
const twDelay = document.getElementById("tw-delay");
const twDelayVal = document.getElementById("tw-delay-val");
const twEasing = document.getElementById("tw-easing");
const twLoop = document.getElementById("tw-loop");
const twPlay = document.getElementById("tw-play");
const twPause = document.getElementById("tw-pause");
const twReset = document.getElementById("tw-reset");
const twBox = document.getElementById("tw-box");
const twProgress = document.getElementById("tw-progress");
const twValue = document.getElementById("tw-value");

for (const name of easingNames) {
  const opt = document.createElement("option");
  opt.value = name;
  opt.textContent = name;
  if (name === "easeOutCubic") opt.selected = true;
  twEasing.append(opt);
}

const tweenState = {
  from: 0,
  to: 420,
  duration: 60,
  delay: 0,
  delayRemaining: 0,
  elapsed: 0,
  direction: 1,
  easing: "easeOutCubic",
  loop: "once",
  playing: false,
};

const applyTweenControls = () => {
  tweenState.from = Number(twFrom.value);
  tweenState.to = Number(twTo.value);
  tweenState.duration = Number(twDuration.value);
  tweenState.delay = Number(twDelay.value);
  tweenState.easing = twEasing.value;
  tweenState.loop = twLoop.value;
  twDurationVal.textContent = String(tweenState.duration);
  twDelayVal.textContent = String(tweenState.delay);
};

const resetTween = () => {
  applyTweenControls();
  tweenState.elapsed = 0;
  tweenState.direction = 1;
  tweenState.delayRemaining = tweenState.delay;
  tweenState.playing = false;
};

const tickTween = () => {
  if (!tweenState.playing) return;
  if (tweenState.delayRemaining > 0) {
    tweenState.delayRemaining -= 1;
    return;
  }
  tweenState.elapsed += 1;
  if (tweenState.elapsed >= tweenState.duration) {
    if (tweenState.loop === "once") {
      tweenState.elapsed = tweenState.duration;
      tweenState.playing = false;
    } else if (tweenState.loop === "infinite") {
      tweenState.elapsed = 0;
      tweenState.direction = 1;
      tweenState.delayRemaining = tweenState.delay;
    } else {
      tweenState.elapsed = 0;
      tweenState.direction *= -1;
      tweenState.delayRemaining = tweenState.delay;
    }
  }
};

const tweenRawProgress = () => clamp(tweenState.elapsed / Math.max(1, tweenState.duration), 0, 1);

const tweenValueAtState = () => {
  const p = tweenRawProgress();
  const dirProgress = tweenState.direction > 0 ? p : 1 - p;
  const eased = clamp(easingFns[tweenState.easing](dirProgress), -0.4, 1.4);
  return lerp(tweenState.from, tweenState.to, eased);
};

const renderTween = () => {
  const stageWidth = twBox.parentElement.clientWidth;
  const minX = 14;
  const maxX = stageWidth - 14 - twBox.clientWidth;
  const value = tweenValueAtState();
  const range = tweenState.to - tweenState.from;
  const normalized = Math.abs(range) < 1e-6 ? 0 : (value - tweenState.from) / range;
  const x = lerp(minX, maxX, clamp(normalized, 0, 1));
  twBox.style.left = `${x}px`;
  twValue.textContent = `value: ${value.toFixed(2)} | progress: ${(tweenRawProgress() * 100).toFixed(1)}%`;
  twProgress.style.width = `${tweenRawProgress() * 100}%`;
};

for (const el of [twFrom, twTo, twDuration, twDelay, twEasing, twLoop]) {
  el.addEventListener("input", () => {
    applyTweenControls();
    renderTween();
  });
}

twPlay.addEventListener("click", () => {
  applyTweenControls();
  tweenState.playing = true;
});
twPause.addEventListener("click", () => {
  tweenState.playing = false;
});
twReset.addEventListener("click", () => {
  resetTween();
  renderTween();
});
resetTween();
renderTween();

// Tab 3: Spring physics
const springCanvas = document.getElementById("spring-canvas");
const springCtx = springCanvas.getContext("2d");
const spPreset = document.getElementById("sp-preset");
const spK = document.getElementById("sp-k");
const spC = document.getElementById("sp-c");
const spM = document.getElementById("sp-m");
const spKVal = document.getElementById("sp-k-val");
const spCVal = document.getElementById("sp-c-val");
const spMVal = document.getElementById("sp-m-val");
const spReset = document.getElementById("sp-reset");
const spRest = document.getElementById("sp-rest");
const spValueGraph = document.getElementById("sp-value-graph").getContext("2d");
const spVelGraph = document.getElementById("sp-vel-graph").getContext("2d");

const springPresets = {
  gentle: { stiffness: 120, damping: 14, mass: 1 },
  wobbly: { stiffness: 180, damping: 12, mass: 1 },
  stiff: { stiffness: 400, damping: 28, mass: 1 },
  slow: { stiffness: 80, damping: 20, mass: 1 },
  molasses: { stiffness: 60, damping: 30, mass: 2 },
};

const springState = {
  pos: { x: springCanvas.width * 0.5, y: springCanvas.height * 0.5 },
  vel: { x: 0, y: 0 },
  target: { x: springCanvas.width * 0.5, y: springCanvas.height * 0.5 },
  restThreshold: 0.02,
  atRest: false,
  historyPos: [],
  historyVel: [],
};

const applySpringControls = () => {
  spKVal.textContent = spK.value;
  spCVal.textContent = spC.value;
  spMVal.textContent = Number(spM.value).toFixed(1);
};

const setSpringPreset = (name) => {
  const p = springPresets[name];
  spK.value = String(p.stiffness);
  spC.value = String(p.damping);
  spM.value = String(p.mass);
  applySpringControls();
};

spPreset.addEventListener("change", () => setSpringPreset(spPreset.value));
for (const el of [spK, spC, spM]) el.addEventListener("input", applySpringControls);
spReset.addEventListener("click", () => {
  springState.pos.x = springCanvas.width * 0.5;
  springState.pos.y = springCanvas.height * 0.5;
  springState.target.x = springState.pos.x;
  springState.target.y = springState.pos.y;
  springState.vel.x = 0;
  springState.vel.y = 0;
  springState.atRest = false;
});

springCanvas.addEventListener("mousemove", (ev) => {
  const rect = springCanvas.getBoundingClientRect();
  springState.target.x = (ev.clientX - rect.left) * (springCanvas.width / rect.width);
  springState.target.y = (ev.clientY - rect.top) * (springCanvas.height / rect.height);
  springState.atRest = false;
});

setSpringPreset("gentle");

const stepSpring = () => {
  const dt = 1 / 60;
  const stiffness = Number(spK.value);
  const damping = Number(spC.value);
  const mass = Number(spM.value);
  const forceX = -stiffness * (springState.pos.x - springState.target.x) - damping * springState.vel.x;
  const forceY = -stiffness * (springState.pos.y - springState.target.y) - damping * springState.vel.y;
  const ax = forceX / mass;
  const ay = forceY / mass;
  springState.vel.x += ax * dt;
  springState.vel.y += ay * dt;
  springState.pos.x += springState.vel.x * dt;
  springState.pos.y += springState.vel.y * dt;

  const dx = springState.pos.x - springState.target.x;
  const dy = springState.pos.y - springState.target.y;
  springState.atRest =
    Math.abs(springState.vel.x) < springState.restThreshold &&
    Math.abs(springState.vel.y) < springState.restThreshold &&
    Math.abs(dx) < springState.restThreshold &&
    Math.abs(dy) < springState.restThreshold;

  springState.historyPos.push(dx);
  springState.historyVel.push(springState.vel.x);
  if (springState.historyPos.length > 180) springState.historyPos.shift();
  if (springState.historyVel.length > 180) springState.historyVel.shift();
};

const drawSparkline = (ctx, values, color, label) => {
  const { canvas } = ctx;
  ctx.clearRect(0, 0, canvas.width, canvas.height);
  ctx.fillStyle = "#f8fafc";
  ctx.fillRect(0, 0, canvas.width, canvas.height);
  ctx.strokeStyle = "#334155";
  ctx.strokeRect(0.5, 0.5, canvas.width - 1, canvas.height - 1);

  if (values.length < 2) return;
  const maxAbs = values.reduce((m, v) => Math.max(m, Math.abs(v)), 1);
  ctx.beginPath();
  values.forEach((value, i) => {
    const x = (i / (values.length - 1)) * canvas.width;
    const y = canvas.height * 0.5 - (value / maxAbs) * (canvas.height * 0.42);
    if (i === 0) ctx.moveTo(x, y);
    else ctx.lineTo(x, y);
  });
  ctx.strokeStyle = color;
  ctx.lineWidth = 2;
  ctx.stroke();

  ctx.fillStyle = "#0f172a";
  ctx.font = "12px 'Space Mono', monospace";
  ctx.fillText(label, 8, 14);
};

const renderSpring = () => {
  springCtx.clearRect(0, 0, springCanvas.width, springCanvas.height);
  springCtx.fillStyle = "#f8fafc";
  springCtx.fillRect(0, 0, springCanvas.width, springCanvas.height);
  springCtx.strokeStyle = "#334155";
  springCtx.strokeRect(0.5, 0.5, springCanvas.width - 1, springCanvas.height - 1);

  springCtx.strokeStyle = "#94a3b8";
  springCtx.beginPath();
  springCtx.moveTo(springState.target.x - 10, springState.target.y);
  springCtx.lineTo(springState.target.x + 10, springState.target.y);
  springCtx.moveTo(springState.target.x, springState.target.y - 10);
  springCtx.lineTo(springState.target.x, springState.target.y + 10);
  springCtx.stroke();

  springCtx.strokeStyle = "#0ea5a3";
  springCtx.lineWidth = 2;
  springCtx.beginPath();
  springCtx.moveTo(springState.target.x, springState.target.y);
  springCtx.lineTo(springState.pos.x, springState.pos.y);
  springCtx.stroke();

  springCtx.fillStyle = "#f97316";
  springCtx.beginPath();
  springCtx.arc(springState.pos.x, springState.pos.y, 12, 0, Math.PI * 2);
  springCtx.fill();

  drawSparkline(spValueGraph, springState.historyPos, "#0ea5a3", "displacement");
  drawSparkline(spVelGraph, springState.historyVel, "#f97316", "velocity");
  spRest.textContent = `at_rest: ${springState.atRest}`;
};

// Tab 4: Timeline editor
const timelineEditor = document.getElementById("timeline-editor");
const tlLoop = document.getElementById("tl-loop");
const tlPlay = document.getElementById("tl-play");
const tlPause = document.getElementById("tl-pause");
const tlReset = document.getElementById("tl-reset");

const timelineState = {
  duration: 240,
  elapsed: 0,
  playing: false,
  loop: "once",
  drag: null,
  tracks: [
    { name: "Square", className: "square", blocks: [{ id: 1, start: 0, duration: 70, easing: "easeOutCubic" }] },
    { name: "Circle", className: "circle", blocks: [{ id: 2, start: 30, duration: 80, easing: "easeInOutQuad" }] },
    { name: "Triangle", className: "triangle", blocks: [{ id: 3, start: 65, duration: 90, easing: "easeOutElastic" }] },
  ],
};
let nextBlockId = 4;
const DEFAULT_TIMELINE_BLOCK_DURATION = 50;

const timelinePreview = document.createElement("div");
timelinePreview.className = "stage";
timelinePreview.innerHTML =
  '<div class="track"></div><div class="box" id="tl-square"></div><div class="box" id="tl-circle"></div><div class="box" id="tl-triangle"></div>';
document.querySelector("#tab-timeline .card:last-child").append(timelinePreview);
const tlSquare = document.getElementById("tl-square");
const tlCircle = document.getElementById("tl-circle");
const tlTriangle = document.getElementById("tl-triangle");
tlCircle.style.background = "linear-gradient(145deg, #fbbf24, #fb923c)";
tlCircle.style.borderRadius = "50%";
tlTriangle.style.background = "linear-gradient(145deg, #f97316, #fdba74)";
tlTriangle.style.clipPath = "polygon(50% 0%, 100% 100%, 0% 100%)";

const clampTimelineBlockStart = (start, duration) =>
  clamp(start, 0, Math.max(0, timelineState.duration - duration));

const renderTimeline = () => {
  timelineEditor.innerHTML = "";
  timelineState.tracks.forEach((trackData, trackIndex) => {
    const track = document.createElement("div");
    track.className = "timeline-track";
    track.dataset.track = String(trackIndex);
    track.innerHTML = `<div class="timeline-label">${trackData.name}</div><div class="playhead"></div>`;

    const playhead = track.querySelector(".playhead");
    playhead.style.left = `${(timelineState.elapsed / timelineState.duration) * 100}%`;

    for (const block of trackData.blocks) {
      const blockEl = document.createElement("div");
      blockEl.className = `timeline-block ${trackData.className}`;
      blockEl.dataset.track = String(trackIndex);
      blockEl.dataset.id = String(block.id);
      blockEl.textContent = block.easing;
      blockEl.style.left = `${(block.start / timelineState.duration) * 100}%`;
      blockEl.style.width = `${(block.duration / timelineState.duration) * 100}%`;

      blockEl.addEventListener("pointerdown", (ev) => {
        timelineState.drag = {
          trackIndex,
          blockId: block.id,
          startX: ev.clientX,
          originalStart: block.start,
        };
        blockEl.setPointerCapture(ev.pointerId);
      });

      blockEl.addEventListener("contextmenu", (ev) => {
        ev.preventDefault();
        trackData.blocks = trackData.blocks.filter((b) => b.id !== block.id);
        renderTimeline();
      });

      track.append(blockEl);
    }

    track.addEventListener("click", (ev) => {
      if (ev.target !== track) return;
      const rect = track.getBoundingClientRect();
      const t = clamp((ev.clientX - rect.left) / rect.width, 0, 1);
      const duration = DEFAULT_TIMELINE_BLOCK_DURATION;
      trackData.blocks.push({
        id: nextBlockId++,
        start: clampTimelineBlockStart(Math.floor(t * timelineState.duration), duration),
        duration,
        easing: easingNames[(trackData.blocks.length + trackIndex * 3) % easingNames.length],
      });
      renderTimeline();
    });

    timelineEditor.append(track);
  });
};

window.addEventListener("pointermove", (ev) => {
  if (!timelineState.drag) return;
  const { trackIndex, blockId, startX, originalStart } = timelineState.drag;
  const trackEl = timelineEditor.children[trackIndex];
  if (!trackEl) return;
  const rect = trackEl.getBoundingClientRect();
  const deltaTicks = Math.round(((ev.clientX - startX) / rect.width) * timelineState.duration);
  const trackData = timelineState.tracks[trackIndex];
  const block = trackData.blocks.find((b) => b.id === blockId);
  if (!block) return;
  block.start = clampTimelineBlockStart(originalStart + deltaTicks, block.duration);
  renderTimeline();
});
window.addEventListener("pointerup", () => {
  timelineState.drag = null;
});

const timelineObjectPos = (trackIndex) => {
  const track = timelineState.tracks[trackIndex];
  const base = [0.15, 0.45, 0.75][trackIndex];
  let current = base;
  for (const block of track.blocks) {
    const end = block.start + block.duration;
    if (timelineState.elapsed >= block.start && timelineState.elapsed <= end) {
      const p = clamp((timelineState.elapsed - block.start) / Math.max(1, block.duration), 0, 1);
      current = easingFns[block.easing](p);
    }
  }
  return clamp(current, 0, 1);
};

const renderTimelinePreview = () => {
  const width = timelinePreview.clientWidth;
  const minX = 14;
  const maxX = width - 14 - tlSquare.clientWidth;
  tlSquare.style.left = `${lerp(minX, maxX, timelineObjectPos(0))}px`;
  tlCircle.style.left = `${lerp(minX, maxX, timelineObjectPos(1))}px`;
  tlTriangle.style.left = `${lerp(minX, maxX, timelineObjectPos(2))}px`;
  tlSquare.style.top = "30px";
  tlCircle.style.top = "82px";
  tlTriangle.style.top = "134px";
  tlSquare.style.position = tlCircle.style.position = tlTriangle.style.position = "absolute";
};

const syncTimelineLoopMode = () => {
  timelineState.loop = tlLoop.value;
};
tlLoop.addEventListener("change", syncTimelineLoopMode);
tlLoop.addEventListener("input", syncTimelineLoopMode);
tlPlay.addEventListener("click", () => {
  if (timelineState.elapsed >= timelineState.duration) {
    timelineState.elapsed = 0;
    renderTimeline();
    renderTimelinePreview();
  }
  timelineState.playing = true;
});
tlPause.addEventListener("click", () => {
  timelineState.playing = false;
});
tlReset.addEventListener("click", () => {
  timelineState.playing = false;
  timelineState.elapsed = 0;
  renderTimeline();
  renderTimelinePreview();
});

renderTimeline();
renderTimelinePreview();

// Frame loop
let lastTs = performance.now();
let tweenAccumulator = 0;
let springAccumulator = 0;
let timelineAccumulator = 0;
let wasmAccumulator = 0;

const frame = (ts) => {
  const dt = ts - lastTs;
  lastTs = ts;

  renderEasingPreview(ts);

  tweenAccumulator += dt;
  while (tweenAccumulator >= 1000 / 60) {
    tickTween();
    tweenAccumulator -= 1000 / 60;
  }
  renderTween();

  springAccumulator += dt;
  while (springAccumulator >= 1000 / 60) {
    if (!springState.atRest) stepSpring();
    springAccumulator -= 1000 / 60;
  }
  renderSpring();

  timelineAccumulator += dt;
  while (timelineAccumulator >= 1000 / 60) {
    if (timelineState.playing) {
      timelineState.elapsed += 1;
      if (timelineState.elapsed > timelineState.duration) {
        if (timelineState.loop === "infinite") timelineState.elapsed = 0;
        else {
          timelineState.elapsed = timelineState.duration;
          timelineState.playing = false;
        }
      }
      renderTimeline();
    }
    timelineAccumulator -= 1000 / 60;
  }
  renderTimelinePreview();

  wasmAccumulator += dt;
  while (wasmAccumulator >= 1000 / 60) {
    if (wasmReady && wasmDemo) wasmDemo.tick();
    wasmAccumulator -= 1000 / 60;
  }
  renderWasmSnapshot();

  requestAnimationFrame(frame);
};

requestAnimationFrame(frame);
