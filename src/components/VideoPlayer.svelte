<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { invoke } from "@tauri-apps/api/core";

  export let src: string | undefined = "D:/Videos/Sinderella/105-min-1-hi.mp4";

  let videoContainer: HTMLElement;
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
  <div
    bind:this={videoContainer}
    id="video-container"
    class="video-container"
  ></div>

  <div class="controls">
    <button on:click={togglePlayback} class="control-btn">
      {isPlaying ? "⏸️" : "▶️"}
    </button>
  </div>
</div>

<style>
  .player-container {
    position: relative;
    width: 100%;
    height: 100%;
  }

  .video-container {
    width: 100%;
    height: 100%;
    background: transparent;
  }

  .controls {
    position: absolute;
    bottom: 0;
    left: 0;
    right: 0;
    padding: 1rem;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    justify-content: center;
  }

  .control-btn {
    background: transparent;
    border: none;
    color: white;
    cursor: pointer;
    padding: 0.5rem;
    font-size: 1.5rem;
  }
</style>
