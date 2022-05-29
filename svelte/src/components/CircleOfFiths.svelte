<script>
    import { w3cwebsocket as W3CWebSocket } from "websocket";

    // Handling websocket midi data
    let midi_keys = {
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
            let message_key = payload["key"];
            midi_keys[message_key] = payload;
        }
    };

    // Drawing circle segments
    const cos = Math.cos;
    const sin = Math.sin;
    const pi = Math.PI;

    const f_matrix_times = ([[a, b], [c, d]], [x, y]) => [
        a * x + b * y,
        c * x + d * y,
    ];
    const f_rotate_matrix = (x) => [
        [cos(x), -sin(x)],
        [sin(x), cos(x)],
    ];
    const f_vec_add = ([a1, a2], [b1, b2]) => [a1 + b1, a2 + b2];

    const create_svg_arc = function (
        radius,
        centerx,
        centery,
        start_angle,
        sweep,
        rotation
    ) {
        sweep = sweep % (2 * pi);
        const rotMatrix = f_rotate_matrix(rotation);
        const [sX, sY] = f_vec_add(
            f_matrix_times(rotMatrix, [
                radius * cos(start_angle),
                radius * sin(start_angle),
            ]),
            [centerx, centery]
        );
        const [eX, eY] = f_vec_add(
            f_matrix_times(rotMatrix, [
                radius * cos(start_angle + sweep),
                radius * sin(start_angle + sweep),
            ]),
            [centerx, centery]
        );

        const fA = sweep > pi ? 1 : 0;
        const fS = sweep > 0 ? 1 : 0;
        return [
            "M",
            sX,
            sY,
            "A",
            radius,
            radius,
            (rotation / (2 * pi)) * 360,
            fA,
            fS,
            eX,
            eY,
        ].join(" ");
    };

    const width = 500;
    const height = 500;
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
    let segment_info = [];
    for (let i = 0; i < 12; i++) {
        let offset_rotation = (6 * pi) / 4;
        let segment_width = (2 * pi) / 12;
        let segment_angle =
            offset_rotation + i * segment_width - segment_width / 2;

        // Text
        let x =
            width / 2 - 0.85 * radius * cos(segment_angle + segment_width / 2);
        let y =
            height / 2 + 0.85 * radius * sin(segment_angle + segment_width / 2);

        segment_info.push({
            path: create_svg_arc(
                radius - depth / 2, // radius, adjusted for depth
                width / 2, // center x
                height / 2, // center y
                0, // start angle, in radian.
                segment_width * 0.95, // angle to sweep, in radian. positive./
                segment_angle // rotation on the whole, in radian
            ),
            color: segment_colors[i],
            text_props: {
                x: x,
                y: y,
                "transform-origin": `${x} ${y}`,
                "text-anchor": "middle",
                "font-size": "1.5rem",
                "z-index": 5,
            },
            text: Object.keys(midi_keys)[(i * 5) % 12],
        });
    }

    /*
        <div>
        {#each Object.entries(midi_keys) as [key, midi_data]}
            <p>{key}, {midi_data.pressed}, {midi_data.timestamp}</p>
        {/each}
        </div>

    */
</script>

<main>
    <h1>The Circle of fifths!</h1>

    <div>
        <svg style="stroke-width:{depth};" {width} {height}>
            {#each segment_info as segment_info}
                <path d={segment_info.path} stroke={segment_info.color} />
            {/each}
            {#each segment_info as segment_info}
                <text {...segment_info.text_props}>{segment_info.text}</text>
            {/each}
        </svg>
    </div>
</main>

<style>
</style>
