<script lang="ts">
  import { onMount } from "svelte";
  // import { webviewsToHide } from "$lib/utils";
  import { createChildWebview } from "$lib/utils";

  onMount(async () => {
    await createChildWebview("seadex", "https://releases.moe/");
    // await webviewsToHide("all"); // doesn't matter what you pass here it should hide all other webviews except main
  });
</script>

<!-- Iframe goes here -->
<!-- <iframe
  id="seadex-iframe"
  title="Seadex"
  src="https://releases.moe/"
  class="h-full w-full border-none"
  style="overflow: hidden"
></iframe> -->
