<script>
    import { onMount } from "svelte";

    let midi_keys = {
        C: false,
        Cs: false,
        D: false,
        Ds: false,
        E: false,
        F: false,
        Fs: false,
        G: false,
        Gs: false,
        A: false,
        As: false,
        B: false,
    };

    let socket;
    onMount(() => {
        socket = new WebSocket("ws://localhost:9001/");
        socket.addEventListener("open", () => {
            console.log("Websocket opened");
        });

        socket.onmessage = function (e) {
            let payload = JSON.parse(e.data);
            if (typeof e.data === "string") {
                let message_key = payload["key"];
                midi_keys[message_key] = !midi_keys[message_key];
            }
        };
    });
</script>

<main>
    <h1>This is the circle-of-fifths component!</h1>
    <div>
        {#each Object.entries(midi_keys) as [midi_key, pressed]}
            <p>{midi_key}, {pressed}</p>
        {/each}
    </div>
</main>

<style>
</style>
