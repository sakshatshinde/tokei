<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { invoke } from "@tauri-apps/api/core";

  export let src: string | undefined = "D:/Downloads/bun33s.mp4";

  // let videoContainer: HTMLElement;
  let isPlaying = false;

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
      await invoke("play_media", { path });
      isPlaying = true;
    } catch (error) {
      console.error("Failed to play media:", error);
    }
  }

  async function togglePlayback() {
    try {
      await invoke("toggle_pause");
      isPlaying = !isPlaying;
    } catch (error) {
      console.error("Failed to toggle playback:", error);
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

<div class="player-container">
  <button on:click={togglePlayback} class="btn btn-outline btn-sm">
    {isPlaying ? "Pause" : "Play"}
  </button>
</div>
