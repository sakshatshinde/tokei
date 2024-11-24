<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  async function createAnilistWindow() {
    try {
      await invoke("create_anilist_window");
    } catch (error) {
      console.error("Failed to create tokie - anilist window:", error);
    }
  }

  onMount(async () => {
    await createAnilistWindow();
  });
</script>

<div class="hero bg-base-200 min-h-screen">
  <div class="hero-content text-center">
    <div class="max-w max-h">
      <h1 class="text-5xl font-bold">Anilist Launched</h1>
      <p class="py-6">Please use the newly launched Anilist window</p>
    </div>
  </div>
</div>
