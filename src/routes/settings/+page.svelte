<!-- FolderPicker.svelte -->
<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  export let onSelect: ((path: string) => void) | undefined = undefined;

  let selectedPath: string;
  let error: string;
  let isSelecting = false;

  onMount(() => {
    const savedAnimePath = localStorage.getItem("savedAnimePath");
    if (savedAnimePath) {
      selectedPath = savedAnimePath;
    } else {
      selectedPath = "";
    }
  });

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

<h1 class="text-2xl tracking-wide subpixel-antialiased font-bold mb-4">
  Settings
</h1>

<div class="form-control w-full max-w-sm">
  <!-- Label -->
  <div class="label flex justify-between items-center">
    <span class="label-text font-medium"
      >Please select a root folder for your anime</span
    >
  </div>

  <!-- Input Group with Unified Outline -->
  <div
    class="flex items-center border dark:border-gray-600 overflow-hidden shadow-sm"
  >
    <!-- Label or Selected Path -->
    <div class="flex-1 px-3 font-bold">
      {#if selectedPath}
        {selectedPath}
      {:else}
        None Selected
      {/if}
    </div>

    <!-- Button -->
    <button class="btn btn-primary rounded-none" on:click={handlePickFolder}>
      {#if isSelecting}
        <span class="font-bold loading loading-spinner loading-sm mr-2"></span>
        Selecting...
      {:else}
        📁
      {/if}
    </button>
  </div>

  <!-- Error Message -->
  {#if error}
    <span class="label-text-alt text-error text-sm">{error}</span>
  {/if}
</div>
