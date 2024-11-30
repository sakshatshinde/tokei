<script lang="ts">
  import ContinueWatching from "../components/ContinueWatching.svelte";
  // import VideoPlayer from "../components/VideoPlayer.svelte";
  import { onMount } from "svelte";
  import { fetchMediaDirectoryStructure, webviewsToHide } from "$lib/utils";

  onMount(async () => {
    try {
      // Hide all other webviews except the main one
      await webviewsToHide("all");

      // Fetch directory structure
      let directoryStructure = await fetchMediaDirectoryStructure();
      console.log("Fetched Directory Structure:", directoryStructure);
    } catch (error) {
      console.error("Error during onMount:", error);
    }
  });
</script>

<ContinueWatching />

<!-- <VideoPlayer src="D:/Downloads/bun33s.mp4" /> -->
