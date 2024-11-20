<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { invoke } from "@tauri-apps/api/core";

  export let src: string | undefined;

  async function initPlayer() {
    try {
      // Pass the container_id properly
      await invoke("init_player");
      if (src) {
        await playMedia(src);
      }
    } catch (error) {
      console.error("Failed to initialize MPV:", error);
    }
  }

  async function playMedia(path: string) {
    try {
      if (!path) {
        console.error("No valid file path provided.");
        return;
      }
      console.log("Attempting to play media from:", path);
      await invoke("play_media", { path });
    } catch (error) {
      console.error("Failed to play media:", error);
    }
  }

  onMount(async () => {
    await initPlayer();
  });

  onDestroy(() => {
    // Cleanup if needed
  });

  // Watch for src changes
  $: if (src) {
    playMedia(src);
  }
</script>
