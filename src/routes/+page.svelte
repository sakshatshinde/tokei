<script lang="ts">
  import ContinueWatching from "../components/ContinueWatching.svelte";
  import VideoPlayer from "../components/VideoPlayer.svelte";

  import { onMount } from "svelte";
  import { webviewsToHide } from "$lib/utils";

  onMount(async () => {
    await webviewsToHide("all"); // doesn't matter what you pass here it should hide all other webviews except main
  });
</script>

<ContinueWatching />

<!-- <VideoPlayer src="D:/Downloads/bun33s.mp4" /> -->
