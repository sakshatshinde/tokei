<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { invoke } from "@tauri-apps/api/core";

  export let src: string | undefined;

  let currentSrc: string | undefined = src; // Track the current source

  async function initPlayer(path: string) {
    try {
      await invoke("init_player", { path });
    } catch (error) {
      console.error("Failed to initialize MPV:", error);
    }
  }

  async function watchPlayerShutdown() {
    try {
      await invoke("watch_player_shutdown");
    } catch (error) {
      console.error("Watching the player shutdown failed:", error);
    }
  }

  // Use the reactive statement in Svelte 5 to react to changes in `src`
  $: {
    if (src && src !== currentSrc) {
      currentSrc = src;
      initPlayer(src);
    }
  }

  onMount(async () => {
    await watchPlayerShutdown();
    if (src) {
      await initPlayer(src);
    }
  });

  onDestroy(async () => {
    await invoke("quit_player");
  });
</script>
