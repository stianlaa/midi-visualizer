<script>
    import { onMount } from "svelte";
    // import { webSocketClient } from "websocket";

    let socket;
    onMount(() => {
        socket = new WebSocket("ws://localhost:9001/");
        socket.addEventListener("open", () => {
            console.log("Websocket opened");
        });

        socket.onmessage = function (e) {
            if (typeof e.data === "string") {
                console.log("Received: '" + e.data + "'");
            }
        };
    });
</script>

<main>
    <h1>This is the circle-of-fifths component!</h1>
</main>

<style>
</style>
