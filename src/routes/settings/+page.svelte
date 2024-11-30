<!-- FolderPicker.svelte -->
<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { webviewsToHide } from "../../lib/utils";

  export let onSelect: ((path: string) => void) | undefined = undefined;

  let selectedPath: string;
  let error: string;
  let errorhandleQuitPlayer: string;
  let isSelecting = false;

  onMount(async () => {
    await webviewsToHide("all"); // doesn't matter what you pass here it should hide all other webviews except main
    const savedAnimePath = localStorage.getItem("savedAnimePath");
    if (savedAnimePath) {
      selectedPath = savedAnimePath;
    } else {
      selectedPath = "";
    }
  });

  async function handleQuitPlayer() {
    try {
      await invoke<string>("quit_player");
    } catch (e) {
      errorhandleQuitPlayer =
        e instanceof Error ? e.message : "mpv doesn't want to quit :/";
      console.error("Error picking folder:", e);
    }
  }

  async function handlePickFolder() {
    isSelecting = true;
    error = "";

    try {
      const result = await invoke<string>("pick_directory");
      if (result) {
        selectedPath = result;
        localStorage.setItem("savedAnimePath", selectedPath);
        if (onSelect) onSelect(result);
      }
    } catch (e) {
      error = e instanceof Error ? e.message : "Failed to select folder";
      console.error("Error picking folder:", e);
    } finally {
      isSelecting = false;
    }
  }
</script>

<h1 class="text-2xl tracking-wide subpixel-antialiased font-semibold mb-4">
  Settings
</h1>

<h1 class="text-l tracking-wide subpixel-antialiased font-semibold mb-4">
  Misc
</h1>

<div class="form-control w-full max-w-sm">
  <!-- Label -->

  <span class="label-text mb-2">Please select a root folder for your anime</span
  >

  <!-- Input Group with Unified Outline -->
  <div
    class="flex items-center border dark:border-gray-600 overflow-hidden shadow-sm"
  >
    <!-- Label or Selected Path -->
    <div class="flex-1 px-3">
      {#if selectedPath}
        {selectedPath}
      {:else}
        None Selected
      {/if}
    </div>

    <!-- Button -->
    <button class="btn btn-outline btn-sm" on:click={handlePickFolder}>
      {#if isSelecting}
        <span class="font-bold loading loading-spinner loading-sm mr-2"></span>
        Selecting...
      {:else}
        📁 Choose a folder
      {/if}
    </button>
  </div>

  <!-- Error Message -->
  {#if error}
    <span class="label-text-alt text-error text-sm">{error}</span>
  {/if}
</div>

<!-- <div class="divider divider-accent"></div> -->

<div>
  <h1 class="text-l tracking-wide subpixel-antialiased font-semibold mb-4 mt-8">
    mpv
  </h1>
  <button class="btn btn-outline btn-sm" on:click={handleQuitPlayer}>
    Force close mpv</button
  >

  <!-- Error Message -->
  {#if errorhandleQuitPlayer}
    <span class="label-text-alt text-error font-bold text-sm p-2"
      >{errorhandleQuitPlayer}</span
    >
  {/if}
</div>
