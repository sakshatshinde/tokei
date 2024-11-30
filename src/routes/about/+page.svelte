<script lang="ts">
  import { onMount } from "svelte";
  import { webviewsToHide } from "../../lib/utils";

  onMount(async () => {
    await webviewsToHide("all"); // doesn't matter what you pass here it should hide all other webviews except main
  });
</script>

<h1 class="text-xl racking-wide subpixel-antialiased font-bold">About</h1>
