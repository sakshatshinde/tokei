<script lang="ts">
  // import { invoke } from "@tauri-apps/api/core";
  // import { onMount } from "svelte";

  // async function createSubspleaseWindow() {
  //   try {
  //     await invoke("create_subsplease_window");
  //   } catch (error) {
  //     console.error("Failed to create tokie - subsplease window:", error);
  //   }
  // }

  // onMount(async () => {
  //   await createSubspleaseWindow();
  // });
</script>

<!-- Iframe goes here -->
<iframe
  id="subsplease-iframe"
  title="Subsplease"
  src="https://subsplease.org"
  class="h-full w-full border-none"
></iframe>

<!-- <div class="hero h-full">
  <div class="hero-content text-center">
    <div class="max-w-2xl px-6">
      <h1 class="text-6xl font-bold mb-6 tracking-tight">
        Subsplease Launched
      </h1>
      <div class="backdrop-blur-sm p-4 mb-6 border">
        <h2 class="text-2xl font-medium tracking-wide">
          One stop shop for new anime
        </h2>
      </div>
      <p class="text-lg leading-relaxed">
        Please use the newly launched Subsplease window
      </p>
    </div>
  </div>
</div> -->
