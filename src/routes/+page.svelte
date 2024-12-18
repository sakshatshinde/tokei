<script lang="ts">
  import {
    Play,
    Folder,
    FileVideo,
    ChevronDown,
    ChevronRight,
  } from "lucide-svelte";
  import { onMount } from "svelte";
  import {
    extractFileName,
    fetchMediaDirectoryStructure,
    webviewsToHide,
  } from "$lib/utils";
  import VideoPlayer from "../components/VideoPlayer.svelte";
  interface VideoDirectory {
    name: string;
    path: string;
    files: string[];
    subdirectories: VideoDirectory[];
  }

  let directoryStructure: VideoDirectory[] = [];
  let allFiles: string[] = [];
  let expandedFolders: Set<string> = new Set();
  let currentVideo: string | null = null;

  onMount(async () => {
    try {
      await webviewsToHide("all");
      const fetchedStructure = await fetchMediaDirectoryStructure();
      directoryStructure = fetchedStructure.subdirectories || [];
      allFiles = collectAllFiles(fetchedStructure);
    } catch (error) {
      console.error("Error during onMount:", error);
    }
  });

  function collectAllFiles(directory: VideoDirectory): string[] {
    let files: string[] = [...(directory.files || [])];
    if (directory.subdirectories) {
      directory.subdirectories.forEach((subdir) => {
        files = files.concat(collectAllFiles(subdir));
      });
    }
    return files;
  }

  function toggleFolder(folderName: string) {
    if (expandedFolders.has(folderName)) {
      expandedFolders.delete(folderName);
    } else {
      expandedFolders.add(folderName);
    }
    expandedFolders = expandedFolders;
  }

  function playFile(filePath: string) {
    console.log(`Attempting to play: ${filePath}`);
    currentVideo = filePath;
  }
</script>

<div class="container">
  {#if currentVideo}
    <VideoPlayer src={currentVideo} on:close={() => (currentVideo = null)} />
  {/if}

  {#if allFiles.length > 0}
    <div class="card bg-base-100 shadow-md">
      <div class="card-body p-3">
        <button
          class="card-title flex gap-2 w-full text-left text-sm"
          on:click={() => toggleFolder("allVideos")}
        >
          {#if expandedFolders.has("allVideos")}
            <ChevronDown class="w-5 h-5" />
          {:else}
            <ChevronRight class="w-5 h-5" />
          {/if}
          <FileVideo class="w-5 h-5" />
          All Videos
          <span class="badge badge-primary badge-xs ml-2"
            >{allFiles.length}</span
          >
        </button>

        {#if expandedFolders.has("allVideos")}
          <div class="space-y-1 mt-2">
            {#each allFiles as file}
              <div
                class="flex items-center bg-base-200 p-2 rounded-md hover:bg-base-300 transition-colors text-sm"
              >
                <button
                  class="btn btn-circle btn-primary btn-xs mr-2"
                  on:click={() => playFile(file)}
                  title="Play video"
                >
                  <Play size={14} />
                </button>
                <span class="truncate">{extractFileName(file)}</span>
              </div>
            {/each}
          </div>
        {/if}
      </div>
    </div>
  {/if}

  {#each directoryStructure as directory}
    {#if directory.files.length > 0 || directory.subdirectories.length > 0}
      <div class="card bg-base-100 shadow-md">
        <div class="card-body p-3">
          <button
            class="card-title flex gap-2 w-full text-left text-sm"
            on:click={() => toggleFolder(directory.name)}
          >
            {#if expandedFolders.has(directory.name)}
              <ChevronDown class="w-5 h-5" />
            {:else}
              <ChevronRight class="w-5 h-5" />
            {/if}
            <Folder class="w-5 h-5" />
            {directory.name}
            <span class="badge badge-secondary badge-xs ml-2">
              {directory.files.length + directory.subdirectories.length}
            </span>
          </button>

          {#if expandedFolders.has(directory.name)}
            <div class="space-y-1 mt-2">
              {#each directory.files as file}
                <div
                  class="flex items-center bg-base-200 p-2 rounded-md hover:bg-base-300 transition-colors text-sm"
                >
                  <button
                    class="btn btn-circle btn-secondary btn-xs mr-2"
                    on:click={() => playFile(file)}
                    title="Play video"
                  >
                    <Play size={14} />
                  </button>
                  <span class="truncate">{extractFileName(file)}</span>
                </div>
              {/each}
            </div>
          {/if}
        </div>
      </div>
    {/if}
  {/each}
</div>
