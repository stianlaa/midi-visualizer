<script>
    import { w3cwebsocket as W3CWebSocket } from "websocket";

    // Handling websocket midi data
    let midiKeys = {
        C: { pressed: false, timestamp: 0 },
        Cs: { pressed: false, timestamp: 0 },
        D: { pressed: false, timestamp: 0 },
        Ds: { pressed: false, timestamp: 0 },
        E: { pressed: false, timestamp: 0 },
        F: { pressed: false, timestamp: 0 },
        Fs: { pressed: false, timestamp: 0 },
        G: { pressed: false, timestamp: 0 },
        Gs: { pressed: false, timestamp: 0 },
        A: { pressed: false, timestamp: 0 },
        As: { pressed: false, timestamp: 0 },
        B: { pressed: false, timestamp: 0 },
    };

    var client = new W3CWebSocket("ws://localhost:9001/");

    client.onerror = function () {
        console.log("Connection Error");
    };

    client.onopen = function () {
        console.log("WebSocket Client Connected");
    };

    client.onclose = function () {
        console.log("Client Closed");
    };

    client.onmessage = function (e) {
        let payload = JSON.parse(e.data);
        if (typeof e.data === "string") {
            let messageKey = payload["key"];
            midiKeys[messageKey] = payload;
        }
    };

    // Drawing circle segments
    const cos = Math.cos;
    const sin = Math.sin;
    const pi = Math.PI;

    const multiplyMatrix = ([[a, b], [c, d]], [x, y]) => [
        a * x + b * y,
        c * x + d * y,
    ];
    const rotateMatrix = (x) => [
        [cos(x), -sin(x)],
        [sin(x), cos(x)],
    ];
    const addVector = ([a1, a2], [b1, b2]) => [a1 + b1, a2 + b2];

    const createSvgArc = function (r, cx, cy, start, sweep, rot) {
        sweep = sweep % (2 * pi);
        const rotMatrix = rotateMatrix(rot);
        const [sX, sY] = addVector(
            multiplyMatrix(rotMatrix, [r * cos(start), r * sin(start)]),
            [cx, cy]
        );
        const [eX, eY] = addVector(
            multiplyMatrix(rotMatrix, [
                r * cos(start + sweep),
                r * sin(start + sweep),
            ]),
            [cx, cy]
        );

        const fA = sweep > pi ? 1 : 0;
        const fS = sweep > 0 ? 1 : 0;
        const rotAng = (rot / (2 * pi)) * 360;
        return ["M", sX, sY, "A", r, r, rotAng, fA, fS, eX, eY].join(" ");
    };

    const height = 500;
    const width = height;
    const depth = width / 8;
    const radius = width / 2;

    const segment_colors = [
        "Aqua",
        "Aquamarine",
        "DarkCyan",
        "DarkGreen",
        "GoldenRod",
        "DarkOrange",
        "Tomato",
        "OrangeRed",
        "Maroon",
        "Violet",
        "DarkOrchid",
        "RoyalBlue",
    ];

    // 12 segments, equally spaced
    $: segmentInfoList = [...Array(12).keys()].map((i) => {
        let ci = (i * 5) % 12; // The circle of fifths changes 5 half-steps between segments

        const rotationOffset = (6 * pi) / 4;
        const segmentWidth = (2 * pi) / 12;
        let segmentAngle = rotationOffset - i * segmentWidth - segmentWidth / 2;

        // Text
        const textRad = 0.85 * radius;
        let x = width / 2 + textRad * cos(segmentAngle + segmentWidth / 2);
        let y = height / 2 + textRad * sin(segmentAngle + segmentWidth / 2);
        return {
            path: createSvgArc(
                radius - depth / 2, // radius, adjusted for depth
                width / 2, // center x
                height / 2, // center y
                0, // start angle, in radian.
                segmentWidth * 0.95, // angle to sweep, in radian. positive./
                segmentAngle // rotation on the whole, in radian
            ),
            color: Object.values(midiKeys)[ci].pressed
                ? segment_colors[i]
                : "grey",
            textProps: {
                x: x,
                y: y,
                "transform-origin": `${x} ${y}`,
                "text-anchor": "middle",
                "font-size": "1.5rem",
                "z-index": 5,
            },
            key: Object.keys(midiKeys)[ci],
            midiValue: Object.values(midiKeys)[ci],
        };
    });
</script>

<main>
    <h1>The Circle of fifths!</h1>

    <div>
        <svg style="stroke-width:{depth};" {width} {height}>
            {#each segmentInfoList as segmentInfo}
                <path d={segmentInfo.path} stroke={segmentInfo.color} />
            {/each}
            {#each segmentInfoList as segmentInfo}
                <text {...segmentInfo.textProps}>{segmentInfo.key}</text>
            {/each}
        </svg>
    </div>
</main>

<style>
</style>
