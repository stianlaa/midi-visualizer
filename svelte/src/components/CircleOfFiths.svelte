<script>
    import { w3cwebsocket as W3CWebSocket } from "websocket";

    const firstKeyIndex = 36; // Smaller keyboards start at a higher octave
    const keyboardKeyNum = 60;
    const octaves = keyboardKeyNum/12;

    const initializeKeys = function (startIndex, numKeys) {
        let k = {};
        for (let i = startIndex; i < startIndex+numKeys; i++) {
            k[i] = { pressed: false, timestamp: 0, index: i};
        }
        return k;
    };
    const keyList = ["C", "Cs", "D", "Ds", "E", "F", "Fs", "G", "Gs", "A", "As", "B"];
    let midiKeys = initializeKeys(firstKeyIndex, keyboardKeyNum);

    // Handling websocket midi data
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
            midiKeys[firstKeyIndex + parseInt(payload.index)] = payload;
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

    const height = 650;
    const width = height;
    const outerRadius = width / 2;
    const innerRadius = width / 4;
    const depth = (outerRadius - innerRadius)/octaves;

    const segmentColors = [
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

    // define segments equally spaced
    $: segmentInfoList = [...Array(keyboardKeyNum).keys()].map((k) => {
        const i = k + firstKeyIndex;
        const ci = (i * 5) % 12; // The circle of fifths changes 5 half-steps between segments
        const radius = innerRadius + ((outerRadius - innerRadius) * k) / keyboardKeyNum;

        const rotationOffset = (6 * pi) / 4;
        const segmentWidth = (2 * pi) / 12;
        const segmentAngle = rotationOffset - i * segmentWidth - segmentWidth / 2;

        // Text
        const textRad = 0.85 * radius;
        const x = width / 2 + textRad * cos(segmentAngle + segmentWidth / 2);
        const y = height / 2 + textRad * sin(segmentAngle + segmentWidth / 2);
        let segmentInfo = {
            path: createSvgArc(
                radius - depth / 2, // radius, adjusted for depth
                width / 2, // center x
                height / 2, // center y
                0, // start angle, in radian.
                segmentWidth * 0.95, // angle to sweep, in radian. positive./
                segmentAngle // rotation on the whole, in radian
            ),
            color: midiKeys[i].pressed
                ? segmentColors[i%12]
                : "grey"
        };

        if (i >= 36 & i < 48) {
            segmentInfo["textProps"] = {
                x: x,
                y: y,
                "transform-origin": `${x} ${y}`,
                "text-anchor": "middle",
                "font-size": "1.5rem",
                "z-index": 5,
                "font-family": "'Brush Script MT', cursive",
            };
            segmentInfo["key"] = keyList[ci];
        };
        return segmentInfo;
    });
</script>

<main>
    <h1>The Spiral of fifths!</h1>

    <div>
        <svg style="stroke-width:{depth};" {width} {height}>
            {#each segmentInfoList as segmentInfo}
                <path d={segmentInfo.path} stroke={segmentInfo.color} />
            {/each}
            {#each segmentInfoList as segmentInfo}
                <text {...segmentInfo.textProps}>{segmentInfo.key}</text>
            {/each}
        </svg>
        <div>{JSON.stringify(midiKeys)}</div>
    </div>
</main>

<style>
</style>
