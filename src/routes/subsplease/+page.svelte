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

<div class="hero bg-base-200 min-h-screen">
  <div class="hero-content text-center">
    <div class="max-w max-h">
      <h1 class="text-5xl font-bold">Subsplease Launched</h1>
      <p class="py-6">Please use the newly launched Subsplease window</p>
    </div>
  </div>
</div>
