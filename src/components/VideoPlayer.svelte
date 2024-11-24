<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { invoke } from "@tauri-apps/api/core";

  export let src: string | undefined;

  async function initPlayer(path: string) {
    try {
      // Pass the container_id properly
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

  onMount(async () => {
    if (src) {
      await initPlayer(src);
    }
    await watchPlayerShutdown();
  });

  onDestroy(() => {
    // Cleanup if needed
  });
</script>
