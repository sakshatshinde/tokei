<script lang="ts">
  import {
    Play,
    Folder,
    FileVideo,
    ChevronDown,
    ChevronRight,
  } from "lucide-svelte";
  import { onMount } from "svelte";
  import { fetchMediaDirectoryStructure, webviewsToHide } from "$lib/utils";

  // Type definition for directory structure
  interface VideoDirectory {
    name: string;
    path: string;
    files: string[];
    subdirectories: VideoDirectory[];
  }

  let directoryStructure: VideoDirectory[] = [];
  let allFiles: string[] = [];
  let expandedFolders: Set<string> = new Set();

  // Fetch directory structure on mount
  onMount(async () => {
    try {
      // Hide all other webviews except the main one
      await webviewsToHide("all");

      // Fetch directory structure
      const fetchedStructure = await fetchMediaDirectoryStructure();
      console.log("Fetched Directory Structure:", fetchedStructure);

      // Set directories directly
      directoryStructure = fetchedStructure.subdirectories || [];

      // Collect all files from subdirectories
      allFiles = collectAllFiles(fetchedStructure);
    } catch (error) {
      console.error("Error during onMount:", error);
    }
  });

  // Recursive function to collect all files
  function collectAllFiles(directory: VideoDirectory): string[] {
    let files: string[] = [...(directory.files || [])];

    // Recursively collect files from subdirectories
    if (directory.subdirectories) {
      directory.subdirectories.forEach((subdir) => {
        files = files.concat(collectAllFiles(subdir));
      });
    }

    return files;
  }

  // Toggle folder expansion
  function toggleFolder(folderName: string) {
    if (expandedFolders.has(folderName)) {
      expandedFolders.delete(folderName);
    } else {
      expandedFolders.add(folderName);
    }
    expandedFolders = expandedFolders;
  }

  // File playback function
  function playFile(filePath: string) {
    console.log(`Attempting to play: ${filePath}`);
    // Add your actual file playing logic here
  }

  // Function to truncate file names
  function truncateFileName(fileName: string, maxLength: number = 50): string {
    if (fileName.length <= maxLength) return fileName;
    return `${fileName.slice(0, maxLength)}...${fileName.split(".").pop()}`;
  }
</script>

<div class="container mx-auto px-4 py-6 space-y-4">
  {#if allFiles.length > 0}
    <div class="card bg-base-100 shadow-xl">
      <div class="card-body p-4">
        <h2 class="card-title flex items-center gap-2 text-primary">
          <FileVideo class="w-6 h-6" />
          All Videos
          <span class="badge badge-primary badge-sm ml-2"
            >{allFiles.length}</span
          >
        </h2>
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3">
          {#each allFiles as file}
            <div
              class="flex items-center justify-between bg-base-200 p-3 rounded-lg hover:bg-base-300 transition-colors"
            >
              <span class="text-sm font-medium truncate max-w-[200px]">
                {truncateFileName(file)}
              </span>
              <button
                class="btn btn-circle btn-primary btn-sm"
                on:click={() => playFile(file)}
                title="Play video"
              >
                <Play size={16} />
              </button>
            </div>
          {/each}
        </div>
      </div>
    </div>
  {/if}

  {#each directoryStructure as directory}
    {#if directory.files.length > 0 || directory.subdirectories.length > 0}
      <div class="card bg-base-100 shadow-xl">
        <div class="card-body p-4">
          <button
            class="card-title flex items-center gap-2 text-secondary w-full text-left"
            on:click={() => toggleFolder(directory.name)}
          >
            {#if expandedFolders.has(directory.name)}
              <ChevronDown class="w-6 h-6" />
            {:else}
              <ChevronRight class="w-6 h-6" />
            {/if}
            <Folder class="w-6 h-6" />
            {directory.name}
            <span class="badge badge-secondary badge-sm ml-2">
              {directory.files.length + directory.subdirectories.length}
            </span>
          </button>

          {#if expandedFolders.has(directory.name)}
            <div
              class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3 mt-3"
            >
              {#each directory.files as file}
                <div
                  class="flex items-center justify-between bg-base-200 p-3 rounded-lg hover:bg-base-300 transition-colors"
                >
                  <span class="text-sm font-medium truncate max-w-[200px]">
                    {truncateFileName(file)}
                  </span>
                  <button
                    class="btn btn-circle btn-secondary btn-sm"
                    on:click={() => playFile(file)}
                    title="Play video"
                  >
                    <Play size={16} />
                  </button>
                </div>
              {/each}
            </div>
          {/if}
        </div>
      </div>
    {/if}
  {/each}
</div>

<style>
  ::-webkit-scrollbar {
    width: 8px;
  }
  ::-webkit-scrollbar-track {
    background: hsl(var(--b2, var(--b1)));
  }
  ::-webkit-scrollbar-thumb {
    background: hsl(var(--p, primary));
    border-radius: 4px;
  }
</style>
