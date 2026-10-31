// The figure at the top of the site: Chua's circuit of 1983, drawn by voltik beside its oscilloscope.
// The page integrates the circuit and writes the CircuiTikZ; voltik does the drawing, again on every
// move. Turning R down takes the trace from a still point through a loop to the double scroll.
var voltikChua = (() => {
  // the values of Kennedy's build of the circuit: farads, henries, siemens, volts
  const C1 = 10e-9, C2 = 100e-9, L = 18e-3, GA = -0.757e-3, GB = -0.409e-3, E = 1.08;
  const DT = 4e-6, SETTLE = 10000, KEEP = 12000, EVERY = 3; // steps left out, steps shown, one point in EVERY
  const HIGH = 2100, LOW = 1600, START = 1850; // ohms: the model runs away below about 1550
  const REACH = [7.2, 1.4, 6.2e-3]; // the largest v1, v2 and iL, to scale them to the screen
  const VIEW = [0.45, 0.25]; // azimuth and elevation of the screen, in radians

  // the current through the nonlinear resistor
  const diode = (v) => GB * v + 0.5 * (GA - GB) * (Math.abs(v + E) - Math.abs(v - E));
  const flow = ([v1, v2, i], R) => [((v2 - v1) / R - diode(v1)) / C1, ((v1 - v2) / R + i) / C2, -v2 / L];

  // the trace once it has settled, by fourth-order Runge-Kutta, as [v1, v2, iL]
  function integrate(R) {
    const points = [];
    let p = [0.1, 0, 0];
    const along = (a, k, h) => a.map((v, i) => v + k[i] * h);
    for (let n = 0; n < SETTLE + KEEP; n++) {
      const k1 = flow(p, R), k2 = flow(along(p, k1, DT / 2), R), k3 = flow(along(p, k2, DT / 2), R), k4 = flow(along(p, k3, DT), R);
      p = p.map((v, i) => v + (DT / 6) * (k1[i] + 2 * k2[i] + 2 * k3[i] + k4[i]));
      if (n >= SETTLE && n % EVERY === 0) points.push(p);
    }
    return points;
  }

  // what the trace is doing, read from it: how far v1 moves, and whether it stays on one side
  function regime(points) {
    const v = points.map((p) => p[0]), low = Math.min(...v), high = Math.max(...v);
    if (high - low < 0.01) return "still point";
    if (low < 0 && high > 0 && Math.min(high, -low) > 1) return "double scroll";
    const peaks = new Set();
    for (let i = 1; i < v.length - 1; i++) if (v[i] > v[i - 1] && v[i] > v[i + 1]) peaks.add(v[i].toFixed(1));
    return peaks.size > 4 ? "single scroll" : "loop";
  }

  // the circuit, with R as it is now
  const schematic = (R) => String.raw`\begin{circuitikz}
  \definecolor{silk}{HTML}{F4F3E8}
  \begin{scope}[silk, text=silk, line width=0.7pt]
    \draw (0,3) to[short, -*] (2,3) to[R=${(R / 1000).toFixed(2)}\,k$\Omega$, -*] (5,3) to[short] (7,3)
      to[generic, l_=$N_R$] (7,0) to[short, -*] (5,0) to[short, -*] (2,0) to[short] (0,0) to[L=$L$] (0,3);
    \draw (2,3) to[C=$C_2$] (2,0);
    \draw (5,3) to[C=$C_1$] (5,0);
  \end{scope}
\end{circuitikz}
`;

  // the trace on the screen, seen from azimuth `az` and elevation `el`
  function scope(points, az, el) {
    const place = ([v1, v2, i]) => {
      const [x, y, z] = [v1 / REACH[0], v2 / REACH[1], i / REACH[2]];
      const across = x * Math.cos(az) - z * Math.sin(az), deep = x * Math.sin(az) + z * Math.cos(az);
      return `(${(across * 3.7).toFixed(2)},${((y * Math.cos(el) + deep * Math.sin(el)) * 2.7).toFixed(2)})`;
    };
    return String.raw`\begin{circuitikz}
  \definecolor{gold}{HTML}{E0B04A}
  \definecolor{grid}{HTML}{1F6B53}
  \path[use as bounding box] (-4,-3) rectangle (4,3);
  \draw[grid, line width=0.3pt, step=1] (-4,-3) grid (4,3);
  \draw[grid, line width=0.7pt] (-4,0) -- (4,0) (0,-3) -- (0,3);
  \clip (-4,-3) rectangle (4,3);
  \draw[gold, line width=0.5pt, line join=round] plot coordinates {${points.map(place).join(" ")}};
\end{circuitikz}
`;
  }

  // in `box`, with `render(code)` → {svg}: the slider turns R, a drag turns the screen
  function mount(box, render) {
    box.innerHTML = `<div class="bench">
        <div class="bench-panel"><div class="bench-figure" id="chua-circuit"></div></div>
        <div class="bench-panel"><div class="bench-figure bench-screen" id="chua-scope" tabindex="0" role="img" aria-label="The oscilloscope of Chua's circuit. Drag, or use the arrow keys, to turn it."></div></div>
      </div>
      <div class="bench-controls">
        <label class="param"><span>R</span><input id="chua-r" type="range" min="${LOW}" max="${HIGH}" step="5" value="${START}"><output></output></label>
        <span class="bench-regime" id="chua-regime"></span>
        <button class="button" type="button" id="chua-play">Turn R down again</button>
      </div>`;
    const [circuit, screen, slider, name] = ["circuit", "scope", "r", "regime"].map((id) => box.querySelector("#chua-" + id));
    let [az, el] = VIEW, points = [], frame = 0, wanted = false;

    const trace = () => { screen.innerHTML = render(scope(points, az, el)).svg; };
    function setR() {
      const R = Number(slider.value);
      points = integrate(R);
      circuit.innerHTML = render(schematic(R)).svg;
      slider.nextElementSibling.value = (R / 1000).toFixed(3) + " kΩ";
      name.textContent = regime(points);
      trace();
    }
    // one drawing per screen refresh, however many moves come in
    const ask = (draw) => {
      if (wanted) return;
      wanted = true;
      requestAnimationFrame(() => { wanted = false; draw(); });
    };

    const stop = () => cancelAnimationFrame(frame);
    function play() {
      stop();
      if (matchMedia("(prefers-reduced-motion: reduce)").matches) { slider.value = START; setR(); return; }
      const began = performance.now(), seconds = 7;
      const tick = (now) => {
        const done = Math.min(1, (now - began) / 1000 / seconds);
        slider.value = Math.round((HIGH + (START - HIGH) * done) / 5) * 5;
        setR();
        if (done < 1) frame = requestAnimationFrame(tick);
      };
      frame = requestAnimationFrame(tick);
    }

    slider.oninput = () => { stop(); ask(setR); };
    box.querySelector("#chua-play").onclick = play;

    let held = null;
    screen.addEventListener("pointerdown", (e) => { held = [e.clientX, e.clientY]; screen.setPointerCapture(e.pointerId); });
    screen.addEventListener("pointermove", (e) => {
      if (!held) return;
      az -= (e.clientX - held[0]) * 0.01;
      el = Math.max(-1.57, Math.min(1.57, el + (e.clientY - held[1]) * 0.01));
      held = [e.clientX, e.clientY];
      ask(trace);
    });
    for (const end of ["pointerup", "pointercancel"]) screen.addEventListener(end, () => { held = null; });
    screen.addEventListener("keydown", (e) => {
      const turn = { ArrowLeft: 0.1, ArrowRight: -0.1 }[e.key], tilt = { ArrowUp: -0.1, ArrowDown: 0.1 }[e.key];
      if (turn === undefined && tilt === undefined) return;
      e.preventDefault();
      az += turn ?? 0;
      el = Math.max(-1.57, Math.min(1.57, el + (tilt ?? 0)));
      ask(trace);
    });

    setR();
    play();
  }

  return { mount, integrate, regime, schematic, scope, VIEW };
})();
if (typeof module !== "undefined") module.exports = voltikChua;
