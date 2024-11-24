<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  async function createSubspleaseWindow() {
    try {
      await invoke("create_subsplease_window");
    } catch (error) {
      console.error("Failed to create tokie - subsplease window:", error);
    }
  }

  onMount(async () => {
    await createSubspleaseWindow();
  });
</script>

<!-- <div class="hero bg-base-200 min-h-screen">
  <div class="hero-content text-center">
    <div class="max-w max-h">
      <h1 class="text-5xl font-semibold">Subsplease Launched</h1>
      <div class="bg-base-600">
        <h1 class="text-xl mt-2">One stop shop for new anime</h1>
      </div>
      <p class="py-6">Please use the newly launched Subsplease window</p>
    </div>
  </div>
</div> -->

<div class="hero min-h-screen bg-black">
  <div class="hero-content text-center">
    <div class="max-w-2xl px-6">
      <h1 class="text-6xl font-bold mb-6 text-white tracking-tight">
        Subsplease Launched
      </h1>
      <div
        class="bg-white/10 backdrop-blur-sm rounded-lg p-4 mb-6 border border-white/20"
      >
        <h2 class="text-2xl font-medium text-gray-300 tracking-wide">
          One stop shop for new anime
        </h2>
      </div>
      <p class="text-lg leading-relaxed">
        Please use the newly launched Subsplease window
      </p>
    </div>
  </div>
</div>

<style>
</style>
