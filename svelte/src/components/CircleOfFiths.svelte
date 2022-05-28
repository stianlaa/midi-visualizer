<script>
    import { w3cwebsocket as W3CWebSocket } from "websocket";

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
</script>

<main>
    <h1>This is the circle-of-fifths component!</h1>
    <div>
        {#each Object.entries(midi_keys) as [key, midi_data]}
            <p>{key}, {midi_data.pressed}, {midi_data.timestamp}</p>
        {/each}
    </div>
</main>

<style>
</style>
